use crate::formatter_traits::{FormatOptionalTokenAndNode, FormatTokenAndNode};
use crate::{
    format_elements, hard_group_elements, space_token, FormatElement, FormatResult, Formatter,
    ToFormatElement,
};
use rome_js_syntax::{TsInterfaceDeclaration, TsInterfaceDeclarationFields};

impl ToFormatElement for TsInterfaceDeclaration {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let TsInterfaceDeclarationFields {
            interface_token,
            id,
            type_parameters,
            extends_clause,
            members,
            l_curly_token,
            r_curly_token,
        } = self.as_fields();
        let interface = interface_token.format(formatter)?;
        let id = id.format(formatter)?;
        let type_parameters = type_parameters.format_or_empty(formatter)?;
        let extends = extends_clause.format_with_or_empty(formatter, |extends| {
            format_elements![extends, space_token()]
        })?;
        let members = formatter.format_delimited_soft_block_spaces(
            &l_curly_token?,
            members.format(formatter)?,
            &r_curly_token?,
        )?;
        Ok(hard_group_elements(format_elements![
            interface,
            space_token(),
            id,
            type_parameters,
            space_token(),
            extends,
            members
        ]))
    }
}
