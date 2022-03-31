use crate::utils::format_binaryish_expression;
use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rome_js_syntax::JsInExpression;
use rome_rowan::AstNode;

impl ToFormatElement for JsInExpression {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        format_binaryish_expression(self.syntax(), formatter)
    }
}
