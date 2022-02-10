use crate::formatter_traits::FormatOptionalTokenAndNode;
use crate::{
    empty_element, format_elements, hard_line_break, space_token, FormatElement, FormatResult,
    Formatter,
};
use rslint_parser::ast::JsInitializerClause;
use rslint_parser::SyntaxToken;

/// Utility function to format the separators of the nodes that belong to the unions
/// of [rslint_parser::ast::TsAnyTypeMember].
///
/// We can have two kind of separators: `,`, `;` or ASI.
/// Because of how the grammar crafts the nodes, the parent will add the separator to the node.
/// So here, we create - on purpose - an empty node.
pub(crate) fn format_type_member_separator(
    separator_token: Option<SyntaxToken>,
    formatter: &Formatter,
) -> FormatResult<FormatElement> {
    if let Some(separator) = separator_token {
        formatter.format_replaced(&separator, empty_element())
    } else {
        Ok(empty_element())
    }
}

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
