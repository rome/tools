use crate::{JsonFormatLanguage, JsonFormatOptions};
use rome_diagnostics::location::FileId;
use rome_formatter::FormatLanguage;
use rome_formatter_test::TestFormatLanguage;
use rome_fs::RomePath;
use rome_json_parser::parse_json;
use rome_parser::AnyParse;

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

    fn read_format_languages_from_file(&self, _path: &mut RomePath) -> Vec<Self::FormatLanguage> {
        unimplemented!()
    }

    fn from_format_language(format_language: &Self::FormatLanguage) -> Self {
        JsonReformatLanguage::new(format_language.options().clone())
    }
}
