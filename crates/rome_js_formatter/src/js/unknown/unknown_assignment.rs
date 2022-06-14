use crate::prelude::*;
use crate::FormatNodeFields;
use rome_js_syntax::JsUnknownAssignment;
use rome_rowan::AstNode;

impl FormatNodeFields<JsUnknownAssignment> for FormatNodeRule<JsUnknownAssignment> {
    fn fmt_fields(node: &JsUnknownAssignment, formatter: &mut JsFormatter) -> FormatResult<()> {
        format_unknown_node(node.syntax()).fmt(formatter)
    }
}
