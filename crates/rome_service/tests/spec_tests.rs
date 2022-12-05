use rome_diagnostics::{print_diagnostic_to_string, DiagnosticExt, FileId};
use rome_json_parser::parse_json;
use rome_service::Configuration;
use std::ffi::OsStr;
use std::fs::read_to_string;
use std::path::Path;
tests_macros::gen_tests! {"tests/invalid/*.{json}", crate::run_invalid_configurations, "module"}

fn run_invalid_configurations(input: &'static str, _: &str, _: &str, _: &str) {
    dbg!(&input);
    let input_file = Path::new(input);
    let file_name = input_file.file_name().and_then(OsStr::to_str).unwrap();
    let extension = input_file.extension().and_then(OsStr::to_str).unwrap();
    let input_code = read_to_string(input_file)
        .unwrap_or_else(|err| panic!("failed to read {:?}: {:?}", input_file, err));

    let result = match extension {
        "json" => {
            let result = parse_json(input_code.as_str(), FileId::zero());
            Configuration::from_json_ast(result.tree())
        }
        _ => {
            panic!("Extension not supported");
        }
    };

    assert!(
        result.is_err(),
        "This test should have diagnostics, but it doesn't have any"
    );
    if let Err(diagnostics) = result {
        let result = print_diagnostic_to_string(
            diagnostics
                .with_file_path(file_name)
                .with_file_source_code(input_code.as_str()),
        );
        insta::with_settings!({
            prepend_module_to_snapshot => false,
            snapshot_path => input_file.parent().unwrap(),
        }, {
            insta::assert_snapshot!(file_name, result, file_name);
        });
    }
}
