pub(crate) mod array;
mod assignment_like;
mod binary_like_expression;
mod conditional;
pub(crate) mod number_utils;
pub(crate) mod string_utils;

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
pub(crate) mod test_each_template;
mod typescript;

use crate::context::trailing_comma::FormatTrailingComma;
use crate::context::Semicolons;
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
pub(crate) struct FormatTypeMemberSeparator<'a> {
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
pub(crate) struct FormatInitializerClause<'a> {
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

pub(crate) struct FormatInterpreterToken<'a> {
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
            write!(f, [interpreter.format()])?;

            match interpreter
                .next_token()
                .map_or(0, |next_token| get_lines_before_token(&next_token))
            {
                0 | 1 => write!(f, [hard_line_break()]),
                _ => write!(f, [empty_line()]),
            }
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

pub(crate) type FormatStatementSemicolon<'a> = FormatOptionalSemicolon<'a>;

/// Formats a semicolon in a position where it is optional (not needed to maintain syntactical correctness).
///
/// * Inserts a new semicolon if it is absent and [JsFormatOptions::semicolons] is [Semicolons::Always].
/// * Removes the semicolon if it is present and [JsFormatOptions::semicolons] is [Semicolons::AsNeeded].
pub(crate) struct FormatOptionalSemicolon<'a> {
    semicolon: Option<&'a JsSyntaxToken>,
}

impl<'a> FormatOptionalSemicolon<'a> {
    pub(crate) fn new(semicolon: Option<&'a JsSyntaxToken>) -> Self {
        Self { semicolon }
    }
}

impl Format<JsFormatContext> for FormatOptionalSemicolon<'_> {
    fn fmt(&self, f: &mut Formatter<JsFormatContext>) -> FormatResult<()> {
        match f.options().semicolons() {
            Semicolons::Always => FormatSemicolon::new(self.semicolon).fmt(f),
            Semicolons::AsNeeded => match self.semicolon {
                None => Ok(()),
                Some(semicolon) => format_removed(semicolon).fmt(f),
            },
        }
    }
}

/// Format some code followed by an optional semicolon.
/// Performs semicolon insertion if it is missing in the input source, the [semicolons option](crate::JsFormatOptions::semicolons) is [Semicolons::Always], and the
/// preceding element isn't an unknown node
pub(crate) struct FormatSemicolon<'a> {
    semicolon: Option<&'a JsSyntaxToken>,
}

impl<'a> FormatSemicolon<'a> {
    pub fn new(semicolon: Option<&'a JsSyntaxToken>) -> Self {
        Self { semicolon }
    }
}

impl Format<JsFormatContext> for FormatSemicolon<'_> {
    fn fmt(&self, f: &mut JsFormatter) -> FormatResult<()> {
        match self.semicolon {
            Some(semicolon) => semicolon.format().fmt(f),
            None => {
                let is_after_unknown = f.elements().start_tag(TagKind::Verbatim).map_or(
                    false,
                    |signal| match signal {
                        Tag::StartVerbatim(kind) => kind.is_unknown(),
                        _ => unreachable!(),
                    },
                );

                if !is_after_unknown {
                    write!(f, [text(";")])?;
                }

                Ok(())
            }
        }
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
            join_with.entry(&format_args![&element, FormatTrailingComma::All]);
        } else {
            join_with.entry(&element);
        }
    }

    join_with.finish()
}
