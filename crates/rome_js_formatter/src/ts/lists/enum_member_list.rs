use crate::generated::FormatTsEnumMemberList;
use crate::prelude::*;
use rome_js_syntax::TsEnumMemberList;

impl FormatRule<TsEnumMemberList> for FormatTsEnumMemberList {
    type Options = JsFormatOptions;

    fn format(
        node: &TsEnumMemberList,
        formatter: &Formatter<JsFormatOptions>,
    ) -> FormatResult<FormatElement> {
        Ok(join_elements(
            soft_line_break_or_space(),
            formatter.format_separated(node, || token(","))?,
        ))
    }
}
