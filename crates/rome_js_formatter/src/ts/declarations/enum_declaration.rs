use crate::prelude::*;
use rome_formatter::write;

use crate::FormatNodeFields;
use rome_js_syntax::{TsEnumDeclaration, TsEnumDeclarationFields};

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
                space_token(),
                format_delimited(&l_curly_token?, &members.format(), &r_curly_token?,)
                    .soft_block_spaces()
            ]
        )
    }
}
