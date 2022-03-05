use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_syntax::{AstNode, JsUnknown};
impl ToFormatElement for JsUnknown {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        Ok(formatter.format_unknown(self.syntax()))
    }
}
