use crate::prelude::*;
use crate::utils::format_with_semicolon;

use rome_js_syntax::JsExpressionStatement;
use rome_js_syntax::JsExpressionStatementFields;

impl FormatNode for JsExpressionStatement {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsExpressionStatementFields {
            expression,
            semicolon_token,
        } = self.as_fields();

        format_with_semicolon(formatter, expression.format(formatter)?, semicolon_token)
    }
}
