mod spec_test;

mod formatter {
	mod json {
		tests_macros::gen_tests! {"tests/specs/json/*.json", super::assert_formatter}
	}

	fn assert_formatter(spec_input_file: &str, expected_file: &str) {
		println!("{}", spec_input_file);
		use rome_formatter::{format_str, FormatOptions};
		let spec_content = std::fs::read_to_string(spec_input_file).unwrap();
		let result = format_str(spec_content.as_str(), FormatOptions::default());
		let expected_output = std::fs::read_to_string(expected_file).unwrap();
		assert_eq!(&expected_output, result.code());
	}
}
