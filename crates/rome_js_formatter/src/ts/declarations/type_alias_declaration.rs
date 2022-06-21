use crate::prelude::*;
use crate::utils::FormatWithSemicolon;

use rome_formatter::{format_args, write};
use rome_js_syntax::{TsTypeAliasDeclaration, TsTypeAliasDeclarationFields};

#[derive(Debug, Clone, Default)]
pub struct FormatTsTypeAliasDeclaration;

impl FormatNodeRule<TsTypeAliasDeclaration> for FormatTsTypeAliasDeclaration {
    fn fmt_fields(&self, node: &TsTypeAliasDeclaration, f: &mut JsFormatter) -> FormatResult<()> {
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
