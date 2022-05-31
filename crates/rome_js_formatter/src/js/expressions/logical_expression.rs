use crate::prelude::*;
use crate::utils::{format_binary_like_expression, JsAnyBinaryLikeExpression};

use crate::FormatNodeFields;
use rome_js_syntax::JsLogicalExpression;

impl FormatNodeFields<JsLogicalExpression> for FormatNodeRule<JsLogicalExpression> {
    fn format_fields(
        node: &JsLogicalExpression,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        format_binary_like_expression(
            JsAnyBinaryLikeExpression::JsLogicalExpression(node.clone()),
            formatter,
        )
    }
}
