use crate::utils::{format_binary_like_expression, JsAnyBinaryLikeExpression};
use crate::{FormatElement, FormatNode, FormatResult, Formatter};
use rome_js_syntax::JsInstanceofExpression;

impl FormatNode for JsInstanceofExpression {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        format_binary_like_expression(
            JsAnyBinaryLikeExpression::JsInstanceofExpression(self.clone()),
            formatter,
        )
    }
}
