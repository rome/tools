use crate::prelude::*;
use crate::utils::format_interpreter;

use rome_js_syntax::JsModule;
use rome_js_syntax::JsModuleFields;

impl FormatNode for JsModule {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsModuleFields {
            interpreter_token,
            directives,
            items,
            eof_token,
        } = self.as_fields();

        formatted![
            formatter,
            format_interpreter(interpreter_token, formatter)?,
            directives.format(formatter)?,
            formatter.format_list(items),
            eof_token.format(formatter)?,
            hard_line_break()
        ]
    }
}
