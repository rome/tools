use rome_core::App;
use rome_formatter::{format, FormatOptions, IndentStyle};
use rome_path::RomePath;
use rslint_parser::{parse, Syntax};
use serde::{Deserialize, Serialize};
use std::fmt::Debug;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Eq, PartialEq, Clone, Copy, Deserialize, Serialize)]
pub enum IndentStyleTest {
    /// Tab
    Tab,
    /// Space, with its quantity
    Space(u8),
}

impl From<IndentStyleTest> for IndentStyle {
    fn from(test: IndentStyleTest) -> Self {
        match test {
            IndentStyleTest::Tab => IndentStyle::Tab,
            IndentStyleTest::Space(s) => IndentStyle::Space(s),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct FormatOptionsTest {
    /// The indent style
    pub indent_style: Option<IndentStyleTest>,

    /// What's the max width of a line. Defaults to 80
    pub line_width: Option<u16>,
}

impl From<FormatOptionsTest> for FormatOptions {
    fn from(test: FormatOptionsTest) -> Self {
        Self {
            indent_style: test
                .indent_style
                .map_or_else(|| IndentStyle::Tab, |value| value.into()),
            line_width: test.line_width.unwrap_or(80),
        }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Options {
    cases: Vec<FormatOptionsTest>,
}

#[derive(Debug, Default)]
struct SnapshotContent {
    input: String,
    output: Vec<(String, FormatOptions)>,
}

impl SnapshotContent {
    pub fn add_output(&mut self, content: &str, options: FormatOptions) {
        self.output.push((String::from(content), options))
    }

    pub fn set_input(&mut self, content: &str) {
        self.input = String::from(content);
    }

    pub fn snap_content(&mut self) -> String {
        let mut output = String::new();
        output.push_str("# Input");
        output.push('\n');
        output.push_str(self.input.as_str());
        output.push_str("\n=============================\n");

        output.push_str("# Outputs\n");
        let iter = self.output.iter();
        for (index, (content, options)) in iter.enumerate() {
            let formal_index = index + 1;
            output.push_str(format!("## Output {formal_index}\n").as_str());
            output.push_str("-----\n");
            output.push_str(format!("{}\n", options).as_str());
            output.push_str("-----\n");
            output.push_str(content.as_str());
        }

        output
    }
}

/// [insta.rs](https://insta.rs/docs) snapshot testing
///
/// For better development workflow, run
/// `cargo watch -i '*.new' -x 'test -p rome_formatter formatter'`
///
/// To review and commit the snapshots, `cargo install cargo-insta`, and run
/// `cargo insta review` or `cargo insta accept`
///
/// The input and the expected output are stored as dedicated files in the `tests/specs` directory where
/// the input file name is `{spec_name}.json` and the output file name is `{spec_name}.json.snap`.
///
/// Specs can be grouped in directories by specifying the directory name in the spec name. Examples:
///
/// # Examples
///
/// * `json/null` -> input: `tests/specs/json/null.json`, expected output: `tests/specs/json/null.json.snap`
/// * `null` -> input: `tests/specs/null.json`, expected output: `tests/specs/null.json.snap`
pub fn run(spec_input_file: &str, _expected_file: &str, test_directory: &str, file_type: &str) {
    let app = App::new();
    let file_path = &spec_input_file;
    let spec_input_file = Path::new(spec_input_file);

    assert!(
        spec_input_file.is_file(),
        "The input '{}' must exist and be a file.",
        spec_input_file.display()
    );

    let mut rome_path = RomePath::new(file_path);
    if app.can_format(&rome_path) {
        let mut snapshot_content = SnapshotContent::default();
        let buffer = rome_path.get_buffer_from_file();
        let syntax = if file_type == "module" {
            Syntax::default().module()
        } else {
            Syntax::default()
        };

        let input = fs::read_to_string(file_path).unwrap();
        snapshot_content.set_input(input.as_str());

        let root = parse(buffer.as_str(), 0, syntax).syntax();
        let formatted_result = format(FormatOptions::default(), &root);
        let file_name = spec_input_file.file_name().unwrap().to_str().unwrap();
        // we ignore the error for now
        let result = formatted_result.unwrap();

        snapshot_content.add_output(result.as_code(), FormatOptions::default());

        let test_directory = PathBuf::from(test_directory);
        let options_path = test_directory.join("options.json");
        if options_path.exists() {
            let mut options_path = RomePath::new(options_path.display().to_string().as_str());
            // SAFETY: we checked its existence already, we assume we have rights to read it
            let options: Options =
                serde_json::from_str(options_path.get_buffer_from_file().as_str()).unwrap();

            for test_case in options.cases {
                let options = test_case.clone();
                let formatted_result = format(test_case.into(), &root).unwrap();
                snapshot_content.add_output(formatted_result.as_code(), options.into());
            }
        }

        insta::with_settings!({
            prepend_module_to_snapshot => false,
            snapshot_path => spec_input_file.parent().unwrap(),
        }, {
            insta::assert_snapshot!(file_name, snapshot_content.snap_content(), file_name);
        });
    }
}
