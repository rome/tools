use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};

use rslint_syntax::JsUnknownParameter;

use rslint_syntax::AstNode;

impl ToFormatElement for JsUnknownParameter {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        Ok(formatter.format_unknown(self.syntax()))
    }
}
