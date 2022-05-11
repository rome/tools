pub(crate) mod array;
mod binary_like_expression;
mod format_conditional;
mod simple;
pub mod string_utils;

mod member_chain;
#[cfg(test)]
mod quickcheck_utils;

use crate::prelude::*;
pub(crate) use binary_like_expression::{format_binary_like_expression, JsAnyBinaryLikeExpression};
pub(crate) use format_conditional::{format_conditional, Conditional};
pub(crate) use member_chain::format_call_expression;
use rome_formatter::{normalize_newlines, QuoteStyle};
use rome_js_syntax::{
    JsAnyExpression, JsAnyFunction, JsAnyRoot, JsAnyStatement, JsInitializerClause, JsLanguage,
    JsTemplateElement, JsTemplateElementFields, Modifiers, TsTemplateElement,
    TsTemplateElementFields, TsType,
};
use rome_js_syntax::{JsSyntaxKind, JsSyntaxNode, JsSyntaxToken};
use rome_rowan::{AstNode, AstNodeList};
use std::borrow::Cow;

pub(crate) use simple::*;

/// Utility function to format the separators of the nodes that belong to the unions
/// of [rome_js_syntax::TsAnyTypeMember].
///
/// We can have two kind of separators: `,`, `;` or ASI.
/// Because of how the grammar crafts the nodes, the parent will add the separator to the node.
/// So here, we create - on purpose - an empty node.
pub(crate) fn format_type_member_separator(
    separator_token: Option<JsSyntaxToken>,
    formatter: &Formatter,
) -> FormatElement {
    if let Some(separator) = separator_token {
        formatter.format_replaced(&separator, empty_element())
    } else {
        empty_element()
    }
}

/// Utility function to format the node [rome_js_syntax::JsInitializerClause]
pub(crate) fn format_initializer_clause(
    formatter: &Formatter,
    initializer: Option<JsInitializerClause>,
) -> FormatResult<FormatElement> {
    formatted![
        formatter,
        [initializer
            .format()
            .with_or_empty(|initializer| { formatted![formatter, [space_token(), initializer]] })]
    ]
}

pub(crate) fn format_interpreter(
    interpreter: Option<JsSyntaxToken>,
    formatter: &Formatter,
) -> FormatResult<FormatElement> {
    formatted![
        formatter,
        [interpreter.format().with_or(
            |interpreter| formatted![formatter, [interpreter, empty_line()]],
            empty_element,
        )]
    ]
}

