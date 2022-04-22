use crate::utils::format_interpreter;
use crate::{format_elements, hard_line_break, Format, FormatElement, FormatNode, Formatter};
use rome_formatter::FormatResult;

use rome_js_syntax::JsScript;
use rome_js_syntax::JsScriptFields;

impl FormatNode for JsScript {
    fn format_fields(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let JsScriptFields {
            interpreter_token,
            directives,
            statements,
            eof_token,
        } = self.as_fields();

        Ok(format_elements![
            format_interpreter(interpreter_token, formatter)?,
            directives.format(formatter)?,
            formatter.format_list(statements),
            eof_token.format(formatter)?,
            hard_line_break()
        ])
    }
}
