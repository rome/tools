use crate::prelude::*;

use rome_js_syntax::JsUnknownStatement;
use rome_rowan::AstNode;

#[derive(Debug, Clone, Default)]
pub struct FormatJsUnknownStatement;

impl FormatNodeRule<JsUnknownStatement> for FormatJsUnknownStatement {
    fn fmt_fields(
        &self,
        node: &JsUnknownStatement,
        formatter: &mut JsFormatter,
    ) -> FormatResult<()> {
        format_unknown_node(node.syntax()).fmt(formatter)
    }
}
