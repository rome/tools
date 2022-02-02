use crate::formatter_traits::FormatTokenAndNode;
use crate::ts::directives::format_directives_list;
use crate::ts::root::format_interpreter;
use crate::{
    format_elements, hard_line_break, FormatElement, FormatResult, Formatter, ToFormatElement,
};
use rslint_parser::ast::JsScript;

impl ToFormatElement for JsScript {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let mut elements = vec![];
        elements.push(format_interpreter(self.interpreter_token(), formatter)?);
        elements.push(format_directives_list(self.directives(), formatter));
        elements.push(formatter.format_list(self.statements()));
        elements.push(self.eof_token().format(formatter)?);

        Ok(format_elements![
            concat_elements(elements),
            hard_line_break()
        ])
    }
}
