use rome_formatter::FormatLanguage;
use rome_parser::AnyParse;

pub mod check_reformat;
pub mod diff_report;
pub mod snapshot_builder;
pub mod test_prettier_snapshot;
pub mod utils;

pub trait TestFormatLanguage {
    type FormatLanguage: FormatLanguage + Clone + 'static;

    fn parse(&self, text: &str) -> AnyParse;

    fn format_language(&self) -> Self::FormatLanguage;
}
