use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_parser::ast::JsAnyImportAssertionEntry;

impl ToFormatElement for JsAnyImportAssertionEntry {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        match self {
            JsAnyImportAssertionEntry::JsImportAssertionEntry(assertion_entry) => {
                assertion_entry.to_format_element(formatter)
            }
            JsAnyImportAssertionEntry::JsUnknownImportAssertionEntry(unknown_assertion_entry) => {
                unknown_assertion_entry.to_format_element(formatter)
            }
        }
    }
}
