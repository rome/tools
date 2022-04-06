use crate::formatter_traits::{FormatOptionalTokenAndNode, FormatTokenAndNode};
use crate::utils::format_with_semicolon;
use crate::{
    format_elements, hard_group_elements, space_token, FormatElement, FormatResult, Formatter,
    ToFormatElement,
};
use rome_js_syntax::TsDeclareFunctionDeclaration;
use rome_js_syntax::TsDeclareFunctionDeclarationFields;

impl ToFormatElement for TsDeclareFunctionDeclaration {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let TsDeclareFunctionDeclarationFields {
            async_token,
            function_token,
            id,
            type_parameters,
            parameters,
            return_type_annotation,
            semicolon_token,
        } = self.as_fields();

        let async_token = async_token.format_with_or_empty(formatter, |async_token| {
            format_elements![async_token, space_token()]
        })?;

        let function_token = function_token.format(formatter)?;
        let id = id.format(formatter)?;
        let type_parameters = type_parameters.format_or_empty(formatter)?;
        let parameters = parameters.format(formatter)?;
        let return_type_annotation = return_type_annotation.format_or_empty(formatter)?;

        Ok(hard_group_elements(format_with_semicolon(
            formatter,
            format_elements![
                async_token,
                function_token,
                space_token(),
                id,
                type_parameters,
                parameters,
                return_type_annotation,
            ],
            semicolon_token,
        )?))
    }
}
