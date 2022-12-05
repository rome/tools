use crate::JsFormatLanguage;
use crate::JsFormatOptions;
use rome_diagnostics::location::FileId;
use rome_formatter::FormatLanguage;
use rome_formatter_test::TestFormatLanguage;
use rome_fs::RomePath;
use rome_js_parser::parse;
use rome_parser::AnyParse;

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

    fn read_format_languages_from_file(&self, _path: &mut RomePath) -> Vec<Self::FormatLanguage> {
        unimplemented!()
    }

    fn from_format_language(format_language: &Self::FormatLanguage) -> Self {
        JsReformatLanguage::new(format_language.options().clone())
    }
}
