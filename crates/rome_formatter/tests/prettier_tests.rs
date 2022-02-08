use parking_lot::{const_mutex, Mutex};
use rome_rowan::{TextRange, TextSize};
use similar::{utils::diff_lines, Algorithm};
use std::{
    env,
    ffi::OsStr,
    fmt::Write,
    fs::{read_to_string, write},
    ops::Range,
    path::Path,
};

use rome_formatter::{FormatOptions, IndentStyle};
use rslint_errors::{file::SimpleFiles, termcolor, Emitter};
use rslint_parser::parse_module;

static REPORTER: DiffReport = DiffReport::new();

tests_macros::gen_tests! {"tests/specs/prettier/**/*.js", test_snapshot, "script"}

fn test_snapshot(input: &'static str, _: &str, _: &str, _: &str) {
    if input.contains("typescript")
        || input.contains("jsx")
        || input.contains("flow")
        || input.contains("prepare_tests")
    {
        return;
    }

    let input_file = Path::new(input);
    let mut input_code = read_to_string(input_file)
        .unwrap_or_else(|err| panic!("failed to read {:?}: {:?}", input_file, err));

    let (_, range_start_index, range_end_index) = strip_placeholders(&mut input_code);

    let parsed = parse_module(&input_code, 0);
    let syntax = parsed.syntax();

    let options = FormatOptions::new(IndentStyle::Space(2));

    let result = match (range_start_index, range_end_index) {
        (Some(start), Some(end)) => {
            // Skip the reversed range tests as its impossible
            // to create a reversed TextRange anyway
            if end < start {
                return;
            }

            rome_formatter::format_range(
                options,
                &syntax,
                TextRange::new(
                    TextSize::try_from(start).unwrap(),
                    TextSize::try_from(end).unwrap(),
                ),
            )
        }
        _ => rome_formatter::format(options, &syntax),
    };

    let formatted = result.expect("formatting failed");
    let formatted = match (range_start_index, range_end_index) {
        (Some(_), Some(_)) => {
            let range = formatted
                .range()
                .expect("the result of format_range should have a range");

            let formatted = formatted.as_code();
            let mut output_code = input_code.clone();
            output_code.replace_range(Range::<usize>::from(range), formatted);
            output_code
        }
        _ => formatted.into_code(),
    };

    let mut snapshot = String::new();

    writeln!(snapshot, "# Input").unwrap();
    writeln!(snapshot, "```js").unwrap();
    writeln!(snapshot, "{}", input_code).unwrap();
    writeln!(snapshot, "```").unwrap();
    writeln!(snapshot).unwrap();

    writeln!(snapshot, "# Output").unwrap();
    writeln!(snapshot, "```js").unwrap();
    writeln!(snapshot, "{}", formatted).unwrap();
    writeln!(snapshot, "```").unwrap();
    writeln!(snapshot).unwrap();

    let file_name = input_file.file_name().and_then(OsStr::to_str).unwrap();

    if !parsed.errors().is_empty() {
        let mut files = SimpleFiles::new();
        files.add(file_name.into(), input_code);

        let mut buffer = termcolor::Buffer::no_color();
        let mut emitter = Emitter::new(&files);

        for error in parsed.errors() {
            emitter
                .emit_with_writer(error, &mut buffer)
                .expect("failed to emit diagnostic");
        }

        writeln!(snapshot, "# Errors").unwrap();
        writeln!(snapshot, "```").unwrap();
        writeln!(
            snapshot,
            "{}",
            std::str::from_utf8(buffer.as_slice()).expect("non utf8 in error buffer")
        )
        .unwrap();
        writeln!(snapshot, "```").unwrap();
        writeln!(snapshot).unwrap();
    }

    insta::with_settings!({
        prepend_module_to_snapshot => false,
        snapshot_path => input_file.parent().unwrap(),
    }, {
        insta::assert_snapshot!(file_name, snapshot, file_name);
    });

    let snapshot_file = input_file
        .extension()
        .and_then(OsStr::to_str)
        .map(|ext| input_file.with_extension(format!("{}.prettier-snap", ext)))
        .filter(|path| path.exists());

    if let Some(snapshot_file) = snapshot_file {
        let mut content = read_to_string(snapshot_file).unwrap();

        strip_placeholders(&mut content);

        if formatted != content {
            let root_path = Path::new(concat!(
                env!("CARGO_MANIFEST_DIR"),
                "/tests/specs/prettier/"
            ));

            let input_file = input_file.strip_prefix(root_path).unwrap_or_else(|_| {
                panic!(
                    "failed to strip prefix {:?} from {:?}",
                    root_path, input_file
                )
            });

            let input_file = input_file.to_str().unwrap();
            REPORTER.report(input_file, formatted, content);
        }
    }
}

