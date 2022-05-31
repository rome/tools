use crate::prelude::*;
use crate::utils::format_with_semicolon;
use crate::FormatNodeFields;
use rome_js_syntax::TsDeclareFunctionDeclaration;
use rome_js_syntax::TsDeclareFunctionDeclarationFields;

impl FormatNodeFields<TsDeclareFunctionDeclaration>
    for FormatNodeRule<TsDeclareFunctionDeclaration>
{
    fn format_fields(
        node: &TsDeclareFunctionDeclaration,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let TsDeclareFunctionDeclarationFields {
            async_token,
            function_token,
            id,
            type_parameters,
            parameters,
            return_type_annotation,
            semicolon_token,
        } = node.as_fields();

        format_with_semicolon(
            formatter,
            formatted![
                formatter,
                [
                    async_token.format().with_or_empty(|async_token| formatted![
                        formatter,
                        [async_token, space_token()]
                    ]),
                    function_token.format(),
                    space_token(),
                    id.format(),
                    type_parameters.format(),
                    parameters.format(),
                    return_type_annotation.format(),
                ]
            ]?,
            semicolon_token,
        )
    }
}
