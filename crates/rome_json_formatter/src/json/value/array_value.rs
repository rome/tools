use crate::prelude::*;
use rome_json_syntax::JsonArrayValue;
use rome_rowan::AstNode;
#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsonArrayValue;
impl FormatNodeRule<JsonArrayValue> for FormatJsonArrayValue {
    fn fmt_fields(&self, node: &JsonArrayValue, f: &mut JsonFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
