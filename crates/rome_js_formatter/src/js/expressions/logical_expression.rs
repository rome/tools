use crate::prelude::*;
use crate::utils::{format_binary_like_expression, JsAnyBinaryLikeExpression};

use crate::FormatNodeFields;
use rome_js_syntax::JsLogicalExpression;

impl FormatNodeFields<JsLogicalExpression> for FormatNodeRule<JsLogicalExpression> {
    fn fmt_fields(node: &JsLogicalExpression, formatter: &mut JsFormatter) -> FormatResult<()> {
        format_binary_like_expression(
            JsAnyBinaryLikeExpression::JsLogicalExpression(node.clone()),
            formatter,
        )
    }
}
