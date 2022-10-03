pub(crate) mod array;
mod assignment_like;
mod binary_like_expression;
mod conditional;
pub mod string_utils;

pub(crate) mod format_class;
pub(crate) mod function_body;
pub mod jsx;
pub(crate) mod member_chain;
mod object;
mod object_like;
mod object_pattern_like;
#[cfg(test)]
mod quickcheck_utils;
pub(crate) mod test_call;
mod typescript;

use crate::parentheses::is_callee;
pub(crate) use crate::parentheses::resolve_left_most_expression;
use crate::prelude::*;
pub(crate) use assignment_like::{
    with_assignment_layout, AssignmentLikeLayout, JsAnyAssignmentLike,
};
pub(crate) use binary_like_expression::{
    needs_binary_like_parentheses, JsAnyBinaryLikeExpression, JsAnyBinaryLikeLeftExpression,
};
pub(crate) use conditional::{ConditionalJsxChain, JsAnyConditional};
pub(crate) use object_like::JsObjectLike;
pub(crate) use object_pattern_like::JsObjectPatternLike;
use rome_formatter::{format_args, write, Buffer};
use rome_js_syntax::{
    JsAnyExpression, JsAnyStatement, JsCallExpression, JsInitializerClause, JsLanguage, Modifiers,
};
use rome_js_syntax::{JsSyntaxNode, JsSyntaxToken};
use rome_rowan::{AstNode, AstNodeList};
pub(crate) use string_utils::*;
pub(crate) use typescript::{
    is_object_like_type, should_hug_type, union_or_intersection_type_needs_parentheses,
    TsIntersectionOrUnionTypeList,
};

/// Tests if expression is a long curried call
///
/// ```javascript
/// `connect(a, b, c)(d)`
/// ```
pub(crate) fn is_long_curried_call(expression: Option<&JsCallExpression>) -> bool {
    if let Some(expression) = expression {
        if let Some(parent_call) = expression.parent::<JsCallExpression>() {
            if let (Ok(arguments), Ok(parent_arguments)) =
                (expression.arguments(), parent_call.arguments())
            {
                return is_callee(expression.syntax(), parent_call.syntax())
                    && arguments.args().len() > parent_arguments.args().len()
                    && !parent_arguments.args().is_empty();
            }
        }
    }

    false
}

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
        let mut recording = f.start_recording();
        write!(recording, [self.content])?;

        let written = recording.stop();

        let is_unknown =
            written
                .start_tag(TagKind::Verbatim)
                .map_or(false, |signal| match signal {
                    Tag::StartVerbatim(kind) => kind.is_unknown(),
                    _ => unreachable!(),
                });

        if let Some(semicolon) = self.semicolon {
            write!(f, [semicolon.format()])?;
        } else if !is_unknown {
            text(";").fmt(f)?;
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
