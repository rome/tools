use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_parser::ast::JsAnyNamedImportSpecifier;

impl ToFormatElement for JsAnyNamedImportSpecifier {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            JsAnyNamedImportSpecifier::JsNamedImportSpecifier(e) => e.to_format_element(formatter),
            JsAnyNamedImportSpecifier::JsShorthandNamedImportSpecifier(e) => {
                e.to_format_element(formatter)
            }
            JsAnyNamedImportSpecifier::JsUnknownNamedImportSpecifier(e) => {
                e.to_format_element(formatter)
            }
        }
    }
}
