use crate::prelude::*;
use rome_json_syntax::JsonMember;
use rome_rowan::AstNode;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsonMember;

impl FormatNodeRule<JsonMember> for FormatJsonMember {
    fn fmt_fields(&self, node: &JsonMember, f: &mut JsonFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
