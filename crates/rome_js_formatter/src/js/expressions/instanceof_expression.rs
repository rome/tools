use crate::utils::{format_binary_like_expression, JsAnyBinaryLikeExpression};
use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rome_js_syntax::JsInstanceofExpression;

impl ToFormatElement for JsInstanceofExpression {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        format_binary_like_expression(
            JsAnyBinaryLikeExpression::JsInstanceofExpression(self.clone()),
            formatter,
        )
    }
}
