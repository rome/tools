use crate::prelude::*;

use rome_js_syntax::JsxExpressionChild;
use rome_rowan::AstNode;

#[derive(Debug, Clone, Default)]
pub struct FormatJsxExpressionChild;

impl FormatNodeRule<JsxExpressionChild> for FormatJsxExpressionChild {
    fn fmt_fields(
        &self,
        node: &JsxExpressionChild,
        formatter: &mut JsFormatter,
    ) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(formatter)
    }
}
