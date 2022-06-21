use crate::prelude::*;
use rome_formatter::write;

use rome_js_syntax::JsTryFinallyStatement;
use rome_js_syntax::JsTryFinallyStatementFields;

#[derive(Debug, Clone, Default)]
pub struct FormatJsTryFinallyStatement;

impl FormatNodeRule<JsTryFinallyStatement> for FormatJsTryFinallyStatement {
    fn fmt_fields(&self, node: &JsTryFinallyStatement, f: &mut JsFormatter) -> FormatResult<()> {
        let JsTryFinallyStatementFields {
            try_token,
            body,
            catch_clause,
            finally_clause,
        } = node.as_fields();

        write![f, [try_token.format(), space_token(), body.format(),]]?;

        if let Some(catch_clause) = catch_clause {
            write!(f, [space_token(), catch_clause.format()])?;
        }

        write!(f, [space_token(), finally_clause.format()])
    }
}
