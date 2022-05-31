use crate::prelude::*;
use rome_formatter::{format_args, write};

use crate::FormatNodeFields;
use rome_js_syntax::JsTryFinallyStatement;
use rome_js_syntax::JsTryFinallyStatementFields;

impl FormatNodeFields<JsTryFinallyStatement> for FormatNodeRule<JsTryFinallyStatement> {
    fn format_fields(node: &JsTryFinallyStatement, f: &mut JsFormatter) -> FormatResult<()> {
        let JsTryFinallyStatementFields {
            try_token,
            body,
            catch_clause,
            finally_clause,
        } = node.as_fields();

        write![
            f,
            [
                try_token.format(),
                space_token(),
                body.format(),
                catch_clause
                    .format()
                    .with_or_empty(|catch_clause, f| write![f, [space_token(), catch_clause]]),
                space_token(),
                finally_clause.format()
            ]
        ]
    }
}
