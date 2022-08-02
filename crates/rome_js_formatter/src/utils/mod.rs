pub(crate) mod array;
mod assignment_like;
mod binary_like_expression;
mod format_conditional;
mod simple;
pub mod string_utils;

pub(crate) mod format_class;
pub mod jsx_utils;
mod member_chain;
mod object;
mod object_like;
mod object_pattern_like;
#[cfg(test)]
mod quickcheck_utils;
mod typescript;

use crate::prelude::*;
pub(crate) use assignment_like::{should_break_after_operator, JsAnyAssignmentLike};
pub(crate) use binary_like_expression::{
    binary_argument_needs_parens, format_binary_like_expression, JsAnyBinaryLikeExpression,
    JsAnyBinaryLikeLeftExpression,
};
pub(crate) use format_conditional::{format_conditional, Conditional};
pub(crate) use member_chain::format_call_expression;
pub(crate) use object_like::JsObjectLike;
pub(crate) use object_pattern_like::JsObjectPatternLike;
use rome_formatter::{format_args, normalize_newlines, write, Buffer, VecBuffer};
use rome_js_syntax::{
    JsAnyExpression, JsAnyFunction, JsAnyStatement, JsInitializerClause, JsLanguage,
    JsTemplateElement, Modifiers, TsTemplateElement, TsType,
};
use rome_js_syntax::{JsSyntaxKind, JsSyntaxNode, JsSyntaxToken};
use rome_rowan::{AstNode, AstNodeList, Direction, SyntaxResult};
use std::fmt::Debug;
pub(crate) use typescript::should_hug_type;

pub(crate) use simple::*;
pub(crate) use string_utils::*;

/// Utility function to format the separators of the nodes that belong to the unions
/// of [rome_js_syntax::TsAnyTypeMember].
///
/// We can have two kind of separators: `,`, `;` or ASI.
/// Because of how the grammar crafts the nodes, the parent will add the separator to the node.
/// So here, we create - on purpose - an empty node.
pub struct FormatTypeMemberSeparator<'a> {
    token: Option<&'a JsSyntaxToken>,
}

impl<'a> FormatTypeMemberSeparator<'a> {
    pub fn new(token: Option<&'a JsSyntaxToken>) -> Self {
        Self { token }
    }
}

impl Format<JsFormatContext> for FormatTypeMemberSeparator<'_> {
    fn fmt(&self, f: &mut JsFormatter) -> FormatResult<()> {
        if let Some(separator) = self.token {
            format_removed(separator).fmt(f)
        } else {
            Ok(())
        }
    }
}

/// Utility function to format the node [rome_js_syntax::JsInitializerClause]
pub struct FormatInitializerClause<'a> {
    initializer: Option<&'a JsInitializerClause>,
}

impl<'a> FormatInitializerClause<'a> {
    pub fn new(initializer: Option<&'a JsInitializerClause>) -> Self {
        Self { initializer }
    }
}

impl Format<JsFormatContext> for FormatInitializerClause<'_> {
    fn fmt(&self, f: &mut JsFormatter) -> FormatResult<()> {
        if let Some(initializer) = self.initializer {
            write!(f, [space(), initializer.format()])
        } else {
            Ok(())
        }
    }
}

pub struct FormatInterpreterToken<'a> {
    token: Option<&'a JsSyntaxToken>,
}

impl<'a> FormatInterpreterToken<'a> {
    pub fn new(interpreter_token: Option<&'a JsSyntaxToken>) -> Self {
        Self {
            token: interpreter_token,
        }
    }
}

impl Format<JsFormatContext> for FormatInterpreterToken<'_> {
    fn fmt(&self, f: &mut JsFormatter) -> FormatResult<()> {
        if let Some(interpreter) = self.token {
            write!(f, [interpreter.format(), empty_line()])
        } else {
            Ok(())
        }
    }
}

