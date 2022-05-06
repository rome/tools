use crate::format_traits::FormatOptional;
use crate::utils::format_with_semicolon;
use crate::{hard_group_elements, space_token, Format, FormatElement, FormatNode, Formatter};
use rome_formatter::FormatResult;
use rome_js_syntax::TsDeclareFunctionDeclaration;
use rome_js_syntax::TsDeclareFunctionDeclarationFields;

impl FormatNode for TsDeclareFunctionDeclaration {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
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
            formatted![formatter, async_token, space_token()]
        })?;

        let function_token = function_token.format(formatter)?;
        let id = id.format(formatter)?;
        let parameters = parameters.format(formatter)?;

        Ok(hard_group_elements(format_with_semicolon(
            formatter,
            formatted![
                formatter,
                async_token,
                function_token,
                space_token(),
                id,
                type_parameters,
                parameters,
                return_type_annotation,
            ]?,
            semicolon_token,
        )?))
    }
}
