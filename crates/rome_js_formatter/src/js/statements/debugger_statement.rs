use crate::utils::format_with_semicolon;
use crate::{format_elements, Format, FormatElement, FormatNode, FormatResult, Formatter};

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
            format_elements![debugger_token.format(formatter)?],
            semicolon_token,
        )
    }
}
