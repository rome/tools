use crate::prelude::*;
use rome_formatter::write;

use crate::builders::format_delimited;
use rome_js_syntax::{TsEnumDeclaration, TsEnumDeclarationFields};

#[derive(Debug, Clone, Default)]
pub struct FormatTsEnumDeclaration;

impl FormatNodeRule<TsEnumDeclaration> for FormatTsEnumDeclaration {
    fn fmt_fields(&self, node: &TsEnumDeclaration, f: &mut JsFormatter) -> FormatResult<()> {
        let TsEnumDeclarationFields {
            const_token,
            enum_token,
            id,
            members,
            l_curly_token,
            r_curly_token,
        } = node.as_fields();

        if let Some(const_token) = const_token {
            write!(f, [const_token.format(), space()])?;
        }

        write!(
            f,
            [
                enum_token.format(),
                space(),
                id.format(),
                space(),
                format_delimited(&l_curly_token?, &members.format(), &r_curly_token?,)
                    .soft_block_spaces()
            ]
        )
    }
}
