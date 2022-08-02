use crate::prelude::*;
use rome_formatter::write;

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

        write!(
            f,
            [FormatWithSemicolon::new(
                &expression.format(),
                semicolon_token.as_ref()
            )]
        )
    }
}
