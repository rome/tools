use crate::prelude::*;

use rome_js_syntax::JsUnknownAssignment;
use rome_rowan::AstNode;

#[derive(Debug, Clone, Default)]
pub struct FormatJsUnknownAssignment;

impl FormatNodeRule<JsUnknownAssignment> for FormatJsUnknownAssignment {
    fn fmt_fields(
        &self,
        node: &JsUnknownAssignment,
        formatter: &mut JsFormatter,
    ) -> FormatResult<()> {
        format_unknown_node(node.syntax()).fmt(formatter)
    }
}
