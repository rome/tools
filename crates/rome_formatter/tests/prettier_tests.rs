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

use rome_diagnostics::{file::SimpleFiles, termcolor, Emitter};
use rome_formatter::{FormatOptions, IndentStyle};
use rome_js_parser::{parse, SourceType};

use crate::check_reformat::CheckReformatParams;

mod check_reformat;

static REPORTER: DiffReport = DiffReport::new();

tests_macros::gen_tests! {"tests/specs/prettier/**/*.{js,ts,jsx,tsx}", crate::test_snapshot, "script"}

const PRETTIER_IGNORE: &str = "prettier-ignore";
const ROME_IGNORE: &str = "rome-ignore format: prettier ignore";

fn test_snapshot(input: &'static str, _: &str, _: &str, _: &str) {
    if input.contains("flow") || input.contains("prepare_tests") {
        return;
    }

    let input_file = Path::new(input);
    let file_name = input_file.file_name().and_then(OsStr::to_str).unwrap();
    let mut input_code = read_to_string(input_file)
        .unwrap_or_else(|err| panic!("failed to read {:?}: {:?}", input_file, err));

    let (_, range_start_index, range_end_index) = strip_placeholders(&mut input_code);
    let parse_input = input_code.replace(PRETTIER_IGNORE, ROME_IGNORE);

    // Prettier testing suite uses JSX tags inside JS files.
    // As there's no way to know in advance which files have JSX syntax, we
    // change the source type only here
    let source_type = if input_file.extension().unwrap() == "js" {
        SourceType::jsx()
    } else if file_name.contains("jsx") && input_file.extension() == Some(OsStr::new("ts")) {
        SourceType::tsx()
    } else {
        input_file.try_into().unwrap()
    };

    let parsed = parse(&parse_input, 0, source_type.clone());

    let has_errors = parsed.has_errors();
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
            let mut output_code = parse_input.clone();
            output_code.replace_range(Range::<usize>::from(range), formatted);
            output_code
        }
        _ => {
            let result = formatted.into_code();

            if !has_errors {
                check_reformat::check_reformat(CheckReformatParams {
                    root: &syntax,
                    text: &result,
                    source_type,
                    file_name,
                    format_options: options,
                });
            }

            result
        }
    };

    let formatted = formatted.replace(ROME_IGNORE, PRETTIER_IGNORE);

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

    if has_errors {
        let mut files = SimpleFiles::new();
        files.add(file_name.into(), parse_input);

        let mut buffer = termcolor::Buffer::no_color();
        let mut emitter = Emitter::new(&files);

        for error in parsed.diagnostics() {
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
