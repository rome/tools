use crate::prelude::*;
use crate::utils::FormatWithSemicolon;

use rome_formatter::write;
use rome_js_syntax::TsDeclareFunctionDeclaration;
use rome_js_syntax::TsDeclareFunctionDeclarationFields;

#[derive(Debug, Clone, Default)]
pub struct FormatTsDeclareFunctionDeclaration;

impl FormatNodeRule<TsDeclareFunctionDeclaration> for FormatTsDeclareFunctionDeclaration {
    fn fmt_fields(
        &self,
        node: &TsDeclareFunctionDeclaration,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
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
                write!(f, [async_token.format(), space()])?;
            }

            write!(
                f,
                [
                    function_token.format(),
                    space(),
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
