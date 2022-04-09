use crate::utils::{format_binary_like_expression, JsAnyBinaryLikeExpression};
use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rome_js_syntax::JsInExpression;

impl ToFormatElement for JsInExpression {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        format_binary_like_expression(
            JsAnyBinaryLikeExpression::JsInExpression(self.clone()),
            formatter,
        )
    }
}
