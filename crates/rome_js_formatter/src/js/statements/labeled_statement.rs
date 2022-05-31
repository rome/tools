use crate::prelude::*;

use crate::FormatNodeFields;
use rome_js_syntax::JsLabeledStatementFields;
use rome_js_syntax::{JsAnyStatement, JsLabeledStatement};

impl FormatNodeFields<JsLabeledStatement> for FormatNodeRule<JsLabeledStatement> {
    fn format_fields(
        node: &JsLabeledStatement,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let JsLabeledStatementFields {
            label_token,
            colon_token,
            body,
        } = node.as_fields();

        let label = label_token.format();
        let colon = colon_token.format();

        let body = body?;
        if matches!(body, JsAnyStatement::JsEmptyStatement(_)) {
            // If the body is an empty statement, force semicolon insertion
            let statement = body.format();
            formatted![formatter, [label, colon, statement, token(";")]]
        } else {
            let statement = body.format();
            formatted![formatter, [label, colon, space_token(), statement]]
        }
    }
}
