use crate::prelude::*;
use rome_json_syntax::JsonObject;
use rome_rowan::AstNode;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsonObject;

impl FormatNodeRule<JsonObject> for FormatJsonObject {
    fn fmt_fields(&self, node: &JsonObject, f: &mut JsonFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
