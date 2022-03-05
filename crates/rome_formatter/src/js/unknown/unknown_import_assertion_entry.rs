use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};

use rslint_syntax::JsUnknownImportAssertionEntry;

use rslint_syntax::AstNode;

impl ToFormatElement for JsUnknownImportAssertionEntry {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        Ok(formatter.format_unknown(self.syntax()))
    }
}
