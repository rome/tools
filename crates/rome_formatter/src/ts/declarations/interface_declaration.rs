use crate::formatter_traits::{FormatOptionalTokenAndNode, FormatTokenAndNode};
use crate::{
    format_elements, group_elements, space_token, FormatElement, FormatResult, Formatter,
    ToFormatElement,
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
        let members = group_elements(formatter.format_delimited_soft_block_spaces(
            &self.l_curly_token()?,
            self.members().format(formatter)?,
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
