use crate::prelude::*;

use rome_js_syntax::JsUnknownMember;
use rome_rowan::AstNode;

#[derive(Debug, Clone, Default)]
pub struct FormatJsUnknownMember;

impl FormatNodeRule<JsUnknownMember> for FormatJsUnknownMember {
    fn fmt_fields(&self, node: &JsUnknownMember, formatter: &mut JsFormatter) -> FormatResult<()> {
        format_unknown_node(node.syntax()).fmt(formatter)
    }
}
