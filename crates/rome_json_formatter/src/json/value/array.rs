use crate::prelude::*;
use rome_json_syntax::JsonArray;
use rome_rowan::AstNode;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsonArray;

impl FormatNodeRule<JsonArray> for FormatJsonArray {
    fn fmt_fields(&self, node: &JsonArray, f: &mut JsonFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
