use std::{env, path::Path};

use rome_formatter::IndentStyle;
use rome_formatter_test::test_prettier_snapshot::{PrettierSnapshot, PrettierTestFile};
use rome_js_formatter::context::JsFormatOptions;
use rome_js_syntax::SourceType;

mod language;

tests_macros::gen_tests! {"tests/specs/prettier/{js,typescript,jsx}/**/*.{js,ts,jsx,tsx}", crate::test_snapshot, "script"}

fn test_snapshot(input: &'static str, _: &str, _: &str, _: &str) {
    countme::enable(true);

    let root_path = Path::new(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/specs/prettier/"
    ));

    let test_file = PrettierTestFile::new(input, root_path);
    let source_type = {
        // Prettier testing suite uses JSX tags inside JS files.
        // As there's no way to know in advance which files have JSX syntax, we
        // change the source type only here
        if test_file.file_extension() == "js" {
            SourceType::jsx()
        } else if test_file.file_name().contains("jsx") && test_file.file_extension() == "ts" {
            SourceType::tsx()
        } else {
            test_file.input_file().try_into().unwrap()
        }
    };

    let options = JsFormatOptions::new(source_type).with_indent_style(IndentStyle::Space(2));
    let language = language::JsTestFormatLanguage::new(options);

    let snapshot = PrettierSnapshot::new(test_file, language);

    snapshot.test()
}
