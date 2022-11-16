use crate::prelude::*;
use crate::utils::{node_has_leading_newline, FormatWithSemicolon};
use rome_formatter::write;

use rome_js_syntax::JsExportNamedFromClause;
use rome_js_syntax::JsExportNamedFromClauseFields;
use rome_rowan::{AstNode, AstSeparatedElement};

#[derive(Debug, Clone, Default)]
pub struct FormatJsExportNamedFromClause;

impl FormatNodeRule<JsExportNamedFromClause> for FormatJsExportNamedFromClause {
    fn fmt_fields(&self, node: &JsExportNamedFromClause, f: &mut JsFormatter) -> FormatResult<()> {
        let JsExportNamedFromClauseFields {
            type_token,
            l_curly_token,
            specifiers,
            r_curly_token,
            from_token,
            source,
            assertion,
            semicolon_token,
        } = node.as_fields();

        let content = format_with(|f| {
            if let Some(type_token) = &type_token {
                write!(f, [type_token.format(), space()])?;
            }

            write!(f, [l_curly_token.format(),])?;

            match specifiers.elements().next() {
                Some(AstSeparatedElement {
                    node: Ok(node),
                    trailing_separator: Ok(separator),
                }) if specifiers.len() == 1 && !f.comments().has_comments(node.syntax()) => {
                    write!(f, [space(), node.format()])?;

                    if let Some(separator) = separator {
                        write!(f, [format_removed(&separator)])?;
                    }

                    write!(f, [space()])?;
                }
                _ => {
                    if node_has_leading_newline(specifiers.syntax()) {
                        write!(f, [block_indent(&specifiers.format()),])?;
                    } else {
                        write!(
                            f,
                            [group(&soft_space_or_block_indent(&specifiers.format())),]
                        )?;
                    };
                }
            }

            write![
                f,
                [
                    r_curly_token.format(),
                    space(),
                    from_token.format(),
                    space(),
                    source.format(),
                ]
            ]?;

            if let Some(assertion) = &assertion {
                write!(f, [space(), assertion.format()])?;
            }

            Ok(())
        });

        write!(
            f,
            [FormatWithSemicolon::new(&content, semicolon_token.as_ref())]
        )
    }
}
