use crate::utils::format_conditional;
use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_parser::ast::JsConditionalExpression;
use rslint_parser::AstNode;

impl ToFormatElement for JsConditionalExpression {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        format_conditional(self.syntax(), formatter, false)
    }
}
