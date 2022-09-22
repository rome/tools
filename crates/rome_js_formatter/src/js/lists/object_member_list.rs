use crate::prelude::*;
use rome_js_syntax::JsObjectMemberList;
use rome_rowan::{AstNode, AstSeparatedList};

#[derive(Debug, Clone, Default)]
pub struct FormatJsObjectMemberList;

impl FormatRule<JsObjectMemberList> for FormatJsObjectMemberList {
    type Context = JsFormatContext;

    fn fmt(&self, node: &JsObjectMemberList, f: &mut JsFormatter) -> FormatResult<()> {
        let mut join = f.join_nodes_with_soft_line();

        for (element, formatted) in node.elements().zip(node.format_separated(",")) {
            join.entry(element.node()?.syntax(), &formatted);
        }

        join.finish()
    }
}
