use crate::ts::root::{format_directives_list, format_interpreter};
use crate::{
    format_elements, hard_line_break, FormatElement, FormatResult, Formatter, ToFormatElement,
};
use rslint_parser::ast::JsModule;

impl ToFormatElement for JsModule {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let mut elements = vec![];

        elements.push(format_interpreter(self.interpreter_token(), formatter)?);
        elements.push(format_directives_list(self.directives(), formatter));
        elements.push(formatter.format_list(self.items()));
        elements.push(formatter.format_token(&self.eof_token()?)?);

        Ok(format_elements![
            concat_elements(elements),
            hard_line_break()
        ])
    }
}
