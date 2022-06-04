use crate::prelude::*;
use rome_formatter::write;

use crate::utils::FormatInterpreterToken;
use crate::FormatNodeFields;
use rome_js_syntax::JsModule;
use rome_js_syntax::JsModuleFields;

impl FormatNodeFields<JsModule> for FormatNodeRule<JsModule> {
    fn fmt_fields(node: &JsModule, f: &mut JsFormatter) -> FormatResult<()> {
        let JsModuleFields {
            interpreter_token,
            directives,
            items,
            eof_token,
        } = node.as_fields();

        write![
            f,
            [
                FormatInterpreterToken::new(interpreter_token.as_ref()),
                directives.format()
            ]
        ]?;

        write!(
            f,
            [
                items.format(),
                format_replaced(&eof_token?, &empty_element()),
                hard_line_break()
            ]
        )
    }
}
