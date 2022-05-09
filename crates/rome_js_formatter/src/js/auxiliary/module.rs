use crate::prelude::*;
use crate::utils::format_interpreter;

use crate::FormatNodeFields;
use rome_js_syntax::JsModule;
use rome_js_syntax::JsModuleFields;

impl FormatNodeFields<JsModule> for FormatNodeRule<JsModule> {
    fn format_fields(node: &JsModule, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsModuleFields {
            interpreter_token,
            directives,
            items,
            eof_token,
        } = node.as_fields();

        formatted![
            formatter,
            format_interpreter(interpreter_token, formatter)?,
            directives.format(),
            formatter.format_list(&items),
            eof_token.format(),
            hard_line_break()
        ]
    }
}
