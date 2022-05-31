use crate::prelude::*;
use crate::utils::format_with_semicolon;

use crate::FormatNodeFields;
use rome_js_syntax::JsVariableStatement;
use rome_js_syntax::JsVariableStatementFields;

impl FormatNodeFields<JsVariableStatement> for FormatNodeRule<JsVariableStatement> {
    fn format_fields(
        node: &JsVariableStatement,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let JsVariableStatementFields {
            declaration,
            semicolon_token,
        } = node.as_fields();

        format_with_semicolon(
            formatter,
            formatted![formatter, [declaration.format()]]?,
            semicolon_token,
        )
    }
}
