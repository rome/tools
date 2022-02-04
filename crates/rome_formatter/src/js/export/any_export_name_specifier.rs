use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_parser::ast::JsAnyExportNamedSpecifier;

impl ToFormatElement for JsAnyExportNamedSpecifier {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            JsAnyExportNamedSpecifier::JsExportNamedShorthandSpecifier(node) => {
                node.to_format_element(formatter)
            }
            JsAnyExportNamedSpecifier::JsExportNamedSpecifier(node) => {
                node.to_format_element(formatter)
            }
        }
    }
}
