mod call_expression;
use crate::formatter_traits::{FormatOptionalTokenAndNode, FormatTokenAndNode};
use crate::{
    empty_element, format_elements, hard_group_elements, hard_line_break, space_token,
    FormatElement, FormatResult, Formatter,
};
pub use call_expression::format_call_expression;
use rslint_parser::ast::{JsAnyStatement, JsInitializerClause};
use rslint_parser::{JsSyntaxKind, SyntaxNode, SyntaxNodeExt, SyntaxToken};

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

#[derive(Debug, PartialEq, Eq)]
struct Suppression<'a> {
    categories: Vec<(&'a str, Option<&'a str>)>,
    reason: &'a str,
}

fn parse_suppression_comment(comment: &str) -> impl Iterator<Item = Suppression> {
    let (head, mut comment) = comment.split_at(2);
    let is_block_comment = match head {
        "//" => false,
        "/*" => {
            comment = comment.strip_suffix("*/").unwrap();
            true
        }
        _ => panic!(),
    };

    comment.lines().filter_map(move |line| {
        // Eat start of line whitespace
        let mut line = line.trim_start();

        // If we're in a block comment eat stars, then whitespace again
        if is_block_comment {
            line = line.trim_start_matches('*').trim_start()
        }

        // TODO: While we do want to detect these, it should
        // only be enabled in a special "migration" mode where
        // they get rewritten as rome-ignore
        if line.trim_end() == "prettier-ignore" {
            return Some(Suppression {
                categories: vec![("format", None)],
                reason: "",
            });
        }

        // Check for the rome-ignore token or skip the line entirely
        line = line.strip_prefix("rome-ignore")?.trim_start();

        let mut categories = Vec::new();

        loop {
            // Find either a colon or opening parenthesis
            let separator = line.find([':', '('])?;

            let (category, rest) = line.split_at(separator);
            let category = category.trim_end();

            // Skip over and match the separator
            let (separator, rest) = rest.split_at(1);

            if separator == ":" {
                if !category.is_empty() {
                    categories.push((category, None));
                }

                line = rest.trim_start();
                break;
            }

            let paren = rest.find(')')?;

            let (value, rest) = rest.split_at(paren);
            let value = value.trim();

            categories.push((category, Some(value)));

            line = rest.strip_prefix(')').unwrap().trim_start();
        }

        let reason = line.trim_end();
        Some(Suppression { categories, reason })
    })
}

pub(crate) fn has_formatter_suppressions(node: &SyntaxNode) -> bool {
    // Lists cannot have a suppression comment attached, it must
    // belong to either the entire parent node or one of the children
    match node.kind() {
        // Files are a special kind of list
        JsSyntaxKind::JS_MODULE | JsSyntaxKind::JS_SCRIPT => return false,
        kind if kind.is_list() => return false,
        _ => {}
    }

    let first_token = match node.first_token() {
        Some(token) => token,
        None => return false,
    };

    first_token
        .leading_trivia()
        .pieces()
        .filter_map(|trivia| trivia.as_comments())
        .any(|comment| {
            for suppression in parse_suppression_comment(comment.text()) {
                for (category, _) in suppression.categories {
                    if category == "format" {
                        return true;
                    }
                }
            }

            false
        })
}

#[cfg(test)]
mod tests {
    use super::{parse_suppression_comment, Suppression};

    #[test]
    fn parse_simple_suppression() {
        assert_eq!(
            parse_suppression_comment("// rome-ignore parse: explanation1").collect::<Vec<_>>(),
            vec![Suppression {
                categories: vec![("parse", None)],
                reason: "explanation1"
            }],
        );

        assert_eq!(
            parse_suppression_comment("/** rome-ignore parse: explanation2 */").collect::<Vec<_>>(),
            vec![Suppression {
                categories: vec![("parse", None)],
                reason: "explanation2"
            }],
        );

        assert_eq!(
            parse_suppression_comment(
                "/**
				  * rome-ignore parse: explanation3
				  */"
            )
            .collect::<Vec<_>>(),
            vec![Suppression {
                categories: vec![("parse", None)],
                reason: "explanation3"
            }],
        );

        assert_eq!(
            parse_suppression_comment(
                "/**
				  * hello
				  * rome-ignore parse: explanation4
				  */"
            )
            .collect::<Vec<_>>(),
            vec![Suppression {
                categories: vec![("parse", None)],
                reason: "explanation4"
            }],
        );
    }

    #[test]
    fn parse_multiple_suppression() {
        assert_eq!(
            parse_suppression_comment("// rome-ignore parse(foo) parse(dog): explanation")
                .collect::<Vec<_>>(),
            vec![Suppression {
                categories: vec![("parse", Some("foo")), ("parse", Some("dog"))],
                reason: "explanation"
            }],
        );

        assert_eq!(
            parse_suppression_comment("/** rome-ignore parse(bar) parse(cat): explanation */")
                .collect::<Vec<_>>(),
            vec![Suppression {
                categories: vec![("parse", Some("bar")), ("parse", Some("cat"))],
                reason: "explanation"
            }],
        );

        assert_eq!(
            parse_suppression_comment(
                "/**
				  * rome-ignore parse(yes) parse(frog): explanation
				  */"
            )
            .collect::<Vec<_>>(),
            vec![Suppression {
                categories: vec![("parse", Some("yes")), ("parse", Some("frog"))],
                reason: "explanation"
            }],
        );

        assert_eq!(
            parse_suppression_comment(
                "/**
				  * hello
				  * rome-ignore parse(wow) parse(fish): explanation
				  */"
            )
            .collect::<Vec<_>>(),
            vec![Suppression {
                categories: vec![("parse", Some("wow")), ("parse", Some("fish"))],
                reason: "explanation"
            }],
        );
    }
}
