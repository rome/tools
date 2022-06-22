use crate::prelude::*;
use crate::utils::FormatInterpreterToken;
use rome_formatter::write;

use rome_js_syntax::JsScript;
use rome_js_syntax::JsScriptFields;

#[derive(Debug, Clone, Default)]
pub struct FormatJsScript;

impl FormatNodeRule<JsScript> for FormatJsScript {
    fn fmt_fields(&self, node: &JsScript, f: &mut JsFormatter) -> FormatResult<()> {
        let JsScriptFields {
            interpreter_token,
            directives,
            statements,
            eof_token,
        } = node.as_fields();

        write![
            f,
            [
                FormatInterpreterToken::new(interpreter_token.as_ref()),
                directives.format(),
            ]
        ]?;

        write![
            f,
            [
                statements.format(),
                format_removed(&eof_token?),
                hard_line_break()
            ]
        ]
    }
}
