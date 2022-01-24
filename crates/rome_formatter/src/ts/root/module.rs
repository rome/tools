use crate::ts::directives::format_directives;
use crate::ts::root::format_module_item_list;
use crate::{
    format_elements, hard_line_break, FormatElement, FormatResult, Formatter, ToFormatElement,
};
use rslint_parser::ast::JsModule;
use rslint_parser::AstNodeList;

impl ToFormatElement for JsModule {
    fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let mut elements = vec![];

        if let Some(interpreter) = self.interpreter_token() {
            elements.push(formatter.format_token(&interpreter)?);
            elements.push(hard_line_break());
        }

        let directives = self.directives();
        if directives.len() > 0 {
            elements.push(format_directives(directives, formatter));
            elements.push(hard_line_break());
        }

        elements.push(format_module_item_list(self.items(), formatter));

        elements.push(formatter.format_token(&self.eof_token()?)?);

        Ok(format_elements![
            concat_elements(elements),
            hard_line_break()
        ])
    }
}
