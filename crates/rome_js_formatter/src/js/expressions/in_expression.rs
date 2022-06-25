use crate::prelude::*;
use crate::utils::{format_binary_like_expression, JsAnyBinaryLikeExpression};

use rome_js_syntax::JsInExpression;

#[derive(Debug, Clone, Default)]
pub struct FormatJsInExpression;

impl FormatNodeRule<JsInExpression> for FormatJsInExpression {
    fn fmt_fields(&self, node: &JsInExpression, formatter: &mut JsFormatter) -> FormatResult<()> {
        format_binary_like_expression(
            JsAnyBinaryLikeExpression::JsInExpression(node.clone()),
            formatter,
        )
    }
}
