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

        let eq_token = eq_token?;

        let content = format_with(|f| {
            write!(
                f,
                [
                    group_elements(&format_args![
                        type_token.format(),
                        space_token(),
                        binding_identifier.format(),
                        type_parameters.format(),
                    ]),
                    space_token(),
                    eq_token.format(),
                ]
            )?;

            if eq_token.has_trailing_comments() {
                let ty = ty.format().memoized();
                let group_id = f.group_id("assignment");

                write!(
                    f,
                    [
                        group_elements(&indent(&soft_line_break_or_space()))
                            .with_group_id(Some(group_id)),
                        line_suffix_boundary(),
                        if_group_breaks(&indent(&ty)).with_group_id(Some(group_id)),
                        if_group_fits_on_line(&format_args![space_token(), ty])
                            .with_group_id(Some(group_id))
                    ]
                )
            } else {
                write!(f, [space_token(), ty.format()])
            }
        });

        write!(
            f,
            [FormatWithSemicolon::new(&content, semicolon_token.as_ref())]
        )
    }
}
