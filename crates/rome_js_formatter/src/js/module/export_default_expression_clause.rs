use crate::prelude::*;
use rome_formatter::{format_args, write};

use crate::utils::FormatWithSemicolon;
use crate::FormatNodeFields;
use rome_js_syntax::JsExportDefaultExpressionClause;
use rome_js_syntax::JsExportDefaultExpressionClauseFields;

impl FormatNodeFields<JsExportDefaultExpressionClause>
    for FormatNodeRule<JsExportDefaultExpressionClause>
{
    fn fmt_fields(node: &JsExportDefaultExpressionClause, f: &mut JsFormatter) -> FormatResult<()> {
        let JsExportDefaultExpressionClauseFields {
            default_token,
            expression,
            semicolon_token,
        } = node.as_fields();

        write!(
            f,
            [FormatWithSemicolon::new(
                &format_args!(default_token.format(), space_token(), expression.format()),
                semicolon_token.as_ref()
            )]
        )
    }
}
