use rome_formatter::FormatLanguage;
use rome_fs::RomePath;
use rome_parser::AnyParse;

pub mod check_reformat;
pub mod diff_report;
pub mod snapshot_builder;
pub mod spec;
pub mod test_prettier_snapshot;
pub mod utils;

pub trait TestFormatLanguage {
    type FormatLanguage: FormatLanguage + Clone + 'static;

    fn parse(&self, text: &str) -> AnyParse;

    fn format_language(&self) -> Self::FormatLanguage;

    fn read_format_languages_from_file(&self, path: &mut RomePath) -> Vec<Self::FormatLanguage>;

    fn from_format_language(format_language: &Self::FormatLanguage) -> Self;
}
