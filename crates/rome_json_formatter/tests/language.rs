use rome_diagnostics::location::FileId;
use rome_formatter_test::TestFormatLanguage;
use rome_json_formatter::context::JsonFormatOptions;
use rome_json_formatter::JsonFormatLanguage;
use rome_json_parser::parse_json;
use rome_parser::AnyParse;

pub struct JsonTestFormatLanguage {
    pub options: JsonFormatOptions,
}

impl JsonTestFormatLanguage {
    pub fn new(options: JsonFormatOptions) -> Self {
        JsonTestFormatLanguage { options }
    }
}

impl TestFormatLanguage for JsonTestFormatLanguage {
    type FormatLanguage = JsonFormatLanguage;

    fn parse(&self, text: &str) -> AnyParse {
        parse_json(text, FileId::zero()).into()
    }

    fn format_language(&self) -> Self::FormatLanguage {
        JsonFormatLanguage::new(self.options.clone())
    }
}
