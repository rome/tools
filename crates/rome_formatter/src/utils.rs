use crate::formatter_traits::FormatOptionalTokenAndNode;
use crate::{
    empty_element, format_elements, hard_line_break, space_token, FormatElement, FormatResult,
    Formatter,
};
use rslint_parser::ast::JsInitializerClause;
use rslint_parser::SyntaxToken;

/// Utility function to format the node [rslint_parser::ast::JsInitializerClause]
pub(crate) fn format_initializer_clause(
    formatter: &Formatter,
    initializer: Option<JsInitializerClause>,
) -> FormatResult<FormatElement> {
    initializer.format_with_or_empty(formatter, |initializer| {
        format_elements![space_token(), initializer]
    })
}

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
