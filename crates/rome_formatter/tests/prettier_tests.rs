use parking_lot::{const_mutex, Mutex};
use similar::{utils::diff_lines, Algorithm};
use std::{
    env,
    ffi::OsStr,
    fmt::Write,
    fs::{read_to_string, write},
    path::Path,
};

use rome_formatter::{FormatOptions, IndentStyle};
use rslint_parser::parse_module;

static REPORTER: DiffReport = DiffReport::new();

tests_macros::gen_tests! {"tests/specs/prettier/**/*.js", test_snapshot, "script"}

fn test_snapshot(input: &'static str, _: &str, _: &str) {
    if input.contains("typescript")
        || input.contains("jsx")
        || input.contains("flow")
        || input.contains("prepare_tests")
    {
        return;
    }

    let input_file = Path::new(input);
    let input_code = read_to_string(input_file)
        .unwrap_or_else(|err| panic!("failed to read {:?}: {:?}", input_file, err));

    let parsed = parse_module(&input_code, 0);
    let syntax = parsed.syntax();

    let options = FormatOptions::new(IndentStyle::Space(2));
    let formatted = rome_formatter::format(options, &syntax).unwrap();

    let mut snapshot = String::new();

    writeln!(snapshot, "# Input").unwrap();
    writeln!(snapshot, "```js").unwrap();
    writeln!(snapshot, "{}", input_code).unwrap();
    writeln!(snapshot, "```").unwrap();
    writeln!(snapshot).unwrap();

    writeln!(snapshot, "# Output").unwrap();
    writeln!(snapshot, "```js").unwrap();
    writeln!(snapshot, "{}", formatted.as_code()).unwrap();
    writeln!(snapshot, "```").unwrap();
    writeln!(snapshot).unwrap();

    let file_name = input_file.file_name().and_then(OsStr::to_str).unwrap();

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
        let content = read_to_string(snapshot_file).unwrap();
        if formatted.as_code() != content {
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
            REPORTER.report(input_file, formatted.into_code(), content);
        }
    }
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
