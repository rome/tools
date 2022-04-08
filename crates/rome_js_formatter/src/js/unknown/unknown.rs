use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rome_js_syntax::JsUnknown;
use rome_rowan::AstNode;

impl ToFormatElement for JsUnknown {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        Ok(formatter.format_unknown(self.syntax()))
    }
}
