use crate::prelude::*;
use rome_formatter::{write, CstFormatContext};

use crate::utils::FormatWithSemicolon;

use rome_js_syntax::JsExpressionStatement;
use rome_js_syntax::JsExpressionStatementFields;

#[derive(Debug, Clone, Default)]
pub struct FormatJsExpressionStatement;

impl FormatNodeRule<JsExpressionStatement> for FormatJsExpressionStatement {
    fn fmt_fields(&self, node: &JsExpressionStatement, f: &mut JsFormatter) -> FormatResult<()> {
        let JsExpressionStatementFields {
            expression,
            semicolon_token,
        } = node.as_fields();

        let has_dangling_comments = f.context().comments().has_dangling_comments(node.syntax());

        write!(
            f,
            [FormatWithSemicolon::new(
                &expression.format(),
                semicolon_token.as_ref()
            )]
        )?;

        if has_dangling_comments {
            write!(f, [space(), format_dangling_comments(node.syntax())])?;
        }

        Ok(())
    }

    fn formats_dangling_comments(&self) -> bool {
        true
    }
}
