use crate::utils::{format_binary_like_expression, JsAnyBinaryLikeExpression};
use crate::{FormatElement, FormatNode, FormatResult, Formatter};
use rome_js_syntax::JsInExpression;

impl FormatNode for JsInExpression {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        format_binary_like_expression(
            JsAnyBinaryLikeExpression::JsInExpression(self.clone()),
            formatter,
        )
    }
}
