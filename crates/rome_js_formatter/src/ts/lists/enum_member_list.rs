use crate::generated::FormatTsEnumMemberList;
use crate::prelude::*;
use rome_js_syntax::TsEnumMemberList;

impl FormatRule<TsEnumMemberList> for FormatTsEnumMemberList {
    type Context = JsFormatContext;

    fn fmt(node: &TsEnumMemberList, f: &mut JsFormatter) -> FormatResult<()> {
        f.join_with(&soft_line_break_or_space())
            .entries(node.format_separated(token(",")))
            .finish()
    }
}
