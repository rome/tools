use crate::{JsonFormatLanguage, JsonFormatOptions};
use rome_diagnostics::location::FileId;
use rome_formatter_test::check_reformat::{CheckReformat, CheckReformatParams};
use rome_json_parser::parse_json;
use rome_json_syntax::JsonSyntaxNode;
use rome_parser::AnyParse;

pub struct JsonCheckReformat<'a> {
    pub root: &'a JsonSyntaxNode,
    pub text: &'a str,
    pub file_name: &'a str,
    pub options: JsonFormatOptions,
}

/// Perform a second pass of formatting on a file, printing a diff if the
/// output doesn't match the input
impl CheckReformat<JsonFormatLanguage> for JsonCheckReformat<'_> {
    fn parse(&self, text: &str) -> AnyParse {
        parse_json(text, FileId::zero()).into()
    }

    fn params(&self) -> CheckReformatParams<JsonFormatLanguage> {
        let JsonCheckReformat {
            root,
            text,
            file_name,
            options,
            ..
        } = self;

        let format_language = JsonFormatLanguage::new(options.clone());

        CheckReformatParams {
            root,
            format_language,
            text,
            file_name,
        }
    }
}
