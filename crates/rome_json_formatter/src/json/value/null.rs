use crate::prelude::*;
use rome_json_syntax::JsonNull;
use rome_rowan::AstNode;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsonNull;

impl FormatNodeRule<JsonNull> for FormatJsonNull {
    fn fmt_fields(&self, node: &JsonNull, f: &mut JsonFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
