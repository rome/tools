use crate::{JsonFormatLanguage, JsonFormatOptions};
use rome_diagnostics::location::FileId;
use rome_formatter_test::TestFormatLanguage;
use rome_json_parser::parse_json;
use rome_parser::AnyParse;

/// Perform a second pass of formatting on a file, printing a diff if the
/// output doesn't match the input
pub struct JsonReformatLanguage {
    pub options: JsonFormatOptions,
}

impl JsonReformatLanguage {
    pub fn new(options: JsonFormatOptions) -> Self {
        JsonReformatLanguage { options }
    }
}

impl TestFormatLanguage for JsonReformatLanguage {
    type FormatLanguage = JsonFormatLanguage;

    fn parse(&self, text: &str) -> AnyParse {
        parse_json(text, FileId::zero()).into()
    }

    fn format_language(&self) -> Self::FormatLanguage {
        JsonFormatLanguage::new(self.options.clone())
    }
}