/// Returns true if this node contains "printable" trivias: comments
/// or empty lines (2 consecutive newlines only separated by whitespace)
pub(crate) fn has_formatter_trivia(node: &JsSyntaxNode) -> bool {
    let mut line_count = 0;

    for token in node.descendants_tokens() {
        for trivia in token.leading_trivia().pieces() {
            if trivia.is_comments() {
                return true;
            } else if trivia.is_newline() {
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
            if trivia.is_comments() {
                return true;
            } else if trivia.is_newline() {
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
        Ok(hard_group_elements(formatted![
            formatter,
            [head, space_token(), body.format(),]
        ]?))
    } else if matches!(body, JsAnyStatement::JsEmptyStatement(_)) {
        // Force semicolon insertion if the body is empty
        formatted![
            formatter,
            [hard_group_elements(head), body.format(), token(";"),]
        ]
    } else {
        formatted![
            formatter,
            [hard_group_elements(head), space_token(), body.format(),]
        ]
    }
}

/// Single instance of a suppression comment, with the following syntax:
///
/// `// rome-ignore { <category> { (<value>) }? }+: <reason>`
///
/// The category broadly describes what feature is being suppressed (formatting,
/// linting, ...) with the value being and optional, category-specific name of
/// a specific element to disable (for instance a specific lint name). A single
/// suppression may specify one or more categories + values, for instance to
/// disable multiple lints at once
///
/// A suppression must specify a reason: this part has no semantic meaning but
/// is required to document why a particular feature is being disable for this
/// line (lint false-positive, specific formatting requirements, ...)
#[derive(Debug, PartialEq, Eq)]
struct Suppression<'a> {
    /// List of categories for this suppression
    ///
    /// Categories are pair of the category name +
    /// an optional category value
    categories: Vec<(&'a str, Option<&'a str>)>,
    /// Reason for this suppression comment to exist
    reason: &'a str,
}

const CATEGORY_FORMAT: &str = "format";

fn parse_suppression_comment(comment: &str) -> impl Iterator<Item = Suppression> {
    let (head, mut comment) = comment.split_at(2);
    let is_block_comment = match head {
        "//" => false,
        "/*" => {
            comment = comment
                .strip_suffix("*/")
                .expect("block comment with no closing token");
            true
        }
        token => panic!("comment with unknown opening token {token:?}"),
    };

    comment.lines().filter_map(move |line| {
        // Eat start of line whitespace
        let mut line = line.trim_start();

        // If we're in a block comment eat stars, then whitespace again
        if is_block_comment {
            line = line.trim_start_matches('*').trim_start()
        }

        // Check for the rome-ignore token or skip the line entirely
        line = line.strip_prefix("rome-ignore")?.trim_start();

        let mut categories = Vec::new();

        loop {
            // Find either a colon opening parenthesis or space
            let separator = line.find(|c: char| c == ':' || c == '(' || c.is_whitespace())?;

            let (category, rest) = line.split_at(separator);
            let category = category.trim_end();

            // Skip over and match the separator
            let (separator, rest) = rest.split_at(1);

            match separator {
                // Colon token: stop parsing categories
                ":" => {
                    if !category.is_empty() {
                        categories.push((category, None));
                    }

                    line = rest.trim_start();
                    break;
                }
                // Paren token: parse a category + value
                "(" => {
                    let paren = rest.find(')')?;

                    let (value, rest) = rest.split_at(paren);
                    let value = value.trim();

                    categories.push((category, Some(value)));

                    line = rest.strip_prefix(')').unwrap().trim_start();
                }
                // Whitespace: push a category without value
                _ => {
                    if !category.is_empty() {
                        categories.push((category, None));
                    }

                    line = rest.trim_start();
                }
            }
        }

        let reason = line.trim_end();
        Some(Suppression { categories, reason })
    })
}

pub(crate) fn has_formatter_suppressions(node: &JsSyntaxNode) -> bool {
    // Lists cannot have a suppression comment attached, it must
    // belong to either the entire parent node or one of the children
    let kind = node.kind();
    if JsAnyRoot::can_cast(kind) || kind.is_list() {
        return false;
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
            parse_suppression_comment(comment.text())
                .flat_map(|suppression| suppression.categories)
                .any(|category| category.0 == CATEGORY_FORMAT)
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

    #[test]
    fn parse_multiple_suppression_categories() {
        assert_eq!(
            parse_suppression_comment("// rome-ignore format lint: explanation")
                .collect::<Vec<_>>(),
            vec![Suppression {
                categories: vec![("format", None), ("lint", None)],
                reason: "explanation"
            }],
        );
    }
}

/// This function consumes a list of modifiers and applies a predictable sorting.
pub(crate) fn sort_modifiers_by_precedence<List, Node>(list: &List) -> Vec<Node>
where
    Node: AstNode<Language = JsLanguage>,
    List: AstNodeList<Language = JsLanguage, Node = Node>,
    Modifiers: for<'a> From<&'a Node>,
{
    let mut nodes_and_modifiers = list.iter().collect::<Vec<Node>>();

    nodes_and_modifiers.sort_unstable_by_key(|node| Modifiers::from(node));

    nodes_and_modifiers
}

/// Utility to format
pub(crate) fn format_template_chunk(
    chunk: JsSyntaxToken,
    formatter: &Formatter,
) -> FormatResult<FormatElement> {
    // Per https://tc39.es/ecma262/multipage/ecmascript-language-lexical-grammar.html#sec-static-semantics-trv:
    // In template literals, the '\r' and '\r\n' line terminators are normalized to '\n'
    Ok(formatter.format_replaced(
        &chunk,
        FormatElement::from(Token::from_syntax_token_cow_slice(
            normalize_newlines(chunk.text_trimmed(), ['\r']),
            &chunk,
            chunk.text_trimmed_range().start(),
        )),
    ))
}

/// Function to format template literals and template literal types
pub(crate) fn format_template_literal(
    literal: TemplateElement,
    formatter: &Formatter,
) -> FormatResult<FormatElement> {
    literal.into_format_element(formatter)
}

pub(crate) enum TemplateElement {
    Js(JsTemplateElement),
    Ts(TsTemplateElement),
}

impl TemplateElement {
    pub fn into_format_element(self, formatter: &Formatter) -> FormatResult<FormatElement> {
        let expression_is_plain = self.is_plain_expression()?;
        let has_comments = self.has_comments();
        let should_hard_group = expression_is_plain && !has_comments;

        let (dollar_curly_token, middle, r_curly_token) = match self {
            TemplateElement::Js(template_element) => {
                let JsTemplateElementFields {
                    dollar_curly_token,
                    expression,
                    r_curly_token,
                } = template_element.as_fields();

                let dollar_curly_token = dollar_curly_token?;
                let expression = formatted![formatter, [expression.format()]]?;
                let r_curly_token = r_curly_token?;

                (dollar_curly_token, expression, r_curly_token)
            }
            TemplateElement::Ts(template_element) => {
                let TsTemplateElementFields {
                    ty,
                    r_curly_token,
                    dollar_curly_token,
                } = template_element.as_fields();

                let dollar_curly_token = dollar_curly_token?;
                let ty = formatted![formatter, [ty.format()]]?;
                let r_curly_token = r_curly_token?;

                (dollar_curly_token, ty, r_curly_token)
            }
        };

        if should_hard_group {
            Ok(hard_group_elements(formatted![
                formatter,
                [dollar_curly_token.format(), middle, r_curly_token.format()]
            ]?))
        } else {
            formatter.format_delimited_soft_block_indent(
                &dollar_curly_token,
                middle,
                &r_curly_token,
            )
        }
    }

    /// We want to break the template element only when we have articulated expressions inside it.
    ///
    /// We a plain expression is when it's one of the following:
    /// - `loreum ${this.something} ipsum`
    /// - `loreum ${a.b.c} ipsum`
    /// - `loreum ${a} ipsum`
    fn is_plain_expression(&self) -> FormatResult<bool> {
        match self {
            TemplateElement::Js(template_element) => {
                let current_expression = template_element.expression()?;
                match current_expression {
                    JsAnyExpression::JsStaticMemberExpression(_)
                    | JsAnyExpression::JsComputedMemberExpression(_)
                    | JsAnyExpression::JsIdentifierExpression(_)
                    | JsAnyExpression::JsAnyLiteralExpression(_)
                    | JsAnyExpression::JsCallExpression(_) => Ok(true),

                    JsAnyExpression::JsParenthesizedExpression(expression) => {
                        // binary and logical expression have their own grouping inside parenthesis,
                        // so we mark the current parenthesized expression as not plain
                        match expression.expression()? {
                            JsAnyExpression::JsLogicalExpression(_)
                            | JsAnyExpression::JsBinaryExpression(_) => Ok(false),
                            _ => Ok(true),
                        }
                    }

                    _ => {
                        if let Some(function) =
                            JsAnyFunction::cast(current_expression.syntax().clone())
                        {
                            Ok(is_simple_function_expression(function)?)
                        } else {
                            Ok(false)
                        }
                    }
                }
            }
            TemplateElement::Ts(template_element) => {
                let is_mapped_type = matches!(template_element.ty()?, TsType::TsMappedType(_));
                Ok(!is_mapped_type)
            }
        }
    }

    fn has_comments(&self) -> bool {
        match self {
            TemplateElement::Js(template_element) => {
                template_element.syntax().has_comments_descendants()
            }
            TemplateElement::Ts(template_element) => {
                template_element.syntax().has_comments_descendants()
            }
        }
    }
}

/// This enum is used to extract a precedence from certain nodes. By comparing the precedence
/// of two nodes, it's possible to change the way certain node should be formatted.
///
/// A use case, for example, is when comparing a node with its parent. If the parent has a lower
/// precedence, then the node can change its formatting.
#[derive(Debug, Ord, PartialOrd, Eq, PartialEq)]
pub(crate) enum FormatPrecedence {
    /// No precedence given to these nodes
    None,

    /// Low priority
    Low,

    /// High priority
    High,
}

impl FormatPrecedence {
    /// Use this function when you want to extract the precedence of the current node
    /// based on whether it can parenthesised or not.
    ///
    /// This function is useful when we want to compare a node against its parent. If the parent has
    /// lower precedence, it means that we can remove the parenthesis from the current node.
    ///
    /// An example can be:
    ///
    /// ```js
    /// let a = ("simple expression") + " or not";
    /// ```
    ///
    /// In this case, we have a parenthesised expression and its parent is a binary expression.
    /// The first one will have [FormatPrecedence::Low] as priority and the second has
    /// [FormatPrecedence::None] as priority. In this case, the parenthesis can be omitted.
    pub fn with_precedence_for_parenthesis(node: Option<&JsSyntaxNode>) -> Self {
        node.map_or(FormatPrecedence::None, |node| match node.kind() {
            JsSyntaxKind::JS_PARENTHESIZED_EXPRESSION => FormatPrecedence::Low,

            JsSyntaxKind::TS_AS_EXPRESSION
            | JsSyntaxKind::TS_NON_NULL_ASSERTION_EXPRESSION
            | JsSyntaxKind::TS_TYPE_ASSERTION_EXPRESSION
            | JsSyntaxKind::JS_UNARY_EXPRESSION
            | JsSyntaxKind::JS_LOGICAL_EXPRESSION
            | JsSyntaxKind::JS_BINARY_EXPRESSION
            | JsSyntaxKind::JS_TEMPLATE
            | JsSyntaxKind::JS_SPREAD
            | JsSyntaxKind::JS_STATIC_MEMBER_EXPRESSION
            | JsSyntaxKind::JS_STATIC_MEMBER_ASSIGNMENT
            | JsSyntaxKind::JS_CALL_EXPRESSION
            | JsSyntaxKind::JS_NEW_EXPRESSION
            | JsSyntaxKind::JS_CONDITIONAL_EXPRESSION
            | JsSyntaxKind::JS_EXTENDS_CLAUSE
            | JsSyntaxKind::TS_IMPLEMENTS_CLAUSE
            | JsSyntaxKind::JS_AWAIT_EXPRESSION
            | JsSyntaxKind::JS_YIELD_ARGUMENT
            | JsSyntaxKind::JS_ARROW_FUNCTION_EXPRESSION
            | JsSyntaxKind::JS_EXPRESSION_STATEMENT
            | JsSyntaxKind::JS_RETURN_STATEMENT
            | JsSyntaxKind::JS_COMPUTED_MEMBER_EXPRESSION
            | JsSyntaxKind::JS_COMPUTED_MEMBER_ASSIGNMENT => FormatPrecedence::High,

            _ => FormatPrecedence::None,
        })
    }
}

/// Format a some code followed by an optional semicolon, and performs
/// semicolon insertion if it was missing in the input source and the
/// preceeding element wasn't an unknown node
pub(crate) fn format_with_semicolon(
    formatter: &Formatter,
    content: FormatElement,
    semicolon: Option<JsSyntaxToken>,
) -> FormatResult<FormatElement> {
    let is_unknown = match content.last_element() {
        Some(FormatElement::Verbatim(elem)) => elem.is_unknown(),
        _ => false,
    };

    formatted![
        formatter,
        [
            content,
            semicolon.format().or_format(if is_unknown {
                empty_element
            } else {
                || token(";")
            })
        ]
    ]
}

pub(crate) fn format_string_literal_token(
    token: JsSyntaxToken,
    formatter: &Formatter,
) -> FormatElement {
    let quoted = token.text_trimmed();
    let (primary_quote_char, secondary_quote_char) = match formatter.options().quote_style {
        QuoteStyle::Double => ('"', '\''),
        QuoteStyle::Single => ('\'', '"'),
    };
    let content =
        if quoted.starts_with(secondary_quote_char) && !quoted.contains(primary_quote_char) {
            let s = &quoted[1..quoted.len() - 1];
            let s = format!("{}{}{}", primary_quote_char, s, primary_quote_char);
            Cow::Owned(normalize_newlines(&s, ['\r']).into_owned())
        } else {
            normalize_newlines(quoted, ['\r'])
        };

    formatter.format_replaced(
        &token,
        Token::from_syntax_token_cow_slice(content, &token, token.text_trimmed_range().start())
            .into(),
    )
}

/// A call like expression is one of:
///
/// - [JsNewExpression]
/// - [JsImportCallExpression]
/// - [JsCallExpression]
pub(crate) fn is_call_like_expression(expression: &JsAnyExpression) -> bool {
    matches!(
        expression,
        JsAnyExpression::JsNewExpression(_)
            | JsAnyExpression::JsImportCallExpression(_)
            | JsAnyExpression::JsCallExpression(_)
    )
}
