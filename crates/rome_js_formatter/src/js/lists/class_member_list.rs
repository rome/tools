use crate::generated::FormatJsClassMemberList;
use crate::prelude::*;
use rome_js_syntax::JsClassMemberList;

impl FormatRule<JsClassMemberList> for FormatJsClassMemberList {
    type Context = JsFormatContext;

    fn fmt(node: &JsClassMemberList, f: &mut JsFormatter) -> FormatResult<()> {
        let mut join = f.join_nodes_with_hardline();

        for member in node {
            join.entry(member.syntax(), &format_or_verbatim(&member));
        }

        join.finish()
    }
}
