use crate::utils::format_interpreter;
use crate::{
    format_elements, formatter_traits::FormatTokenAndNode, hard_line_break, FormatElement,
    FormatResult, Formatter, ToFormatElement,
};

use rome_js_syntax::JsScript;
use rome_js_syntax::JsScriptFields;

impl ToFormatElement for JsScript {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
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
