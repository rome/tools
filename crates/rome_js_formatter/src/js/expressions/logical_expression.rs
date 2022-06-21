use crate::prelude::*;
use crate::utils::{format_binary_like_expression, JsAnyBinaryLikeExpression};

use rome_js_syntax::JsLogicalExpression;

#[derive(Debug, Clone, Default)]
pub struct FormatJsLogicalExpression;

impl FormatNodeRule<JsLogicalExpression> for FormatJsLogicalExpression {
    fn fmt_fields(
        &self,
        node: &JsLogicalExpression,
        formatter: &mut JsFormatter,
    ) -> FormatResult<()> {
        format_binary_like_expression(
            JsAnyBinaryLikeExpression::JsLogicalExpression(node.clone()),
            formatter,
        )
    }
}
