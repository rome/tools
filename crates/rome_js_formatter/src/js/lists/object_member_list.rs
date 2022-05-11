use crate::formatter::TrailingSeparator;
use crate::generated::FormatJsObjectMemberList;
use crate::prelude::*;
use rome_js_syntax::JsObjectMemberList;
use rome_rowan::{AstNode, AstSeparatedList};

impl FormatRule<JsObjectMemberList> for FormatJsObjectMemberList {
    type Options = JsFormatOptions;

    fn format(
        node: &JsObjectMemberList,
        formatter: &Formatter<JsFormatOptions>,
    ) -> FormatResult<FormatElement> {
        let members =
            formatter.format_separated(node, || token(","), TrailingSeparator::default())?;

        Ok(join_elements_soft_line(
            node.elements()
                // This unwrap is guarded by the call to format_separated above
                .map(|node| node.node().unwrap().syntax().clone())
                .zip(members),
        ))
    }
}
