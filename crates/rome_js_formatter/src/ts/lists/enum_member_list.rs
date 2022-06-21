use crate::generated::FormatTsEnumMemberList;
use crate::prelude::*;
use crate::utils::node_has_leading_newline;
use rome_js_syntax::{JsSyntaxKind, TsEnumMemberList};

impl FormatRule<TsEnumMemberList> for FormatTsEnumMemberList {
    type Context = JsFormatContext;

    fn fmt(node: &TsEnumMemberList, f: &mut JsFormatter) -> FormatResult<()> {
        let has_newline = node_has_leading_newline(node.syntax());

        f.join_with(&if has_newline {
            hard_line_break()
        } else {
            soft_line_break_or_space()
        })
        .entries(node.format_separated(JsSyntaxKind::COMMA))
        .finish()
    }
}
