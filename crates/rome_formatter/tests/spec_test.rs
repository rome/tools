use rome_core::App;
use rome_formatter::{format, FormatOptions};
use rome_path::RomePath;
use rslint_parser::{parse, Syntax};
use std::fs;
use std::path::Path;

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
pub fn run(spec_input_file: &str, _: &str, file_type: &str) {
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
        let buffer = rome_path.get_buffer_from_file();
        let syntax = if file_type == "module" {
            Syntax::default().module()
        } else {
            Syntax::default()
        };

        let root = parse(buffer.as_str(), 0, syntax).syntax();
        let formatted_result = format(FormatOptions::default(), &root);
        let file_name = spec_input_file.file_name().unwrap().to_str().unwrap();
        let input = fs::read_to_string(file_path).unwrap();
        let result = formatted_result.unwrap();
        // we ignore the error for now
        let snapshot = format!("# Input\n{}\n---\n# Output\n{}", input, result.code());

        insta::with_settings!({
            prepend_module_to_snapshot => false,
            snapshot_path => spec_input_file.parent().unwrap(),
        }, {
            insta::assert_snapshot!(file_name, snapshot, file_name);
        });
    }
}
