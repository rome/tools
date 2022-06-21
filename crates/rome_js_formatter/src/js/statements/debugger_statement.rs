use crate::prelude::*;
use rome_formatter::{format_args, write};

use crate::utils::FormatWithSemicolon;

use rome_js_syntax::JsDebuggerStatement;
use rome_js_syntax::JsDebuggerStatementFields;

#[derive(Debug, Clone, Default)]
pub struct FormatJsDebuggerStatement;

impl FormatNodeRule<JsDebuggerStatement> for FormatJsDebuggerStatement {
    fn fmt_fields(&self, node: &JsDebuggerStatement, f: &mut JsFormatter) -> FormatResult<()> {
        let JsDebuggerStatementFields {
            debugger_token,
            semicolon_token,
        } = node.as_fields();

        write!(
            f,
            [FormatWithSemicolon::new(
                &format_args!(debugger_token.format()),
                semicolon_token.as_ref()
            ),]
        )
    }
}
