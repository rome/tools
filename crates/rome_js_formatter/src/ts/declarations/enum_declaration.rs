use crate::formatter::TrailingSeparator;
use crate::formatter_traits::{FormatOptionalTokenAndNode, FormatTokenAndNode};
use crate::{
    format_elements, join_elements, soft_line_break_or_space, space_token, token, FormatElement,
    FormatResult, Formatter, ToFormatElement,
};
use rome_js_syntax::{TsEnumDeclaration, TsEnumDeclarationFields};

impl ToFormatElement for TsEnumDeclaration {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let TsEnumDeclarationFields {
            const_token,
            enum_token,
            id,
            members,
            l_curly_token,
            r_curly_token,
        } = self.as_fields();

        let const_token = const_token.format_with_or_empty(formatter, |const_token| {
            format_elements![const_token, space_token()]
        })?;
        let enum_token = enum_token.format_with(formatter, |enum_token| {
            format_elements![enum_token, space_token()]
        })?;
        let id = id.format_with(formatter, |id| format_elements![id, space_token()])?;

        let members =
            formatter.format_separated(&members, || token(","), TrailingSeparator::default())?;
        let list = formatter.format_delimited_soft_block_spaces(
            &l_curly_token?,
            join_elements(soft_line_break_or_space(), members),
            &r_curly_token?,
        )?;

        Ok(format_elements![const_token, enum_token, id, list])
    }
}
