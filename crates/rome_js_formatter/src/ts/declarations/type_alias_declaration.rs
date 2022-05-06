use crate::format_traits::FormatOptional;
use crate::utils::format_with_semicolon;
use crate::{
    hard_group_elements, space_token, Format, FormatElement, FormatNode, Formatter,
};
use rome_formatter::FormatResult;
use rome_js_syntax::{TsTypeAliasDeclaration, TsTypeAliasDeclarationFields};

impl FormatNode for TsTypeAliasDeclaration {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
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
            formatted![
                formatter,
                type_token,
                space_token(),
                binding_identifier,
                type_parameters,
                space_token(),
                equal_token,
                space_token(),
                ty,
            ]?,
            semicolon_token,
        )?))
    }
}
