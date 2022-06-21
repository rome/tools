use crate::prelude::*;
use rome_formatter::write;

use crate::utils::FormatInterpreterToken;

use rome_js_syntax::JsModule;
use rome_js_syntax::JsModuleFields;

#[derive(Debug, Clone, Default)]
pub struct FormatJsModule;

impl FormatNodeRule<JsModule> for FormatJsModule {
    fn fmt_fields(&self, node: &JsModule, f: &mut JsFormatter) -> FormatResult<()> {
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
                format_removed(&eof_token?),
                hard_line_break()
            ]
        )
    }
}
