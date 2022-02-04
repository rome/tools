use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_parser::ast::JsUnknownImportAssertionEntry;
use rslint_parser::AstNode;

impl ToFormatElement for JsUnknownImportAssertionEntry {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        Ok(formatter.format_verbatim(self.syntax()))
    }
}
