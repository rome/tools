use crate::prelude::*;
use crate::utils::FormatWithSemicolon;
use crate::FormatNodeFields;
use rome_formatter::{format_args, write};
use rome_js_syntax::{TsTypeAliasDeclaration, TsTypeAliasDeclarationFields};

impl FormatNodeFields<TsTypeAliasDeclaration> for FormatNodeRule<TsTypeAliasDeclaration> {
    fn fmt_fields(node: &TsTypeAliasDeclaration, f: &mut JsFormatter) -> FormatResult<()> {
        let TsTypeAliasDeclarationFields {
            type_token,
            binding_identifier,
            type_parameters,
            eq_token,
            ty,
            semicolon_token,
        } = node.as_fields();

        write!(
            f,
            [FormatWithSemicolon::new(
                &format_args!(
                    type_token.format(),
                    space_token(),
                    binding_identifier.format(),
                    type_parameters.format(),
                    space_token(),
                    eq_token.format(),
                    space_token(),
                    ty.format(),
                ),
                semicolon_token.as_ref()
            )]
        )
    }
}
