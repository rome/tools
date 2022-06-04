use crate::prelude::*;
use crate::utils::FormatWithSemicolon;
use crate::FormatNodeFields;
use rome_formatter::write;
use rome_js_syntax::TsDeclareFunctionDeclaration;
use rome_js_syntax::TsDeclareFunctionDeclarationFields;

impl FormatNodeFields<TsDeclareFunctionDeclaration>
    for FormatNodeRule<TsDeclareFunctionDeclaration>
{
    fn fmt_fields(node: &TsDeclareFunctionDeclaration, f: &mut JsFormatter) -> FormatResult<()> {
        let TsDeclareFunctionDeclarationFields {
            async_token,
            function_token,
            id,
            type_parameters,
            parameters,
            return_type_annotation,
            semicolon_token,
        } = node.as_fields();

        let declaration = format_with(|f| {
            if let Some(async_token) = &async_token {
                write!(f, [async_token.format(), space_token()])?;
            }

            write!(
                f,
                [
                    function_token.format(),
                    space_token(),
                    id.format(),
                    type_parameters.format(),
                    parameters.format(),
                    return_type_annotation.format(),
                ]
            )
        });

        write!(
            f,
            [FormatWithSemicolon::new(
                &declaration,
                semicolon_token.as_ref()
            )]
        )
    }
}
