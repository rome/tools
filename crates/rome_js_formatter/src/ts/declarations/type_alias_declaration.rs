use crate::prelude::*;
use crate::utils::format_with_semicolon;
use crate::FormatNodeFields;
use rome_js_syntax::{TsTypeAliasDeclaration, TsTypeAliasDeclarationFields};

impl FormatNodeFields<TsTypeAliasDeclaration> for FormatNodeRule<TsTypeAliasDeclaration> {
    fn format_fields(
        node: &TsTypeAliasDeclaration,
        formatter: &Formatter<JsFormatOptions>,
    ) -> FormatResult<FormatElement> {
        let TsTypeAliasDeclarationFields {
            type_token,
            binding_identifier,
            type_parameters,
            eq_token,
            ty,
            semicolon_token,
        } = node.as_fields();

        format_with_semicolon(
            formatter,
            formatted![
                formatter,
                [
                    type_token.format(),
                    space_token(),
                    binding_identifier.format(),
                    type_parameters.format(),
                    space_token(),
                    eq_token.format(),
                    space_token(),
                    ty.format(),
                ]
            ]?,
            semicolon_token,
        )
    }
}
