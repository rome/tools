use crate::formatter_traits::FormatOptionalTokenAndNode;
use crate::{format_elements, hard_line_break, FormatElement, FormatResult, Formatter};
use rslint_parser::SyntaxToken;

mod any_module_item;
mod module;
mod script;

pub fn format_interpreter(
    interpreter: Option<SyntaxToken>,
    formatter: &Formatter,
) -> FormatResult<FormatElement> {
    interpreter.format_with_or_empty(formatter, |element| {
        format_elements![element, hard_line_break()]
    })
}
