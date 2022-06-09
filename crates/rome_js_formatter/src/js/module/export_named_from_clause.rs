use crate::prelude::*;
use crate::utils::{has_leading_newline, FormatWithSemicolon};
use rome_formatter::write;

use crate::FormatNodeFields;
use rome_js_syntax::JsExportNamedFromClause;
use rome_js_syntax::JsExportNamedFromClauseFields;
use rome_rowan::AstNode;

impl FormatNodeFields<JsExportNamedFromClause> for FormatNodeRule<JsExportNamedFromClause> {
    fn fmt_fields(node: &JsExportNamedFromClause, f: &mut JsFormatter) -> FormatResult<()> {
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
                write!(f, [type_token.format(), space_token()])?;
            }

            if has_leading_newline(specifiers.syntax()) {
                write!(
                    f,
                    [format_delimited(
                        l_curly_token.as_ref()?,
                        &specifiers.format(),
                        r_curly_token.as_ref()?,
                    )
                    .block_indent()]
                )?;
            } else {
                write!(
                    f,
                    [format_delimited(
                        l_curly_token.as_ref()?,
                        &specifiers.format(),
                        r_curly_token.as_ref()?,
                    )
                    .soft_block_spaces()]
                )?;
            };

            write![
                f,
                [
                    space_token(),
                    from_token.format(),
                    space_token(),
                    source.format(),
                ]
            ]?;

            if let Some(assertion) = &assertion {
                write!(f, [space_token(), assertion.format()])?;
            }

            Ok(())
        });

        write!(
            f,
            [FormatWithSemicolon::new(&content, semicolon_token.as_ref())]
        )
    }
}
