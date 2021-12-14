use rome_core::create_app;
use rome_formatter::{format_file, FormatOptions};
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
pub fn run(spec_input_file: &str, _: &str) {
	let app = create_app();
	let file_path = &spec_input_file;
	let spec_input_file = Path::new(spec_input_file);

	assert!(
		spec_input_file.is_file(),
		"The input '{}' must exist and be a file.",
		spec_input_file.display()
	);

	let result = format_file(file_path, FormatOptions::default(), &app);
	let file_name = spec_input_file.file_name().unwrap().to_str().unwrap();
	let input = fs::read_to_string(file_path).unwrap();
	let snapshot = format!("# Input\n{}\n---\n# Output\n{}", input, result.code());

	insta::with_settings!({
		prepend_module_to_snapshot => false,
		snapshot_path => spec_input_file.parent().unwrap(),
	}, {
		insta::assert_snapshot!(file_name, snapshot, file_name);
	});
}
