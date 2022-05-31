use crate::formatter::verbatim_node;
use crate::prelude::*;
use crate::FormatNodeFields;
use rome_js_syntax::JsUnknownAssignment;
use rome_rowan::AstNode;

impl FormatNodeFields<JsUnknownAssignment> for FormatNodeRule<JsUnknownAssignment> {
    fn format_fields(
        node: &JsUnknownAssignment,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        verbatim_node(node.syntax()).format(formatter)
    }
}
