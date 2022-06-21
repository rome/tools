use crate::prelude::*;
use rome_js_syntax::JsClassMemberList;

#[derive(Debug, Clone, Default)]
pub struct FormatJsClassMemberList;

impl FormatNodeRule<JsClassMemberList> for FormatJsClassMemberList {
    fn fmt_fields(&self, node: &JsClassMemberList, f: &mut JsFormatter) -> FormatResult<()> {
        let mut join = f.join_nodes_with_hardline();

        for member in node {
            join.entry(member.syntax(), &format_or_verbatim(&member));
        }

        join.finish()
    }
}
