use crate::JsFormatLanguage;
use crate::JsFormatOptions;
use rome_diagnostics::location::FileId;
use rome_formatter_test::TestFormatLanguage;
use rome_js_parser::parse;
use rome_parser::AnyParse;

/// Perform a second pass of formatting on a file, printing a diff if the
/// output doesn't match the input
pub struct JsReformatLanguage {
    pub options: JsFormatOptions,
}

impl JsReformatLanguage {
    pub fn new(options: JsFormatOptions) -> Self {
        JsReformatLanguage { options }
    }
}

impl TestFormatLanguage for JsReformatLanguage {
    type FormatLanguage = JsFormatLanguage;

    fn parse(&self, text: &str) -> AnyParse {
        parse(text, FileId::zero(), self.options.source_type()).into()
    }

    fn format_language(&self) -> Self::FormatLanguage {
        JsFormatLanguage::new(self.options.clone())
    }
}
