mod call_expression;
use crate::formatter_traits::{FormatOptionalTokenAndNode, FormatTokenAndNode};
use crate::{
    empty_element, format_elements, hard_group_elements, hard_line_break, space_token,
    FormatElement, FormatResult, Formatter,
};
pub use call_expression::format_call_expression;
use rslint_parser::ast::{JsAnyStatement, JsInitializerClause};
use rslint_parser::{SyntaxNode, SyntaxNodeExt, SyntaxToken};

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

/// Returns true if this node contains "printable" trivias: comments
/// or empty lines (2 consecutive newlines only separated by whitespace)
pub(crate) fn has_formatter_trivia(node: &SyntaxNode) -> bool {
    let mut line_count = 0;

    for token in node.tokens() {
        for trivia in token.leading_trivia().pieces() {
            if trivia.as_comments().is_some() {
                return true;
            } else if trivia.as_newline().is_some() {
                line_count += 1;
                if line_count >= 2 {
                    return true;
                }
            }
        }

        // This is where the token would be,
        // reset the consecutive newline counter
        line_count = 0;

        for trivia in token.trailing_trivia().pieces() {
            if trivia.as_comments().is_some() {
                return true;
            } else if trivia.as_newline().is_some() {
                line_count += 1;
                if line_count >= 2 {
                    return true;
                }
            }
        }
    }

    false
}

/// Format an element with a single line head and a body that might
/// be either a block or a single statement
///
/// This will place the head element inside a [hard_group_elements], but
/// the body will broken out of flat printing if its a single statement
pub(crate) fn format_head_body_statement(
    formatter: &Formatter,
    head: FormatElement,
    body: JsAnyStatement,
) -> FormatResult<FormatElement> {
    if matches!(body, JsAnyStatement::JsBlockStatement(_)) {
        Ok(hard_group_elements(format_elements![
            head,
            body.format(formatter)?
        ]))
    } else {
        Ok(format_elements![
            hard_group_elements(head),
            body.format(formatter)?
        ])
    }
}
