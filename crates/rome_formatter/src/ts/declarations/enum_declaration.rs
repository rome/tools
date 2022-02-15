use crate::formatter_traits::{FormatOptionalTokenAndNode, FormatTokenAndNode};
use crate::{
    format_elements, join_elements, soft_line_break_or_space, space_token, token, FormatElement,
    FormatResult, Formatter, ToFormatElement,
};
use rslint_parser::ast::TsEnumDeclaration;

impl ToFormatElement for TsEnumDeclaration {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let const_token = self
            .const_token()
            .format_with_or_empty(formatter, |const_token| {
                format_elements![const_token, space_token()]
            })?;
        let enum_token = self.enum_token().format_with(formatter, |enum_token| {
            format_elements![enum_token, space_token()]
        })?;
        let id = self
            .id()
            .format_with(formatter, |id| format_elements![id, space_token()])?;

        let members = formatter.format_separated(self.members(), || token(","))?;
        let list = formatter.format_delimited_soft_block_spaces(
            &self.l_curly_token()?,
            join_elements(soft_line_break_or_space(), members),
            &self.r_curly_token()?,
        )?;

        Ok(format_elements![const_token, enum_token, id, list])
    }
}
