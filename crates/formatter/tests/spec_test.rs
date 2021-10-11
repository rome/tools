use core::create_app;
use rome_formatter::{format_file, FormatOptions};
use std::fs;
use std::path::Path;

/// Tests that format a given input file, results in the expected formatted output.
///
/// The input and the expected output are stored as dedicated files in the `tests/specs` directory where
/// the input file name is `{spec_name}.json` and the output file name is `{spec_name.expected}.json`.
///
/// Specs can be grouped in directories by specifying the directory name in the spec name. Examples:
///
/// # Examples
///
/// * `json/null` -> input: `tests/specs/json/null.json`, expected output: `tests/specs/json/null.expected.json`
/// * `null` -> input: `tests/specs/null.json`, expected output: `tests/specs/null.expected.json`
pub fn run(spec_input_file: &str, expected_file: &str) {
	let app = create_app();
	let file_path = &spec_input_file;
	let spec_input_file = Path::new(spec_input_file);
	let expected_file = Path::new(expected_file);

	assert!(
		spec_input_file.is_file(),
		"The input '{}' must exist and be a file.",
		spec_input_file.display()
	);

	assert!(
		expected_file.is_file(),
		"The expected output '{}' must exist and be a file.",
		expected_file.display(),
	);

	let result = format_file(file_path, FormatOptions::default(), &app);
	let expected_output = fs::read_to_string(expected_file).unwrap();

	assert_eq!(&expected_output, result.code());
}
