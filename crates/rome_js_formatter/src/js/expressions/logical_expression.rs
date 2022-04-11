use crate::utils::{format_binary_like_expression, JsAnyBinaryLikeExpression};
use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rome_js_syntax::JsLogicalExpression;

impl ToFormatElement for JsLogicalExpression {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        format_binary_like_expression(
            JsAnyBinaryLikeExpression::JsLogicalExpression(self.clone()),
            formatter,
        )
    }
}
