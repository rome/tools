use crate::utils::format_with_semicolon;
use crate::{formatted, Format, FormatElement, FormatNode, Formatter};
use rome_formatter::FormatResult;

use rome_js_syntax::JsDebuggerStatement;
use rome_js_syntax::JsDebuggerStatementFields;

impl FormatNode for JsDebuggerStatement {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsDebuggerStatementFields {
            debugger_token,
            semicolon_token,
        } = self.as_fields();

        format_with_semicolon(
            formatter,
            formatted![formatter, debugger_token.format(formatter)?]?,
            semicolon_token,
        )
    }
}
