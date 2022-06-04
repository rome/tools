use crate::prelude::*;
use crate::FormatNodeFields;
use rome_formatter::write;
use rome_js_syntax::JsTryStatement;
use rome_js_syntax::JsTryStatementFields;

impl FormatNodeFields<JsTryStatement> for FormatNodeRule<JsTryStatement> {
    fn fmt_fields(node: &JsTryStatement, f: &mut JsFormatter) -> FormatResult<()> {
        let JsTryStatementFields {
            try_token,
            body,
            catch_clause,
        } = node.as_fields();

        write![
            f,
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
