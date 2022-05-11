use crate::prelude::*;

use rome_js_syntax::JsLabeledStatementFields;
use rome_js_syntax::{JsAnyStatement, JsLabeledStatement};

impl FormatNode for JsLabeledStatement {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsLabeledStatementFields {
            label_token,
            colon_token,
            body,
        } = self.as_fields();

        let label = label_token.format(formatter)?;
        let colon = colon_token.format(formatter)?;

        let body = body?;
        if matches!(body, JsAnyStatement::JsEmptyStatement(_)) {
            // If the body is an empty statement, force semicolon insertion
            let statement = body.format(formatter)?;
            formatted![formatter, label, colon, statement, token(";")]
        } else {
            let statement = body.format(formatter)?;
            formatted![formatter, label, colon, space_token(), statement]
        }
    }
}
