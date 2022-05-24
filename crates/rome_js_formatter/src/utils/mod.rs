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
use rome_formatter::normalize_newlines;
use rome_js_syntax::suppression::{has_suppressions_category, SuppressionCategory};
use rome_js_syntax::{
    JsAnyClassMemberName, JsAnyExpression, JsAnyFunction, JsAnyObjectMemberName, JsAnyStatement,
    JsInitializerClause, JsLanguage, JsLiteralMemberName, JsTemplateElement,
    JsTemplateElementFields, Modifiers, TextSize, TsTemplateElement, TsTemplateElementFields,
    TsType,
};
use rome_js_syntax::{JsSyntaxKind, JsSyntaxNode, JsSyntaxToken};
use rome_rowan::{AstNode, AstNodeList};
pub(crate) use simple::*;
pub(crate) use string_utils::*;

/// Utility function to format the separators of the nodes that belong to the unions
/// of [rome_js_syntax::TsAnyTypeMember].
///
/// We can have two kind of separators: `,`, `;` or ASI.
/// Because of how the grammar crafts the nodes, the parent will add the separator to the node.
/// So here, we create - on purpose - an empty node.
pub(crate) fn format_type_member_separator(
    separator_token: Option<JsSyntaxToken>,
    formatter: &JsFormatter,
) -> FormatElement {
    if let Some(separator) = separator_token {
        formatter.format_replaced(&separator, empty_element())
    } else {
        empty_element()
    }
}

/// Utility function to format the node [rome_js_syntax::JsInitializerClause]
pub(crate) fn format_initializer_clause(
    formatter: &JsFormatter,
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
    formatter: &JsFormatter,
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

/// Returns true if this node contains newlines in trivias.
pub(crate) fn has_leading_newline(node: &JsSyntaxNode) -> bool {
    if let Some(leading_trivia) = node.first_leading_trivia() {
        for piece in leading_trivia.pieces() {
            if piece.is_newline() {
                return true;
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
    formatter: &JsFormatter,
    head: FormatElement,
    body: JsAnyStatement,
) -> FormatResult<FormatElement> {
    if matches!(body, JsAnyStatement::JsBlockStatement(_)) {
        formatted![formatter, [head, space_token(), body.format(),]]
    } else if matches!(body, JsAnyStatement::JsEmptyStatement(_)) {
        // Force semicolon insertion if the body is empty
        formatted![formatter, [head, body.format(), token(";"),]]
    } else {
        formatted![formatter, [head, space_token(), body.format(),]]
    }
}

pub(crate) fn has_formatter_suppressions(node: &JsSyntaxNode) -> bool {
    has_suppressions_category(SuppressionCategory::Format, node)
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
    formatter: &JsFormatter,
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
    formatter: &JsFormatter,
) -> FormatResult<FormatElement> {
    literal.into_format_element(formatter)
}

pub(crate) enum TemplateElement {
    Js(JsTemplateElement),
    Ts(TsTemplateElement),
}

impl TemplateElement {
    pub fn into_format_element(self, formatter: &JsFormatter) -> FormatResult<FormatElement> {
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

        let middle = format_elements![middle, line_suffix_boundary()];

        if should_hard_group {
            formatted![
                formatter,
                [dollar_curly_token.format(), middle, r_curly_token.format()]
            ]
        } else {
            formatter
                .delimited(&dollar_curly_token, middle, &r_curly_token)
                .soft_block_indent()
                .finish()
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
    formatter: &JsFormatter,
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

/// Data structure used to merge into one the following nodes:
///
/// - [JsAnyObjectMemberName]
/// - [JsAnyClassMemberName]
/// - [JsLiteralMemberName]
///
/// Once merged, the enum is used to get specific members (the literal ones) and elide
/// the quotes from them, when the algorithm sees fit
pub(crate) enum MemberName {
    Object(JsAnyObjectMemberName),
    Class(JsAnyClassMemberName),
    Literal(JsLiteralMemberName),
}

impl From<JsAnyClassMemberName> for MemberName {
    fn from(node: JsAnyClassMemberName) -> Self {
        Self::Class(node)
    }
}

impl From<JsAnyObjectMemberName> for MemberName {
    fn from(node: JsAnyObjectMemberName) -> Self {
        Self::Object(node)
    }
}

impl From<JsLiteralMemberName> for MemberName {
    fn from(literal: JsLiteralMemberName) -> Self {
        Self::Literal(literal)
    }
}

const QUOTES_TO_OMIT: [char; 2] = ['\"', '\''];

#[derive(Eq, PartialEq)]
pub(crate) enum MemberContext {
    Type,
    Member,
}

impl MemberContext {
    /// We can change the text only if there alphanumeric or alphabetic characters, depending on the mode
    fn text_can_be_replaced(&self, text_to_check: &str) -> bool {
        // Text here is quoteless. If it's empty, it means it is an empty string and we can't
        // do any transformation
        if text_to_check.is_empty() {
            return false;
        }

        let mut has_seen_number = false;
        text_to_check.chars().enumerate().all(|(index, c)| {
            if index == 0 && c.is_numeric() {
                // In TypeScript, numbers like members have different meaning from numbers.
                // Hence, if we see a number, we bail straightaway
                if self == &MemberContext::Type {
                    return false;
                } else {
                    has_seen_number = true;
                }
            }

            let is_eligible_character = if has_seen_number {
                // as we've seen a number, now eligible characters can only contain numbers
                c.is_numeric()
            } else {
                c.is_alphanumeric()
            };
            is_eligible_character || matches!(c, '_' | '$')
        })
    }
}

/// Function used by the formatter, where we pass a complaint member and it returns a [FormatElement[
/// where the text has its quotes removed.
pub(crate) fn format_member_name<Member: Into<MemberName>>(
    member_name: Member,
    formatter: &Formatter<JsFormatOptions>,
    context: MemberContext,
) -> FormatResult<FormatElement> {
    fn replace_node(
        name: JsSyntaxToken,
        formatter: &Formatter<JsFormatOptions>,
        context: MemberContext,
    ) -> FormatResult<FormatElement> {
        let text = name.text_trimmed();

        if text.starts_with(QUOTES_TO_OMIT) && text.ends_with(QUOTES_TO_OMIT) {
            let quote_less_text = &text[1..text.len() - 1];
            if context.text_can_be_replaced(quote_less_text) {
                Ok(formatter.format_replaced(
                    &name,
                    Token::from_syntax_token_cow_slice(
                        Cow::Borrowed(quote_less_text),
                        &name,
                        // slide the offset of one as we removed a character from the beginning
                        name.text_trimmed_range().start().add(TextSize::from(1)),
                    )
                    .into(),
                ))
            } else {
                Ok(format_string_literal_token(name, formatter))
            }
        } else {
            Ok(format_string_literal_token(name, formatter))
        }
    }

    let name = match member_name.into() {
        MemberName::Object(object) => match object {
            JsAnyObjectMemberName::JsComputedMemberName(name) => name.format(formatter)?,
            JsAnyObjectMemberName::JsLiteralMemberName(name) => {
                replace_node(name.value()?, formatter, context)?
            }
        },
        MemberName::Class(class) => match class {
            JsAnyClassMemberName::JsComputedMemberName(node) => node.format(formatter)?,
            JsAnyClassMemberName::JsLiteralMemberName(node) => {
                replace_node(node.value()?, formatter, context)?
            }
            JsAnyClassMemberName::JsPrivateClassMemberName(node) => node.format(formatter)?,
        },
        MemberName::Literal(literal) => replace_node(literal.value()?, formatter, context)?,
    };

    Ok(name)
}
