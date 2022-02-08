use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_parser::{ast::JsExpressionSnipped, AstNode};
impl ToFormatElement for JsExpressionSnipped {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        Ok(formatter.format_verbatim(self.syntax()))
    }
}