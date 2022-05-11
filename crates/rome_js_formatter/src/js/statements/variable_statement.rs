use crate::prelude::*;
use crate::utils::format_with_semicolon;

use rome_js_syntax::JsVariableStatement;
use rome_js_syntax::JsVariableStatementFields;

impl FormatNode for JsVariableStatement {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsVariableStatementFields {
            declaration,
            semicolon_token,
        } = self.as_fields();

        format_with_semicolon(formatter, declaration.format(formatter)?, semicolon_token)
    }
}