/// Find and replace the cursor, range start and range end placeholders in a
/// Prettier snapshot tests and return their indices in the resulting string
fn strip_placeholders(input_code: &mut String) -> (Option<usize>, Option<usize>, Option<usize>) {
    const CURSOR_PLACEHOLDER: &str = "<|>";
    const RANGE_START_PLACEHOLDER: &str = "<<<PRETTIER_RANGE_START>>>";
    const RANGE_END_PLACEHOLDER: &str = "<<<PRETTIER_RANGE_END>>>";

    let mut cursor_index = None;
    let mut range_start_index = None;
    let mut range_end_index = None;

    if let Some(index) = input_code.find(CURSOR_PLACEHOLDER) {
        input_code.replace_range(index..index + CURSOR_PLACEHOLDER.len(), "");
        cursor_index = Some(index);
    }

    if let Some(index) = input_code.find(RANGE_START_PLACEHOLDER) {
        input_code.replace_range(index..index + RANGE_START_PLACEHOLDER.len(), "");
        range_start_index = Some(index);

        if let Some(cursor) = &mut cursor_index {
            if *cursor > index {
                *cursor -= RANGE_START_PLACEHOLDER.len();
            }
        }
    }

    if let Some(index) = input_code.find(RANGE_END_PLACEHOLDER) {
        input_code.replace_range(index..index + RANGE_END_PLACEHOLDER.len(), "");
        range_end_index = Some(index);

        if let Some(cursor) = &mut cursor_index {
            if *cursor > index {
                *cursor -= RANGE_END_PLACEHOLDER.len();
            }
        }
        if let Some(cursor) = &mut range_start_index {
            // Prettier has tests for reversed ranges
            if *cursor > index {
                *cursor -= RANGE_END_PLACEHOLDER.len();
            }
        }
    }

    (cursor_index, range_start_index, range_end_index)
}

struct DiffReport {
    state: Mutex<Vec<(&'static str, String, String)>>,
}

impl DiffReport {
    const fn new() -> Self {
        Self {
            state: const_mutex(Vec::new()),
        }
    }

    fn report(&self, file_name: &'static str, rome: String, prettier: String) {
        self.state.lock().push((file_name, rome, prettier));
    }

    fn print(&self) {
        // Only create the report file if the REPORT_PRETTIER
        // environment variable is set to 1
        match env::var("REPORT_PRETTIER") {
            Ok(value) if value == "1" => {}
            _ => return,
        }

        let mut report = String::new();

        let mut state = self.state.lock();

        state.sort_by_key(|(name, ..)| *name);

        for (file_name, rome, prettier) in state.iter() {
            writeln!(report, "# {}", file_name).unwrap();
            writeln!(report, "```diff").unwrap();

            for (tag, line) in diff_lines(Algorithm::default(), prettier, rome) {
                let line = line.strip_suffix('\n').unwrap_or(line);
                writeln!(report, "{}{}", tag, line).unwrap();
            }

            writeln!(report, "```").unwrap();
        }

        write("report.md", report).unwrap();
    }
}

#[ctor::dtor]
fn print_report() {
    REPORTER.print();
}
