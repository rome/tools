use crate::prelude::*;
use crate::utils::{format_binary_like_expression, JsAnyBinaryLikeExpression};

use rome_js_syntax::JsInstanceofExpression;

#[derive(Debug, Clone, Default)]
pub struct FormatJsInstanceofExpression;

impl FormatNodeRule<JsInstanceofExpression> for FormatJsInstanceofExpression {
    fn fmt_fields(
        &self,
        node: &JsInstanceofExpression,
        formatter: &mut JsFormatter,
    ) -> FormatResult<()> {
        format_binary_like_expression(
            JsAnyBinaryLikeExpression::JsInstanceofExpression(node.clone()),
            formatter,
        )
    }
}
