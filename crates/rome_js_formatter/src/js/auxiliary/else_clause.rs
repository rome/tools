use crate::prelude::*;

use crate::utils::FormatStatementBody;
use rome_formatter::write;
use rome_js_syntax::JsElseClause;
use rome_js_syntax::JsElseClauseFields;

#[derive(Debug, Clone, Default)]
pub struct FormatJsElseClause;

impl FormatNodeRule<JsElseClause> for FormatJsElseClause {
    fn fmt_fields(&self, node: &JsElseClause, f: &mut JsFormatter) -> FormatResult<()> {
        use rome_js_syntax::JsAnyStatement::*;

        let JsElseClauseFields {
            else_token,
            alternate,
        } = node.as_fields();

        let alternate = alternate?;

        write!(
            f,
            [
                else_token.format(),
                group(
                    &FormatStatementBody::new(&alternate)
                        .with_forced_space(matches!(alternate, JsIfStatement(_)))
                )
            ]
        )
    }
}
