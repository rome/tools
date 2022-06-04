use crate::prelude::*;
use rome_formatter::write;

use crate::utils::has_leading_newline;
use crate::FormatNodeFields;
use rome_js_syntax::{TsEnumDeclaration, TsEnumDeclarationFields};
use rome_rowan::AstNode;

impl FormatNodeFields<TsEnumDeclaration> for FormatNodeRule<TsEnumDeclaration> {
    fn fmt_fields(node: &TsEnumDeclaration, f: &mut JsFormatter) -> FormatResult<()> {
        let TsEnumDeclarationFields {
            const_token,
            enum_token,
            id,
            members,
            l_curly_token,
            r_curly_token,
        } = node.as_fields();

        if let Some(const_token) = const_token {
            write!(f, [const_token.format(), space_token()])?;
        }

        write!(
            f,
            [
                enum_token.format(),
                space_token(),
                id.format(),
                space_token()
            ]
        )?;

        let has_newline = has_leading_newline(members.syntax());

        let members = format_with(|f| {
            f.join_with(&if has_newline {
                hard_line_break()
            } else {
                soft_line_break_or_space()
            })
            .entries(members.format_separated(token(",")))
            .finish()
        });

        write!(
            f,
            [format_delimited(&l_curly_token?, &members, &r_curly_token?,).soft_block_spaces()]
        )
    }
}
