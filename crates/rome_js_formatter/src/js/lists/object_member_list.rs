use crate::prelude::*;
use rome_js_syntax::{JsObjectMemberList, JsSyntaxKind};
use rome_rowan::{AstNode, AstSeparatedList};

#[derive(Debug, Clone, Default)]
pub struct FormatJsObjectMemberList;

impl FormatNodeRule<JsObjectMemberList> for FormatJsObjectMemberList {
    fn fmt_fields(&self, node: &JsObjectMemberList, f: &mut JsFormatter) -> FormatResult<()> {
        let mut join = f.join_nodes_with_soft_line();

        for (element, formatted) in node
            .elements()
            .zip(node.format_separated(JsSyntaxKind::COMMA))
        {
            join.entry(element.node()?.syntax(), &formatted);
        }

        join.finish()
    }
}
