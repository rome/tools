use crate::{
    empty_element, format_elements, formatter_traits::FormatOptionalTokenAndNode, hard_line_break,
    FormatElement, FormatResult, Formatter,
};
use rslint_parser::SyntaxToken;

pub(crate) fn format_interpreter(
    interpreter: Option<SyntaxToken>,
    formatter: &Formatter,
) -> FormatResult<FormatElement> {
    interpreter.format_with_or(
        formatter,
        |interpreter| format_elements![interpreter, hard_line_break()],
        empty_element,
    )
}
