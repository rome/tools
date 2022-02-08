use crate::{
    format_elements, formatter_traits::FormatTokenAndNode, hard_line_break,
    interpreter::format_interpreter, FormatElement, FormatResult, Formatter, ToFormatElement,
};

use rslint_parser::ast::JsModule;

impl ToFormatElement for JsModule {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        Ok(format_elements![
            format_interpreter(self.interpreter_token(), formatter)?,
            self.directives().format(formatter)?,
            formatter.format_list(self.items()),
            self.eof_token().format(formatter)?,
            hard_line_break()
        ])
    }
}
