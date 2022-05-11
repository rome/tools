use crate::formatter::TrailingSeparator;
use crate::generated::FormatTsEnumMemberList;
use crate::prelude::*;
use rome_js_syntax::TsEnumMemberList;

impl FormatRule<TsEnumMemberList> for FormatTsEnumMemberList {
    fn format(node: &TsEnumMemberList, formatter: &Formatter) -> FormatResult<FormatElement> {
        Ok(join_elements(
            soft_line_break_or_space(),
            formatter.format_separated(node, || token(","), TrailingSeparator::default())?,
        ))
    }
}
