use rome_diagnostics::location::FileId;
use rome_formatter_test::check_reformat::{CheckReformat, CheckReformatParams};
use rome_js_formatter::context::JsFormatOptions;
use rome_js_formatter::JsFormatLanguage;
use rome_js_parser::parse;
use rome_js_syntax::JsSyntaxNode;
use rome_parser::AnyParse;

pub struct JsCheckReformat<'a> {
    pub root: &'a JsSyntaxNode,
    pub text: &'a str,
    pub file_name: &'a str,
    pub options: JsFormatOptions,
}

/// Perform a second pass of formatting on a file, printing a diff if the
/// output doesn't match the input
impl CheckReformat<JsFormatLanguage> for JsCheckReformat<'_> {
    fn parse(&self, text: &str) -> AnyParse {
        parse(text, FileId::zero(), self.options.source_type()).into()
    }

    fn params(&self) -> CheckReformatParams<JsFormatLanguage> {
        let JsCheckReformat {
            root,
            text,
            file_name,
            options,
        } = self;

        let format_language = JsFormatLanguage::new(options.clone());

        CheckReformatParams {
            root,
            format_language,
            text,
            file_name,
        }
    }
}
