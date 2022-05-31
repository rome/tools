use crate::prelude::*;

use crate::FormatNodeFields;
use rome_js_syntax::JsTryStatement;
use rome_js_syntax::JsTryStatementFields;

impl FormatNodeFields<JsTryStatement> for FormatNodeRule<JsTryStatement> {
    fn format_fields(
        node: &JsTryStatement,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let JsTryStatementFields {
            try_token,
            body,
            catch_clause,
        } = node.as_fields();

        formatted![
            formatter,
            [
                try_token.format(),
                space_token(),
                body.format(),
                space_token(),
                catch_clause.format(),
            ]
        ]
    }
}
