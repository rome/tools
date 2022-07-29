use crate::prelude::*;

use rome_formatter::write;
use rome_js_syntax::JsInitializerClause;
use rome_js_syntax::JsInitializerClauseFields;

#[derive(Debug, Clone, Default)]
pub struct FormatJsInitializerClause;

impl FormatNodeRule<JsInitializerClause> for FormatJsInitializerClause {
    fn fmt_fields(&self, node: &JsInitializerClause, f: &mut JsFormatter) -> FormatResult<()> {
        let JsInitializerClauseFields {
            eq_token,
            expression,
        } = node.as_fields();

        write![f, [eq_token.format(), space(), expression.format()]]
    }
}
