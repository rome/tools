use crate::prelude::*;
use crate::utils::node_has_leading_newline;
use rome_js_syntax::TsEnumMemberList;

#[derive(Debug, Clone, Default)]
pub struct FormatTsEnumMemberList;

impl FormatRule<TsEnumMemberList> for FormatTsEnumMemberList {
    type Context = JsFormatContext;

    fn fmt(&self, node: &TsEnumMemberList, f: &mut JsFormatter) -> FormatResult<()> {
        let has_newline = node_has_leading_newline(node.syntax());

        f.join_with(&if has_newline {
            hard_line_break()
        } else {
            soft_line_break_or_space()
        })
        .entries(node.format_separated(",").nodes_grouped())
        .finish()
    }
}
