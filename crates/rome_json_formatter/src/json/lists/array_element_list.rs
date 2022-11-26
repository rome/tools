use crate::prelude::*;
use rome_json_syntax::JsonArrayElementList;

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsonArrayElementList;

impl FormatRule<JsonArrayElementList> for FormatJsonArrayElementList {
    type Context = JsonFormatContext;
    fn fmt(&self, node: &JsonArrayElementList, f: &mut JsonFormatter) -> FormatResult<()> {
        format_verbatim_node(node.syntax()).fmt(f)
    }
}
