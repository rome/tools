use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rome_js_syntax::{AstNode, JsxElement};
impl ToFormatElement for JsxElement {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        Ok(formatter.format_verbatim(self.syntax()))
    }
}
