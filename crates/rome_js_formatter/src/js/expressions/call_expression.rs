use crate::utils::format_call_expression;
use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rome_js_syntax::JsCallExpression;
use rome_rowan::AstNode;

impl ToFormatElement for JsCallExpression {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        format_call_expression(self.syntax(), formatter)
    }
}
