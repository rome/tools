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
        let type_parameters = type_parameters;

        let type_parameter_group = type_parameters
            .as_ref()
            .map(|_| f.group_id("type-parameters"));

        let format_id = format_with(|f| {
            write!(f, [id.format(),])?;

            if let Some(type_parameters) = &type_parameters {
                write!(
                    f,
                    [type_parameters.format().with_options(type_parameter_group)]
                )?;
            }

            Ok(())
        });

        let should_indent_extends_only = type_parameters
            .as_ref()
            .map_or(false, |params| !params.syntax().has_trailing_comments());

        let format_extends = format_with(|f| {
            if let Some(extends_clause) = &extends_clause {
                if should_indent_extends_only {
                    write!(
                        f,
                        [
                            if_group_breaks(&space()).with_group_id(type_parameter_group),
                            if_group_fits_on_line(&soft_line_break_or_space())
                                .with_group_id(type_parameter_group),
                        ]
                    )?;
                } else {
                    write!(f, [soft_line_break_or_space()])?;
                }

                write!(f, [extends_clause.format(), space()])?;
            }

            Ok(())
        });

        write![f, [interface_token.format(), space()]]?;

        // Manually handle the trailing comments and push them into the members block
        // to prevent that a comment gets formatted on the same line as the opening `{`
        let last_node = match (&type_parameters, &extends_clause) {
            (_, Some(extends_clause)) => extends_clause.syntax(),
            (Some(type_parameters), None) => type_parameters.syntax(),
            (None, None) => id.syntax(),
        };

        let last_token = last_node.last_token();
        let mut has_trailing_comments = false;

        if let Some(last_token) = &last_token {
            for comment in last_token
                .trailing_trivia()
                .pieces()
                .filter_map(|piece| piece.as_comments())
            {
                has_trailing_comments = true;
                f.state_mut().mark_comment_as_formatted(&comment);
            }
        }

        let id_has_trailing_comments = id.syntax().has_trailing_comments();
        if id_has_trailing_comments || extends_clause.is_some() {
            if should_indent_extends_only {
                write!(
                    f,
                    [group(&format_args!(format_id, indent(&format_extends)))]
                )?;
            } else {
                write!(
                    f,
                    [group(&indent(&format_args!(format_id, format_extends)))]
                )?;
            }
        } else {
            write!(f, [format_id, format_extends])?;
        }

        write!(
            f,
            [
                space(),
                format_delimited(
                    &l_curly_token,
                    &format_args![
                        format_with(|f| {
                            // Write the manual handled comments
                            if let Some(last_token) = &last_token {
                                if has_trailing_comments {
                                    write!(
                                        f,
                                        [
                                            format_trailing_trivia(last_token)
                                                .skip_formatted_check(),
                                            hard_line_break()
                                        ]
                                    )?;
                                }
                            }

                            Ok(())
                        }),
                        members.format()
                    ],
                    &r_curly_token
                )
                .block_indent()
            ]
        )
    }
}
