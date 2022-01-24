use crate::ts::directives::format_directives;
use crate::{
    empty_element, format_elements, hard_line_break, FormatElement, FormatResult, Formatter,
};
use rslint_parser::ast::JsDirectiveList;
use rslint_parser::AstNodeList;
use rslint_parser::SyntaxToken;

mod any_module_item;
mod module;
mod script;

pub fn format_directives_list(directives: JsDirectiveList, formatter: &Formatter) -> FormatElement {
    if directives.len() > 0 {
        format_elements![format_directives(directives, formatter), hard_line_break()]
    } else {
        empty_element()
    }
}

pub fn format_interpreter(
    interpreter: Option<SyntaxToken>,
    formatter: &Formatter,
) -> FormatResult<FormatElement> {
    let result = if let Some(interpreter) = interpreter {
        format_elements![formatter.format_token(&interpreter)?, hard_line_break()]
    } else {
        empty_element()
    };
    Ok(result)
}
