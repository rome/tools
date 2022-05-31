use crate::prelude::*;
use crate::utils::{format_binary_like_expression, JsAnyBinaryLikeExpression};

use crate::FormatNodeFields;
use rome_js_syntax::JsBinaryExpression;

impl FormatNodeFields<JsBinaryExpression> for FormatNodeRule<JsBinaryExpression> {
    fn format_fields(
        node: &JsBinaryExpression,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        format_binary_like_expression(
            JsAnyBinaryLikeExpression::JsBinaryExpression(node.clone()),
            formatter,
        )
    }
}
