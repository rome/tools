use crate::prelude::*;

use rome_formatter::write;
use rome_js_syntax::{TsInterfaceDeclaration, TsInterfaceDeclarationFields};

#[derive(Debug, Clone, Default)]
pub struct FormatTsInterfaceDeclaration;

impl FormatNodeRule<TsInterfaceDeclaration> for FormatTsInterfaceDeclaration {
    fn fmt_fields(&self, node: &TsInterfaceDeclaration, f: &mut JsFormatter) -> FormatResult<()> {
        let TsInterfaceDeclarationFields {
            interface_token,
            id,
            type_parameters,
            extends_clause,
            members,
            l_curly_token,
            r_curly_token,
        } = node.as_fields();

        write![
            f,
            [
                interface_token.format(),
                space_token(),
                id.format(),
                type_parameters.format(),
                space_token(),
            ]
        ]?;

        if let Some(extends_clause) = extends_clause {
            write!(f, [extends_clause.format(), space_token()])?;
        }

        write!(
            f,
            [
                format_delimited(&l_curly_token?, &members.format(), &r_curly_token?,)
                    .block_indent()
            ]
        )
    }
}
