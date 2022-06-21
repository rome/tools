use crate::prelude::*;

use rome_js_syntax::JsUnknownExpression;
use rome_rowan::AstNode;

#[derive(Debug, Clone, Default)]
pub struct FormatJsUnknownExpression;

impl FormatNodeRule<JsUnknownExpression> for FormatJsUnknownExpression {
    fn fmt_fields(
        &self,
        node: &JsUnknownExpression,
        formatter: &mut JsFormatter,
    ) -> FormatResult<()> {
        format_unknown_node(node.syntax()).fmt(formatter)
    }
}
