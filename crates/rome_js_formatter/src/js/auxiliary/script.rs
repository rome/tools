use crate::prelude::*;
use crate::utils::format_interpreter;

use crate::FormatNodeFields;
use rome_js_syntax::JsScript;
use rome_js_syntax::JsScriptFields;

impl FormatNodeFields<JsScript> for FormatNodeRule<JsScript> {
    fn format_fields(
        node: &JsScript,
        formatter: &Formatter<JsFormatOptions>,
    ) -> FormatResult<FormatElement> {
        let JsScriptFields {
            interpreter_token,
            directives,
            statements,
            eof_token,
        } = node.as_fields();

        formatted![
            formatter,
            [
                format_interpreter(interpreter_token, formatter)?,
                directives.format(),
                formatter.format_list_with_hard_line(&statements),
                formatter.format_replaced(&eof_token?, empty_element()),
                hard_line_break()
            ]
        ]
    }
}
