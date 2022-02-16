use crate::formatter_traits::{FormatOptionalTokenAndNode, FormatTokenAndNode};
use crate::{
    format_elements, group_elements, soft_block_indent, soft_line_break_or_space, space_token,
    FormatElement, FormatResult, Formatter, ToFormatElement,
};
use rslint_parser::ast::TsInterfaceDeclaration;

impl ToFormatElement for TsInterfaceDeclaration {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let interface = self.interface_token().format(formatter)?;
        let id = self.id().format(formatter)?;
        let type_parameters = self.type_parameters().format_or_empty(formatter)?;
        let extends = self
            .extends_clause()
            .format_with_or_empty(formatter, |extends| {
                format_elements![extends, space_token()]
            })?;
        let members = group_elements(formatter.format_delimited(
            &self.l_curly_token()?,
            |open_token_trailing, close_token_leading| {
                let list = self.members().to_format_element(formatter)?;
                Ok(format_elements![
                    soft_line_break_or_space(),
                    soft_block_indent(format_elements![
                        open_token_trailing,
                        list,
                        close_token_leading
                    ]),
                    soft_line_break_or_space()
                ])
            },
            &self.r_curly_token()?,
        )?);
        Ok(format_elements![
            interface,
            space_token(),
            id,
            type_parameters,
            space_token(),
            extends,
            members
        ])
    }
}
