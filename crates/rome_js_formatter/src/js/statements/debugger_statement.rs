use crate::prelude::*;
use crate::utils::format_with_semicolon;

use crate::FormatNodeFields;
use rome_js_syntax::JsDebuggerStatement;
use rome_js_syntax::JsDebuggerStatementFields;

impl FormatNodeFields<JsDebuggerStatement> for FormatNodeRule<JsDebuggerStatement> {
    fn format_fields(
        node: &JsDebuggerStatement,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let JsDebuggerStatementFields {
            debugger_token,
            semicolon_token,
        } = node.as_fields();

        format_with_semicolon(
            formatter,
            formatted![formatter, [debugger_token.format()]]?,
            semicolon_token,
        )
    }
}
