use rome_diagnostics::location::FileId;
use std::{env, path::Path};

use rome_formatter::IndentStyle;
use rome_formatter_test::check_reformat::CheckReformat;
use rome_formatter_test::test_prettier_snapshot::{PrettierTestFile, PrettierTestSnapshot};
use rome_json_formatter::context::JsonFormatOptions;
use rome_json_formatter::JsonFormatLanguage;
use rome_parser::AnyParse;

use rome_json_parser::parse_json;
use rome_json_syntax::JsonSyntaxNode;

#[derive(serde::Serialize)]
struct TestInfo {
    test_file: String,
}

mod check_reformat;

tests_macros::gen_tests! {"tests/specs/prettier/{json}/**/*.{json}", crate::test_snapshot, ""}

fn test_snapshot(input: &'static str, _: &str, _: &str, _: &str) {
    countme::enable(true);

    let root_path = Path::new(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/specs/prettier/"
    ));

    JsonPrettierTestSnapshot::new(input, root_path).test()
}

pub struct JsonPrettierTestSnapshot<'a> {
    test_file: PrettierTestFile<'a>,
    parsed: AnyParse,
    options: JsonFormatOptions,
}

impl PrettierTestSnapshot<JsonFormatLanguage> for JsonPrettierTestSnapshot<'_> {
    fn test_file(&self) -> &PrettierTestFile {
        &self.test_file
    }

    fn format_language(&self) -> JsonFormatLanguage {
        JsonFormatLanguage::new(self.options.clone())
    }

    fn parsed(&self) -> &AnyParse {
        &self.parsed
    }

    fn check_reformat(&self, root: &JsonSyntaxNode, formatted: &str) {
        let file_name = self.test_file.file_name();

        let json_check_reformat = check_reformat::JsonCheckReformat {
            root,
            text: formatted,
            file_name,
            options: self.options.clone(),
        };
        json_check_reformat.check_reformat();
    }
}

impl<'a> JsonPrettierTestSnapshot<'a> {
    pub fn new(input: &'static str, root_path: &'a Path) -> Self {
        let test_file = PrettierTestFile::new(input, root_path);

        let parsed = parse_json(test_file.parse_input(), FileId::zero()).into();

        let options = JsonFormatOptions::default().with_indent_style(IndentStyle::Space(2));

        JsonPrettierTestSnapshot {
            test_file,
            parsed,
            options,
        }
    }
}
