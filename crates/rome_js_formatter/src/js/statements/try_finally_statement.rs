use crate::prelude::*;

use crate::FormatNodeFields;
use rome_js_syntax::JsTryFinallyStatement;
use rome_js_syntax::JsTryFinallyStatementFields;

impl FormatNodeFields<JsTryFinallyStatement> for FormatNodeRule<JsTryFinallyStatement> {
    fn format_fields(
        node: &JsTryFinallyStatement,
        formatter: &Formatter<JsFormatOptions>,
    ) -> FormatResult<FormatElement> {
        let JsTryFinallyStatementFields {
            try_token,
            body,
            catch_clause,
            finally_clause,
        } = node.as_fields();

        formatted![
            formatter,
            [
                try_token.format(),
                space_token(),
                body.format(),
                catch_clause
                    .format()
                    .with_or_empty(|catch_clause| formatted![
                        formatter,
                        [space_token(), catch_clause]
                    ]),
                space_token(),
                finally_clause.format()
            ]
        ]
    }
}
