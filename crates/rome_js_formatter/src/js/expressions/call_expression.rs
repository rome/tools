use crate::prelude::*;
use crate::utils::format_call_expression;

use rome_js_syntax::JsCallExpression;
use rome_rowan::AstNode;

#[derive(Debug, Clone, Default)]
pub struct FormatJsCallExpression;

impl FormatNodeRule<JsCallExpression> for FormatJsCallExpression {
    fn fmt_fields(&self, node: &JsCallExpression, formatter: &mut JsFormatter) -> FormatResult<()> {
        format_call_expression(node.syntax(), formatter)
    }
}
