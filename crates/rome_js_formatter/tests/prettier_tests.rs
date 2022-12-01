use rome_diagnostics::location::FileId;
use std::{env, path::Path};

use rome_formatter::IndentStyle;
use rome_formatter_test::check_reformat::CheckReformat;
use rome_formatter_test::test_prettier_snapshot::{PrettierTestFile, PrettierTestSnapshot};
use rome_js_formatter::context::JsFormatOptions;
use rome_js_formatter::JsFormatLanguage;
use rome_js_parser::parse;
use rome_js_syntax::{JsSyntaxNode, SourceType};
use rome_parser::AnyParse;

mod check_reformat;

tests_macros::gen_tests! {"tests/specs/prettier/{js,typescript,jsx}/**/*.{js,ts,jsx,tsx}", crate::test_snapshot, "script"}

fn test_snapshot(input: &'static str, _: &str, _: &str, _: &str) {
    countme::enable(true);

    let root_path = Path::new(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/specs/prettier/"
    ));

    JsPrettierTestSnapshot::new(input, root_path).test()
}

pub struct JsPrettierTestSnapshot<'a> {
    test_file: PrettierTestFile<'a>,
    parsed: AnyParse,
    options: JsFormatOptions,
}

impl PrettierTestSnapshot<JsFormatLanguage> for JsPrettierTestSnapshot<'_> {
    fn test_file(&self) -> &PrettierTestFile {
        &self.test_file
    }

    fn format_language(&self) -> JsFormatLanguage {
        JsFormatLanguage::new(self.options.clone())
    }

    fn parsed(&self) -> &AnyParse {
        &self.parsed
    }

    fn check_reformat(&self, root: &JsSyntaxNode, formatted: &str) {
        let file_name = self.test_file.file_name();

        let js_check_reformat = check_reformat::JsCheckReformat {
            root,
            text: formatted,
            file_name,
            options: self.options.clone(),
        };
        js_check_reformat.check_reformat();
    }
}

impl<'a> JsPrettierTestSnapshot<'a> {
    pub fn new(input: &'static str, root_path: &'a Path) -> Self {
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

        let parsed = parse(test_file.parse_input(), FileId::zero(), source_type).into();

        let options = JsFormatOptions::new(source_type).with_indent_style(IndentStyle::Space(2));

        JsPrettierTestSnapshot {
            test_file,
            parsed,
            options,
        }
    }
}
