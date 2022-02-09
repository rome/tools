use crate::formatter_traits::FormatOptionalTokenAndNode;
use crate::{format_elements, space_token, FormatElement, FormatResult, Formatter};
use rslint_parser::ast::JsInitializerClause;

/// Utility function to format the node [rslint_parser::ast::JsInitializerClause]
pub(crate) fn format_initializer_clause(
    formatter: &Formatter,
    initializer: Option<JsInitializerClause>,
) -> FormatResult<FormatElement> {
    initializer.format_with_or_empty(formatter, |initializer| {
        format_elements![space_token(), initializer]
    })
}
