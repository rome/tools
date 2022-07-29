use crate::prelude::*;
use rome_formatter::{format_args, write};

use crate::utils::FormatWithSemicolon;

use rome_js_syntax::JsExportDefaultExpressionClause;
use rome_js_syntax::JsExportDefaultExpressionClauseFields;

#[derive(Debug, Clone, Default)]
pub struct FormatJsExportDefaultExpressionClause;

impl FormatNodeRule<JsExportDefaultExpressionClause> for FormatJsExportDefaultExpressionClause {
    fn fmt_fields(
        &self,
        node: &JsExportDefaultExpressionClause,
        f: &mut JsFormatter,
    ) -> FormatResult<()> {
        let JsExportDefaultExpressionClauseFields {
            default_token,
            expression,
            semicolon_token,
        } = node.as_fields();

        write!(
            f,
            [FormatWithSemicolon::new(
                &format_args!(default_token.format(), space(), expression.format()),
                semicolon_token.as_ref()
            )]
        )
    }
}
