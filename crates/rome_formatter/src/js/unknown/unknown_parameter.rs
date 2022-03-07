use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};

use rome_js_syntax::JsUnknownParameter;

use rome_js_syntax::AstNode;

impl ToFormatElement for JsUnknownParameter {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        Ok(formatter.format_unknown(self.syntax()))
    }
}
