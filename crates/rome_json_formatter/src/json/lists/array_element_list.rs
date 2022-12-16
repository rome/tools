use crate::prelude::*;
use rome_json_syntax::JsonArrayElementList;
use crate::separated::FormatAstSeparatedListExtension;
use rome_rowan::{AstNode, AstSeparatedList};

#[derive(Debug, Clone, Default)]
pub(crate) struct FormatJsonArrayElementList;

impl FormatRule<JsonArrayElementList> for FormatJsonArrayElementList {
    type Context = JsonFormatContext;
    fn fmt(&self, node: &JsonArrayElementList, f: &mut JsonFormatter) -> FormatResult<()> {
		let mut join = f.join_nodes_with_soft_line();

        for (element, formatted) in node.elements().zip(node.format_separated(",")) {
            join.entry(element.node()?.syntax(), &formatted);
        }

        join.finish()
    }
}
