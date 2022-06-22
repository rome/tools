use crate::prelude::*;

use rome_formatter::{format_args, write};
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

        let l_curly_token = l_curly_token?;
        let r_curly_token = r_curly_token?;
        let id = id?;

        let format_id = format_with(|f| write!(f, [id.format(), type_parameters.format()]));

        let format_extends = format_with(|f| {
            if let Some(extends_clause) = &extends_clause {
                write!(
                    f,
                    [
                        soft_line_break_or_space(),
                        extends_clause.format(),
                        space_token()
                    ]
                )?;
            }

            Ok(())
        });

        write![f, [interface_token.format(), space_token()]]?;

        let should_indent_extends_only = type_parameters
            .as_ref()
            .map_or(true, |params| !params.syntax().has_trailing_comments());

        let id_has_trailing_comments = id.syntax().has_trailing_comments();
        if id_has_trailing_comments || extends_clause.is_some() {
            if should_indent_extends_only {
                write!(
                    f,
                    [group_elements(&format_args!(
                        format_id,
                        indent(&format_extends)
                    ))]
                )?;
            } else {
                write!(
                    f,
                    [group_elements(&indent(&format_args!(
                        format_id,
                        format_extends
                    )))]
                )?;
            }
        } else {
            write!(f, [format_id, format_extends])?;
        }

        write!(
            f,
            [
                space_token(),
                format_delimited(&l_curly_token, &members.format(), &r_curly_token).block_indent()
            ]
        )
    }
}
