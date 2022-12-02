use rome_diagnostics::location::FileId;
use rome_formatter_test::TestFormatLanguage;
use rome_js_formatter::context::JsFormatOptions;
use rome_js_formatter::JsFormatLanguage;
use rome_js_parser::parse;
use rome_parser::AnyParse;

pub struct JsTestFormatLanguage {
    options: JsFormatOptions,
}

impl JsTestFormatLanguage {
    pub fn new(options: JsFormatOptions) -> Self {
        JsTestFormatLanguage { options }
    }
}

impl TestFormatLanguage for JsTestFormatLanguage {
    type FormatLanguage = JsFormatLanguage;

    fn parse(&self, text: &str) -> AnyParse {
        parse(text, FileId::zero(), self.options.source_type()).into()
    }

    fn format_language(&self) -> Self::FormatLanguage {
        JsFormatLanguage::new(self.options.clone())
    }
}
