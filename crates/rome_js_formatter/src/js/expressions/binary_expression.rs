use crate::prelude::*;
use crate::utils::{format_binary_like_expression, JsAnyBinaryLikeExpression};

use rome_js_syntax::JsBinaryExpression;

#[derive(Debug, Clone, Default)]
pub struct FormatJsBinaryExpression;

impl FormatNodeRule<JsBinaryExpression> for FormatJsBinaryExpression {
    fn fmt_fields(
        &self,
        node: &JsBinaryExpression,
        formatter: &mut JsFormatter,
    ) -> FormatResult<()> {
        format_binary_like_expression(
            JsAnyBinaryLikeExpression::JsBinaryExpression(node.clone()),
            formatter,
        )
    }
}
