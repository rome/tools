use crate::prelude::*;
use crate::utils::{format_binary_like_expression, JsAnyBinaryLikeExpression};

use crate::FormatNodeFields;
use rome_js_syntax::JsInExpression;

impl FormatNodeFields<JsInExpression> for FormatNodeRule<JsInExpression> {
    fn format_fields(
        node: &JsInExpression,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        format_binary_like_expression(
            JsAnyBinaryLikeExpression::JsInExpression(node.clone()),
            formatter,
        )
    }
}