/// Returns true if this node contains "printable" trivias: comments
/// or empty lines (2 consecutive newlines only separated by whitespace)
pub(crate) fn has_formatter_trivia(node: &JsSyntaxNode) -> bool {
    let mut line_count = 0;

    for token in node.descendants_tokens(Direction::Next) {
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
pub(crate) fn node_has_leading_newline(node: &JsSyntaxNode) -> bool {
    if let Some(leading_trivia) = node.first_leading_trivia() {
        for piece in leading_trivia.pieces() {
            if piece.is_newline() {
                return true;
            }
        }
    }
    false
}

/// Formats the body of a statement where it can either be a single statement, an empty statement,
/// or a block statement.
pub(crate) struct FormatStatementBody<'a> {
    body: &'a JsAnyStatement,
    force_space: bool,
}

impl<'a> FormatStatementBody<'a> {
    pub fn new(body: &'a JsAnyStatement) -> Self {
        Self {
            body,
            force_space: false,
        }
    }

    /// Prevents that the consequent is formatted on its own line and indented by one level and
    /// instead gets separated by a space.
    pub fn with_forced_space(mut self, forced: bool) -> Self {
        self.force_space = forced;
        self
    }
}

impl Format<JsFormatContext> for FormatStatementBody<'_> {
    fn fmt(&self, f: &mut Formatter<JsFormatContext>) -> FormatResult<()> {
        use JsAnyStatement::*;

        if let JsEmptyStatement(empty) = &self.body {
            write!(f, [empty.format()])
        } else if matches!(&self.body, JsBlockStatement(_)) || self.force_space {
            write!(f, [space(), self.body.format()])
        } else {
            write!(
                f,
                [indent(&format_args![
                    soft_line_break_or_space(),
                    self.body.format()
                ])]
            )
        }
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
pub(crate) fn format_template_chunk(chunk: JsSyntaxToken, f: &mut JsFormatter) -> FormatResult<()> {
    // Per https://tc39.es/ecma262/multipage/ecmascript-language-lexical-grammar.html#sec-static-semantics-trv:
    // In template literals, the '\r' and '\r\n' line terminators are normalized to '\n'

    write!(
        f,
        [format_replaced(
            &chunk,
            &syntax_token_cow_slice(
                normalize_newlines(chunk.text_trimmed(), ['\r']),
                &chunk,
                chunk.text_trimmed_range().start(),
            )
        )]
    )
}

/// Function to format template literals and template literal types
pub(crate) fn format_template_literal(
    literal: TemplateElement,
    formatter: &mut JsFormatter,
) -> FormatResult<()> {
    write!(formatter, [literal])
}

pub(crate) enum TemplateElement {
    Js(JsTemplateElement),
    Ts(TsTemplateElement),
}

impl Format<JsFormatContext> for TemplateElement {
    fn fmt(&self, f: &mut JsFormatter) -> FormatResult<()> {
        let expression_is_plain = self.is_plain_expression()?;
        let has_comments = self.has_comments();
        let should_hard_group = expression_is_plain && !has_comments;

        let content = format_with(|f| {
            match self {
                TemplateElement::Js(template) => {
                    write!(f, [template.expression().format()])?;
                }
                TemplateElement::Ts(template) => {
                    write!(f, [template.ty().format()])?;
                }
            }

            write!(f, [line_suffix_boundary()])
        });

        if should_hard_group {
            write!(
                f,
                [
                    self.dollar_curly_token().format(),
                    content,
                    self.r_curly_token().format()
                ]
            )
        } else {
            write!(
                f,
                [format_delimited(
                    &self.dollar_curly_token()?,
                    &content,
                    &self.r_curly_token()?
                )
                .soft_block_indent()]
            )
        }
    }
}

impl TemplateElement {
    fn dollar_curly_token(&self) -> SyntaxResult<JsSyntaxToken> {
        match self {
            TemplateElement::Js(template) => template.dollar_curly_token(),
            TemplateElement::Ts(template) => template.dollar_curly_token(),
        }
    }

    fn r_curly_token(&self) -> SyntaxResult<JsSyntaxToken> {
        match self {
            TemplateElement::Js(template) => template.r_curly_token(),
            TemplateElement::Ts(template) => template.r_curly_token(),
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
/// preceding element wasn't an unknown node
pub struct FormatWithSemicolon<'a> {
    content: &'a dyn Format<JsFormatContext>,
    semicolon: Option<&'a JsSyntaxToken>,
}

impl<'a> FormatWithSemicolon<'a> {
    pub fn new(
        content: &'a dyn Format<JsFormatContext>,
        semicolon: Option<&'a JsSyntaxToken>,
    ) -> Self {
        Self { content, semicolon }
    }
}

impl Format<JsFormatContext> for FormatWithSemicolon<'_> {
    fn fmt(&self, f: &mut JsFormatter) -> FormatResult<()> {
        let mut buffer = VecBuffer::new(f.state_mut());

        write!(buffer, [self.content])?;

        let content = buffer.into_element();

        let is_unknown = match content.last_element() {
            Some(FormatElement::Verbatim(elem)) => elem.is_unknown(),
            _ => false,
        };

        f.write_element(content)?;

        if let Some(semicolon) = self.semicolon {
            write!(f, [semicolon.format()])?;
        } else if !is_unknown {
            format_inserted(JsSyntaxKind::SEMICOLON).fmt(f)?;
        }
        Ok(())
    }
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

/// This function is in charge to format the call arguments.
pub(crate) fn write_arguments_multi_line<S: Format<JsFormatContext>, I>(
    separated: I,
    f: &mut JsFormatter,
) -> FormatResult<()>
where
    I: Iterator<Item = S>,
{
    let mut iterator = separated.peekable();
    let mut join_with = f.join_with(soft_line_break_or_space());

    while let Some(element) = iterator.next() {
        let last = iterator.peek().is_none();

        if last {
            join_with.entry(&format_args![&element, &if_group_breaks(&text(","))]);
        } else {
            join_with.entry(&element);
        }
    }

    join_with.finish()
}
