use crate::formatter_traits::{FormatOptionalTokenAndNode, FormatTokenAndNode};
use crate::utils::format_with_semicolon;
use crate::{
    format_elements, hard_group_elements, space_token, FormatElement, FormatResult, Formatter,
    ToFormatElement,
};
use rome_js_syntax::{TsTypeAliasDeclaration, TsTypeAliasDeclarationFields};

impl ToFormatElement for TsTypeAliasDeclaration {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let TsTypeAliasDeclarationFields {
            type_token,
            binding_identifier,
            type_parameters,
            eq_token,
            ty,
            semicolon_token,
        } = self.as_fields();

        let type_token = type_token.format(formatter)?;
        let binding_identifier = binding_identifier.format(formatter)?;
        let type_parameters = type_parameters.format_or_empty(formatter)?;
        let equal_token = eq_token.format(formatter)?;
        let ty = ty.format(formatter)?;

        Ok(hard_group_elements(format_with_semicolon(
            formatter,
            format_elements![
                type_token,
                space_token(),
                binding_identifier,
                type_parameters,
                space_token(),
                equal_token,
                space_token(),
                ty,
            ],
            semicolon_token,
        )?))
    }
}
