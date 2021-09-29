use rome_formatter::{format_str, FormatOptions};
use std::fs;
use std::path::PathBuf;

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
pub fn spec_test(spec_name: &str) {
	let spec_input_file = spec_file_name(spec_name, "json");

	assert!(
		spec_input_file.is_file(),
		"The input '{}' for the spec '{}' must exist and be a file.",
		spec_name,
		spec_input_file.display()
	);

	let expected_file = spec_file_name(spec_name, "expected.json");
	assert!(
		expected_file.is_file(),
		"The expected output '{}' for the spec '{}' must exist and be a file.",
		expected_file.display(),
		spec_name
	);

	let spec_content = fs::read_to_string(spec_input_file).unwrap();

	let result = format_str(spec_content.as_str(), FormatOptions::default());

	let expected_output = fs::read_to_string(expected_file).unwrap();
	assert_eq!(expected_output, result.root().text().to_string());
}

fn spec_file_name(spec_name: &str, extension: &str) -> PathBuf {
	let specs_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
		.join("tests")
		.join("specs");

	assert!(
		specs_dir.is_dir(),
		"The directory '{}' containing the specification files doesn't exist or isn't a directory.",
		specs_dir.display()
	);

	let base_spec_path =
		PathBuf::from(spec_name.replace('/', std::path::MAIN_SEPARATOR.to_string().as_str()));

	specs_dir.join(base_spec_path.with_extension(extension))
}
