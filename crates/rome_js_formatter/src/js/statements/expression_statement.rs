use crate::prelude::*;
use rome_formatter::{format_args, write};

use crate::utils::FormatWithSemicolon;
use crate::FormatNodeFields;
use rome_js_syntax::JsExpressionStatement;
use rome_js_syntax::JsExpressionStatementFields;

impl FormatNodeFields<JsExpressionStatement> for FormatNodeRule<JsExpressionStatement> {
    fn fmt_fields(node: &JsExpressionStatement, f: &mut JsFormatter) -> FormatResult<()> {
        let JsExpressionStatementFields {
            expression,
            semicolon_token,
        } = node.as_fields();

        write!(
            f,
            [FormatWithSemicolon::new(
                &format_args!(expression.format()),
                semicolon_token.as_ref()
            )]
        )
    }
}
