use crate::prelude::*;

use crate::parentheses::resolve_parent;
use rome_formatter::{format_args, write};
use rome_js_syntax::JsExtendsClauseFields;
use rome_js_syntax::JsSyntaxKind::JS_ASSIGNMENT_EXPRESSION;
use rome_js_syntax::{JsExtendsClause, JsSyntaxKind};

#[derive(Debug, Clone, Default)]
pub struct FormatJsExtendsClause;

impl FormatNodeRule<JsExtendsClause> for FormatJsExtendsClause {
    fn fmt_fields(&self, node: &JsExtendsClause, f: &mut JsFormatter) -> FormatResult<()> {
        let JsExtendsClauseFields {
            extends_token,
            super_class,
            type_arguments,
        } = node.as_fields();

        let super_class = super_class?;

        let format_super = format_with(|f| {
            let content =
                format_with(|f| write!(f, [super_class.format(), type_arguments.format()]));

            let has_trailing_comments = if let Some(type_arguments) = &type_arguments {
                type_arguments.syntax().has_trailing_comments()
            } else {
                super_class.syntax().has_trailing_comments()
            };

            if node
                .syntax()
                .parent()
                .and_then(|node| resolve_parent(&node))
                .map_or(false, |p| p.kind() == JS_ASSIGNMENT_EXPRESSION)
            {
                if super_class.syntax().has_leading_comments() || has_trailing_comments {
                    write!(
                        f,
                        [format_parenthesize(
                            super_class.syntax().first_token().as_ref(),
                            &content,
                            super_class.syntax().last_token().as_ref()
                        )]
                    )
                } else {
                    let content = content.memoized();
                    write!(
                        f,
                        [
                            if_group_breaks(&format_args![
                                // Format_inserted is fine here because it is known that super has no comments
                                format_inserted(JsSyntaxKind::L_PAREN),
                                &soft_block_indent(&content),
                                format_inserted(JsSyntaxKind::R_PAREN),
                            ]),
                            if_group_fits_on_line(&content)
                        ]
                    )
                }
            } else {
                content.fmt(f)
            }
        });

        write![f, [extends_token.format(), space(), group(&format_super)]]
    }
}
