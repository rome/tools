use crate::{
    empty_element, format_elements, hard_line_break, FormatElement, FormatResult, Formatter,
};
use rslint_parser::SyntaxToken;

mod any_module_item;
mod module;
mod script;

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
