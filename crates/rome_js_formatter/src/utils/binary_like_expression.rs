use crate::prelude::*;
use rome_formatter::{write, Buffer, CstFormatContext};
use rome_js_syntax::{
    JsAnyExpression, JsAnyInProperty, JsBinaryExpression, JsBinaryOperator, JsInExpression,
    JsInstanceofExpression, JsLogicalExpression, JsLogicalOperator, JsPrivateName, JsSyntaxKind,
    JsSyntaxNode, JsSyntaxToken, OperatorPrecedence,
};

use crate::utils::should_break_after_operator;
use rome_rowan::{declare_node_union, AstNode, SyntaxResult};
use std::fmt::Debug;
use std::iter::FusedIterator;
use std::ops::Deref;

/// This function is charge to flat binaryish expressions that have the same precedence of their operators
///
/// This means that expressions like `some && thing && elsewhere` are entitled to fall in the same group.
///
/// Instead, if we encounter something like `some && thing  || elsewhere && thing`, we will creat two groups:
/// `[some, thing]` and `[elsewhere, thing]`, each group will be grouped together.
///
///
/// Let's take for example:
///
/// ```js
/// some && thing && elsewhere && happy
/// ```
///
/// These expressions have nested nodes, which is roughly something like this:
///
/// ```block
/// JsLogicalExpression {
///     left: JsLogicalExpression {
///         left: JsLogicalExpression {
///             left: "some"
///             operator: "&&",
///             right: "thing"
///         }
///         operator: "&&"
///         right: "elsewhere"
///     }
///     operator: "&&"
///     right: "happy"
/// }
/// ```
///
/// Our final result should be something like this:
/// ```js
/// some &&
/// thing &&
/// elsewhere &&
/// happy
/// ```
///
/// So what we are going to do here is:
/// - create a vector of flatten items, where the most nested node is the first one,`left: "some"` in our
/// example. The last one will be the first that we encounter, in this case the node that contains `right: "happy"`
/// - each element of the vector will contain two elements. One is the AST node, the other one is its
/// formatted version
/// - the formatted elements will be grouped
///
///
/// The flattening of the groups is done by traversing the binary like expression in post-order, first visiting the left most binary like expression:
/// - not printing nodes/token twice
/// - not "forget" tokens/nodes
/// - apply recursions as long as we encounter the same operator
///
/// By looking at the formatting, we want to make sure that the operator is always attached to the
/// "left" part of the expression, which means that the last "right" wont' have any operator.
///
/// In order to achieve that, we basically carry with us the operator of the previous node.
///
/// Let's try to understand it by checking the example again. The first time we attempt to create a
/// flatten item is when we encounter: `some && thing`, which is a `JsLogicalExpression`.
/// Nothing fancy here. Although, if we needed to format this node, you would notice that we don't have
/// a second operator, because our end result should be:
///
/// ```js
/// some &&
/// thing &&
/// ```
///
/// So what we do is to "borrow" (no Rust reference) the operator "&&" that belongs to the "parent" -
/// or, if want to see it from a recursion point of view, the previous node that we visited -
/// in our case `elsewhere &&`. We then take its operator token and pass it down.
///
/// Eventually we will have a `[ JsLogicalExpression, operator2: "&&" ]`.
///
/// With these elements, we can now create two formatted elements:
/// - `[left, operator: "&&" ]`
/// - `[right, operator2: "&&" ]`
///
/// Now let's continue until we arrive to the last node that we want to try to format, which is:
/// `&& happy`. If we follow the logic explained so far, this node doesn't have an operator
/// anymore because we passed it to its child. And we can't try to add a new operator.
/// But this is fine! Because this is want we wanted! By removing the operator, we are left with `happy`
/// which is what we wanted since the beginning!
pub(crate) fn format_binary_like_expression(
    expression: JsAnyBinaryLikeExpression,
    f: &mut JsFormatter,
) -> FormatResult<()> {
    let mut flatten_items = FlattenItems::default();
    let current_node = expression.clone();

    let post_order_binary_like_expressions = PostorderIterator::new(expression);
    let mut left: Option<JsAnyBinaryLikeExpression> = None;

    for parent in post_order_binary_like_expressions {
        let parent_operator = parent.operator_token()?;

        if let Some(left) = left {
            // It's only possible to suppress the formatting of the whole binary expression formatting OR
            // the formatting of the right hand side value but not of a nested binary expression.
            f.context()
                .comments()
                .mark_suppression_checked(left.syntax());
            flatten_items.flatten_binary_expression_right_hand_side(left, Some(parent_operator))?;
        } else {
            // Leaf binary like expression. Format the left hand side.
            // The right hand side gets formatted when traversing upwards in the tree.

            let left = parent.left()?;
            let has_comments = left.syntax().has_comments_direct();

            flatten_items.items.push(FlattenItem::new(
                FlattenedBinaryExpressionPart::Left { expression: left },
                Some(parent_operator),
                has_comments.into(),
            ));
        }

        left = Some(parent);
    }

    // Format the top most binary like expression
    if let Some(root) = left {
        flatten_items.flatten_binary_expression_right_hand_side(root, None)?;
    }

    let group = FlattenedBinaryExpressionPart::Group {
        current: JsAnyBinaryLikeLeftExpression::JsAnyExpression(current_node.into_expression()),
        expressions_start: flatten_items.current_group_start,
        expressions_end: flatten_items.len(),
        parenthesized: false,
    };

    group.write(f, &flatten_items)
}

/// Small wrapper to identify the operation of an expression and deduce their precedence
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub(crate) enum BinaryLikeOperator {
    Logical(JsLogicalOperator),
    Binary(JsBinaryOperator),
    Instanceof,
    In,
}

impl BinaryLikeOperator {
    pub const fn precedence(&self) -> OperatorPrecedence {
        match self {
            BinaryLikeOperator::Logical(logical) => logical.precedence(),
            BinaryLikeOperator::Binary(binary) => binary.precedence(),
            BinaryLikeOperator::Instanceof | BinaryLikeOperator::In => {
                OperatorPrecedence::Relational
            }
        }
    }

    pub const fn is_logical(&self) -> bool {
        matches!(self, BinaryLikeOperator::Logical(_))
    }

    pub const fn is_remainder(&self) -> bool {
        matches!(
            self,
            BinaryLikeOperator::Binary(JsBinaryOperator::Remainder)
        )
    }
}

impl From<JsLogicalOperator> for BinaryLikeOperator {
    fn from(operator: JsLogicalOperator) -> Self {
        BinaryLikeOperator::Logical(operator)
    }
}

impl From<JsBinaryOperator> for BinaryLikeOperator {
    fn from(binary: JsBinaryOperator) -> Self {
        BinaryLikeOperator::Binary(binary)
    }
}

/// This function returns `true` when the binary expression should be wrapped in parentheses to either
/// * a) correctly encode precedence
/// * b) Improve readability by adding parentheses around expressions where the precedence may not be obvious to many readers.
pub(crate) fn binary_argument_needs_parens(
    parent_operator: BinaryLikeOperator,
    node: &JsAnyBinaryLikeLeftExpression,
    is_right: bool,
) -> SyntaxResult<bool> {
    let current_operator =
        if let Some(binary_like) = JsAnyBinaryLikeExpression::cast(node.syntax().clone()) {
            binary_like.operator()?
        } else {
            return Ok(false);
        };

    // For logical nodes, add parentheses if the operators aren't the same
    if parent_operator.is_logical() && current_operator.is_logical() {
        return Ok(parent_operator != current_operator);
    }

    let current_precedence = current_operator.precedence();
    let parent_precedence = parent_operator.precedence();

    #[allow(clippy::if_same_then_else, clippy::needless_bool)]
    // If the parent has a higher precedence than parentheses are necessary to not change the semantic meaning
    // when re-parsing.
    let result = if parent_precedence > current_precedence {
        true
    } else if is_right && parent_precedence == current_precedence {
        true
    }
    // Add parentheses around bitwise and bit shift operators
    // `a * 3 >> 5` -> `(a * 3) >> 5`
    else if parent_precedence.is_bitwise() || parent_precedence.is_shift() {
        true
    }
    // `a % 4 + 4` -> `a % (4 + 4)`
    else if parent_precedence < current_precedence && current_operator.is_remainder() {
        parent_precedence.is_additive()
    } else if parent_precedence == current_precedence
        && !should_flatten(parent_operator, current_operator)
    {
        true
    } else {
        false
    };

    Ok(result)
}

// False positive, Removing the `+ 'a` lifetime fails to compile with `hidden type for `impl Trait` captures lifetime that does not appear in bounds`
#[allow(clippy::needless_lifetimes)]
fn format_sub_expression<'a>(
    parent_operator: BinaryLikeOperator,
    sub_expression: &'a JsAnyBinaryLikeLeftExpression,
) -> impl Format<JsFormatContext> + 'a {
    format_with(move |f| {
        if binary_argument_needs_parens(parent_operator, sub_expression, true)? {
            format_parenthesize(
                sub_expression.syntax().first_token().as_ref(),
                &sub_expression,
                sub_expression.syntax().last_token().as_ref(),
            )
            .grouped_with_soft_block_indent()
            .fmt(f)
        } else {
            write!(f, [sub_expression])
        }
    })
}

fn keep_on_same_line(flatten_nodes: &[FlattenItem]) -> bool {
    // We don't want to have 1 + 2 to break, for example.
    // But if there are any trailing comments, break it.
    flatten_nodes.len() <= 2 && flatten_nodes.iter().all(|node| !node.has_comments())
}

fn is_inside_parenthesis(current_node: &JsSyntaxNode) -> bool {
    let parent_kind = current_node.parent().map(|parent| parent.kind());

    matches!(
        parent_kind,
        Some(
            JsSyntaxKind::JS_IF_STATEMENT
                | JsSyntaxKind::JS_DO_WHILE_STATEMENT
                | JsSyntaxKind::JS_WHILE_STATEMENT
                | JsSyntaxKind::JS_SWITCH_STATEMENT
                | JsSyntaxKind::JS_TEMPLATE_ELEMENT
                | JsSyntaxKind::TS_TEMPLATE_ELEMENT
        )
    )
}

/// This function checks whether the chain of logical/binary expressions **should not** be indented
///
/// There are some cases where the indentation is done by the parent, so if the parent is already doing
/// the indentation, then there's no need to do a second indentation.
/// [Prettier applies]: https://github.com/prettier/prettier/blob/b0201e01ef99db799eb3716f15b7dfedb0a2e62b/src/language-js/print/binaryish.js#L122-L125
fn should_not_indent_if_parent_indents(current_node: &JsAnyBinaryLikeLeftExpression) -> bool {
    let parent = current_node
        .syntax()
        .ancestors()
        .skip(1)
        .find(|parent| parent.kind() != JsSyntaxKind::JS_PARENTHESIZED_EXPRESSION);

    let parent_kind = parent.as_ref().map(|node| node.kind());

    let great_parent = parent.and_then(|parent| parent.parent());
    let great_parent_kind = great_parent.map(|node| node.kind());

    match (parent_kind, great_parent_kind) {
        (Some(JsSyntaxKind::JS_PROPERTY_OBJECT_MEMBER), _)
        | (Some(JsSyntaxKind::JS_INITIALIZER_CLAUSE), Some(JsSyntaxKind::JS_VARIABLE_DECLARATOR)) => {
            current_node
                .as_expression()
                .and_then(|expression| should_break_after_operator(expression).ok())
                .unwrap_or(false)
        }
        (
            Some(
                JsSyntaxKind::JS_RETURN_STATEMENT
                | JsSyntaxKind::JS_THROW_STATEMENT
                | JsSyntaxKind::JS_ARROW_FUNCTION_EXPRESSION,
            ),
            _,
        ) => true,
        _ => false,
    }
}

/// There are other cases where the parent decides to inline the the element; in
/// these cases the decide to actually break on a new line and indent it.
///
/// This function checks what the parents adheres to this behaviour
fn should_indent_if_parent_inlines(current_node: &JsAnyBinaryLikeLeftExpression) -> bool {
    let parent = current_node.syntax().parent();
    let grand_parent = parent.as_ref().and_then(|p| p.parent());

    match (parent, grand_parent) {
        (Some(parent), Some(grand_parent)) => {
            parent.kind() == JsSyntaxKind::JS_INITIALIZER_CLAUSE
                && grand_parent.kind() == JsSyntaxKind::JS_VARIABLE_DECLARATOR
        }

        _ => false,
    }
}

#[derive(Debug, Default)]
struct FlattenItems {
    items: Vec<FlattenItem>,

    /// Position into `items` where the next group starts.
    current_group_start: usize,
}

impl Deref for FlattenItems {
    type Target = [FlattenItem];

    fn deref(&self) -> &Self::Target {
        &self.items
    }
}

impl FlattenItems {
    /// Formats the right hand side of a binary like expression
    fn flatten_binary_expression_right_hand_side(
        &mut self,
        expression: JsAnyBinaryLikeExpression,
        parent_operator: Option<JsSyntaxToken>,
    ) -> FormatResult<()> {
        let should_flatten = expression.can_flatten()?;

        if should_flatten {
            self.flatten_right_hand_side(expression, parent_operator)
        } else {
            self.flatten_new_binary_like_group(expression, parent_operator)
        }
    }

    /// Flattens the right hand operand of a binary like expression.
    fn flatten_right_hand_side(
        &mut self,
        binary_like_expression: JsAnyBinaryLikeExpression,
        parent_operator: Option<JsSyntaxToken>,
    ) -> FormatResult<()> {
        let right = JsAnyBinaryLikeLeftExpression::JsAnyExpression(binary_like_expression.right()?);
        let has_comments = right.syntax().has_comments_direct();

        let flatten_item = FlattenItem::new(
            FlattenedBinaryExpressionPart::Right {
                parent: binary_like_expression,
            },
            parent_operator,
            has_comments.into(),
        );
        self.items.push(flatten_item);

        Ok(())
    }

    /// The left hand-side expression and the current operator cannot be flattened.
    /// Format the left hand side on its own and potentially wrap it in parentheses before formatting
    /// the right-hand side of the current expression.
    fn flatten_new_binary_like_group(
        &mut self,
        binary_like_expression: JsAnyBinaryLikeExpression,
        parent_operator: Option<JsSyntaxToken>,
    ) -> FormatResult<()> {
        if let Some(last) = self.items.last_mut() {
            // Remove any line breaks and the trailing operator so that the operator/trailing aren't part
            // of the parenthesized expression.
            last.terminator = TrailingTerminator::None;
            last.operator = None;
        }

        let left = binary_like_expression.left()?;
        let operator = binary_like_expression.operator()?;
        let operator_token = binary_like_expression.operator_token()?;

        let operator_has_trailing_comments = operator_token.has_trailing_comments();
        let left_parenthesized = binary_argument_needs_parens(operator, &left, false)?;
        let mut left_item = FlattenItem::new(
            FlattenedBinaryExpressionPart::Group {
                current: left,
                expressions_start: self.current_group_start,
                expressions_end: self.items.len(),
                parenthesized: left_parenthesized,
            },
            Some(operator_token),
            operator_has_trailing_comments.into(),
        );

        if operator_has_trailing_comments {
            left_item = left_item.with_terminator(TrailingTerminator::HardLineBreak);
        }

        self.current_group_start = self.len();
        self.items.push(left_item);

        let right = JsAnyBinaryLikeLeftExpression::JsAnyExpression(binary_like_expression.right()?);

        // Flatten the right node
        let parent_operator_has_comments = parent_operator
            .as_ref()
            .map(|operator| operator.has_leading_comments());

        let mut right_item = FlattenItem::new(
            FlattenedBinaryExpressionPart::Right {
                parent: binary_like_expression,
            },
            parent_operator,
            Commented::No,
        );

        // Format the parent operator
        if let Some(parent_operator_has_comments) = parent_operator_has_comments {
            // Here we care only about trailing comments that belong to the previous operator
            if parent_operator_has_comments {
                right_item = right_item
                    .with_comments(true)
                    .with_terminator(TrailingTerminator::HardLineBreak)
            }
        } else {
            // Here we want to check only leading comments;
            // trailing comments will be added after the end of the whole expression.
            // We want to handle cases like `lorem && (3 + 5 == 9) // comment`.
            // This part is a signal to the formatter to tell it if the whole expression should break.
            right_item = right_item.with_comments(right.syntax().has_leading_comments())
        };

        self.items.push(right_item);

        Ok(())
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Commented {
    Yes,
    No,
}

impl From<Commented> for bool {
    fn from(comments: Commented) -> Self {
        match comments {
            Commented::Yes => true,
            Commented::No => false,
        }
    }
}
impl From<bool> for Commented {
    fn from(b: bool) -> Self {
        match b {
            true => Commented::Yes,
            false => Commented::No,
        }
    }
}

/// The left or right sub part of a binary expression.
#[derive(Debug)]
enum FlattenedBinaryExpressionPart {
    /// The right hand side of a binary expression. Needs to format the parent operator and the right expression
    Right {
        /// The parent expression
        parent: JsAnyBinaryLikeExpression,
    },

    /// The very first left hand side of a binary expression. Only formats the expression
    Left {
        /// The left hand side expression
        expression: JsAnyBinaryLikeLeftExpression,
    },

    /// A group of expressions that can be grouped/printed together.
    Group {
        /// The binary expression that should be formatted now
        current: JsAnyBinaryLikeLeftExpression,

        /// Start end/index into the flattened items array from where the left hand side expressions start
        expressions_start: usize,
        expressions_end: usize,

        /// Whether to parenthesize the expression
        parenthesized: bool,
    },
}

impl FlattenedBinaryExpressionPart {
    fn write(&self, f: &mut JsFormatter, items: &[FlattenItem]) -> FormatResult<()> {
        match self {
            FlattenedBinaryExpressionPart::Right { parent } => {
                let right = JsAnyBinaryLikeLeftExpression::JsAnyExpression(parent.right()?);

                write!(f, [format_sub_expression(parent.operator()?, &right)])
            }
            FlattenedBinaryExpressionPart::Left { expression } => {
                write!(f, [expression])
            }
            FlattenedBinaryExpressionPart::Group {
                current,
                expressions_start,
                expressions_end,
                parenthesized,
            } => {
                let expressions = &items[*expressions_start..*expressions_end];
                let content = format_with(|f| {
                    let keep_on_same_line = keep_on_same_line(expressions);

                    let mut groups = expressions.iter().map(|group| {
                        format_with(|f| {
                            group.expression.write(f, items)?;

                            if let Some(operator) = &group.operator {
                                write!(f, [space(), operator.format()])?;
                            }

                            match &group.terminator {
                                TrailingTerminator::None => (),
                                TrailingTerminator::HardLineBreak => {
                                    write!(f, [hard_line_break()])?
                                }
                            };

                            Ok(())
                        })
                    });

                    if keep_on_same_line {
                        // we bail early if group doesn't need to be broken. We don't need to do further checks
                        f.join_with(space()).entries(groups).finish()
                    } else if is_inside_parenthesis(current.syntax()) {
                        f.join_with(soft_line_break_or_space())
                            .entries(groups)
                            .finish()
                    } else if should_not_indent_if_parent_indents(current) {
                        write!(
                            f,
                            [group(&format_once(|f| {
                                f.join_with(soft_line_break_or_space())
                                    .entries(groups)
                                    .finish()
                            }))]
                        )
                    } else if should_indent_if_parent_inlines(current) {
                        write!(
                            f,
                            [soft_line_indent_or_space(&group(&format_once(|f| {
                                f.join_with(soft_line_break_or_space())
                                    .entries(groups)
                                    .finish()
                            })))]
                        )
                    } else {
                        // if none of the previous conditions is met,
                        // we take out the first element from the rest of the group
                        // and indent the rest of the groups in a new line

                        // SAFETY: Safe because `keep_on_same_line` returns `true` if this is a single
                        // binary expression without any nested sub expressions.
                        write!(f, [groups.next().unwrap()])?;

                        write!(
                            f,
                            [group(&soft_line_indent_or_space(&format_once(|f| {
                                f.join_with(soft_line_break_or_space())
                                    .entries(groups)
                                    .finish()
                            })))]
                        )
                    }
                });

                if *parenthesized {
                    let first_token = current.syntax().first_token();
                    let last_token = current.syntax().last_token();

                    format_parenthesize(first_token.as_ref(), &content, last_token.as_ref())
                        .grouped_with_soft_block_indent()
                        .fmt(f)
                } else {
                    write!(f, [content])
                }
            }
        }
    }
}

#[derive(Debug)]
struct FlattenItem {
    expression: FlattenedBinaryExpressionPart,
    operator: Option<JsSyntaxToken>,
    terminator: TrailingTerminator,
    comments: Commented,
}

#[derive(Debug)]
enum TrailingTerminator {
    None,
    HardLineBreak,
}

impl FlattenItem {
    fn new(
        expression: FlattenedBinaryExpressionPart,
        operator: Option<JsSyntaxToken>,
        comments: Commented,
    ) -> Self {
        Self {
            expression,
            operator,
            terminator: TrailingTerminator::None,
            comments,
        }
    }

    fn has_comments(&self) -> bool {
        matches!(self.comments, Commented::Yes)
    }

    fn with_terminator(mut self, terminator: TrailingTerminator) -> Self {
        self.terminator = terminator;
        self
    }

    fn with_comments<I: Into<Commented>>(mut self, comments: I) -> Self {
        self.comments = comments.into();
        self
    }
}

/// The [PostorderIterator] visits every node twice. First on the way down to find the left most binary
/// like expression, then on the way back up when it yields the binary like expressions.
/// This enum encodes the information whatever the iterator is on its way down (`Enter`) or traversing
/// upwards (`Exit`).
#[derive(Debug)]
enum VisitEvent {
    Enter(JsAnyBinaryLikeExpression),
    Exit(JsAnyBinaryLikeExpression),
}

/// Iterator that first returns the left-most binary-like expression and then traverses upwards to the start node.
/// The binary like expression nodes are yielded when traversing upwards.
///
/// # Examples
///
/// ```js
/// a && b && c && d
/// ```
/// This produces a tree with the following shape:
///
/// ```txt
///         &&
///        / \
///       /   \
///      &&   d
///     / \
///    /   \
///   &&    c
///  / \
/// a   b
/// ```
///
/// The iterator follows the left branches of the binary expressions without until it hits any non
/// binary-like expression (in this case the reference identifier `a`). From there, the iterator starts
/// traversing upwards again and yields the binary expression along the way. The returned nodes for the above
/// examples are (in that exact order):
/// 1. `a && b`
/// 2. `a && b && c`
/// 3. `a && b && c && d`
struct PostorderIterator {
    /// The next node to visit or [None] if the iterator passed the start node (is at its end).
    next: Option<VisitEvent>,

    /// The start node. Necessary to know when to stop iterating.
    start: JsSyntaxNode,
}

impl PostorderIterator {
    fn new(start: JsAnyBinaryLikeExpression) -> Self {
        Self {
            start: start.syntax().clone(),
            next: Some(VisitEvent::Enter(start)),
        }
    }
}

impl Iterator for PostorderIterator {
    type Item = JsAnyBinaryLikeExpression;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.next.take()? {
                VisitEvent::Enter(binary) => {
                    let left_expression = binary
                        .left()
                        .ok()
                        .and_then(|left| left.as_expression().cloned())
                        .and_then(|left| JsAnyBinaryLikeExpression::cast(left.syntax().clone()));

                    if let Some(expression) = left_expression {
                        self.next = Some(VisitEvent::Enter(expression));
                    } else {
                        // If left is missing or it isn't a binary like expression, then format it as part of the parent binary like expression
                        self.next = Some(VisitEvent::Exit(binary));
                    }
                }
                VisitEvent::Exit(node) => {
                    if node.syntax() != &self.start {
                        self.next = node.syntax().parent().map(|parent| {
                            // SAFETY: Calling `unwrap` here is safe because the iterator only enters (traverses into) a node
                            // if it is a valid binary like expression.
                            let expression = JsAnyBinaryLikeExpression::cast(parent).unwrap();
                            VisitEvent::Exit(expression)
                        });
                    }

                    return Some(node);
                }
            }
        }
    }
}

impl FusedIterator for PostorderIterator {}

declare_node_union! {
    pub(crate) JsAnyBinaryLikeExpression = JsLogicalExpression | JsBinaryExpression | JsInstanceofExpression | JsInExpression
}

impl JsAnyBinaryLikeExpression {
    pub fn left(&self) -> SyntaxResult<JsAnyBinaryLikeLeftExpression> {
        match self {
            JsAnyBinaryLikeExpression::JsLogicalExpression(logical) => logical
                .left()
                .map(JsAnyBinaryLikeLeftExpression::JsAnyExpression),
            JsAnyBinaryLikeExpression::JsBinaryExpression(binary) => binary
                .left()
                .map(JsAnyBinaryLikeLeftExpression::JsAnyExpression),
            JsAnyBinaryLikeExpression::JsInstanceofExpression(instanceof) => instanceof
                .left()
                .map(JsAnyBinaryLikeLeftExpression::JsAnyExpression),
            JsAnyBinaryLikeExpression::JsInExpression(in_expression) => in_expression
                .property()
                .map(JsAnyBinaryLikeLeftExpression::from),
        }
    }

    fn operator_token(&self) -> SyntaxResult<JsSyntaxToken> {
        match self {
            JsAnyBinaryLikeExpression::JsLogicalExpression(logical) => logical.operator_token(),
            JsAnyBinaryLikeExpression::JsBinaryExpression(binary) => binary.operator_token(),
            JsAnyBinaryLikeExpression::JsInstanceofExpression(instanceof) => {
                instanceof.instanceof_token()
            }
            JsAnyBinaryLikeExpression::JsInExpression(in_expression) => in_expression.in_token(),
        }
    }

    pub(crate) fn operator(&self) -> SyntaxResult<BinaryLikeOperator> {
        match self {
            JsAnyBinaryLikeExpression::JsLogicalExpression(logical) => {
                logical.operator().map(BinaryLikeOperator::Logical)
            }
            JsAnyBinaryLikeExpression::JsBinaryExpression(binary) => {
                binary.operator().map(BinaryLikeOperator::Binary)
            }
            JsAnyBinaryLikeExpression::JsInstanceofExpression(_) => {
                Ok(BinaryLikeOperator::Instanceof)
            }
            JsAnyBinaryLikeExpression::JsInExpression(_) => Ok(BinaryLikeOperator::In),
        }
    }

    pub fn right(&self) -> SyntaxResult<JsAnyExpression> {
        match self {
            JsAnyBinaryLikeExpression::JsLogicalExpression(logical) => logical.right(),
            JsAnyBinaryLikeExpression::JsBinaryExpression(binary) => binary.right(),
            JsAnyBinaryLikeExpression::JsInstanceofExpression(instanceof) => instanceof.right(),
            JsAnyBinaryLikeExpression::JsInExpression(in_expression) => in_expression.object(),
        }
    }

    fn into_expression(self) -> JsAnyExpression {
        match self {
            JsAnyBinaryLikeExpression::JsLogicalExpression(logical) => {
                JsAnyExpression::JsLogicalExpression(logical)
            }
            JsAnyBinaryLikeExpression::JsBinaryExpression(binary) => {
                JsAnyExpression::JsBinaryExpression(binary)
            }
            JsAnyBinaryLikeExpression::JsInstanceofExpression(instanceof) => {
                JsAnyExpression::JsInstanceofExpression(instanceof)
            }
            JsAnyBinaryLikeExpression::JsInExpression(in_expression) => {
                JsAnyExpression::JsInExpression(in_expression)
            }
        }
    }

    /// Determines if a binary like expression should be inline or not.
    pub fn should_inline(&self) -> bool {
        match self {
            JsAnyBinaryLikeExpression::JsLogicalExpression(logical_expression) => {
                match logical_expression.right() {
                    Ok(JsAnyExpression::JsObjectExpression(object_expression)) => {
                        object_expression.members().iter().count() > 0
                    }
                    Ok(JsAnyExpression::JsArrayExpression(array_expression)) => {
                        array_expression.elements().iter().count() > 0
                    }
                    _ => false,
                }
            }
            _ => false,
        }
    }
    /// Determines if the left expression can be flattened together with this expression.
    ///
    /// Returns `false` if the left hand side isn't a binary like expression and otherwise.
    ///
    /// Delegates to [should_flatten] for all other expressions.
    fn can_flatten(&self) -> SyntaxResult<bool> {
        let left = self.left()?;

        let result = if let Some(left) = JsAnyBinaryLikeExpression::cast(left.into_syntax()) {
            should_flatten(self.operator()?, left.operator()?)
        } else {
            false
        };

        Ok(result)
    }
}

impl From<JsAnyBinaryLikeExpression> for JsAnyExpression {
    fn from(binary_like: JsAnyBinaryLikeExpression) -> Self {
        match binary_like {
            JsAnyBinaryLikeExpression::JsLogicalExpression(logical) => logical.into(),
            JsAnyBinaryLikeExpression::JsBinaryExpression(binary) => binary.into(),
            JsAnyBinaryLikeExpression::JsInstanceofExpression(instanceof) => instanceof.into(),
            JsAnyBinaryLikeExpression::JsInExpression(in_expression) => in_expression.into(),
        }
    }
}

/// Returns `true` if a binary expression nested into another binary expression should be flattened together.
///
/// This is generally the case if both operator have the same precedence. See the inline comments for the exceptions
/// to that rule.
fn should_flatten(parent_operator: BinaryLikeOperator, child_operator: BinaryLikeOperator) -> bool {
    let parent_precedence = parent_operator.precedence();
    let child_precedence = child_operator.precedence();

    #[allow(clippy::if_same_then_else, clippy::needless_bool)]
    if parent_precedence != child_precedence {
        false
    }
    // `**` is right associative.
    else if parent_precedence.is_exponential() {
        false
    }
    // avoid `a == b == c` -> `(a == b) == c`
    else if parent_precedence.is_equality() && child_precedence.is_equality() {
        false
    }
    // `a % b * c` -> `(a % b) * c`
    // `a * b % c` -> `(a * b) % c`
    else if (child_operator.is_remainder() && parent_precedence.is_multiplicative())
        || (parent_operator.is_remainder() && child_precedence.is_multiplicative())
    {
        false
    }
    // `a * b / c` -> `(a * b) / c`
    else if child_operator != parent_operator
        && child_precedence.is_multiplicative()
        && parent_precedence.is_multiplicative()
    {
        false
    }
    // `a >> b >> c` -> `(a >> b) >> c`
    else if parent_precedence.is_shift() && child_precedence.is_shift() {
        false
    } else {
        true
    }
}

declare_node_union! {
    pub(crate) JsAnyBinaryLikeLeftExpression = JsAnyExpression | JsPrivateName
}

impl JsAnyBinaryLikeLeftExpression {
    fn as_expression(&self) -> Option<&JsAnyExpression> {
        match self {
            JsAnyBinaryLikeLeftExpression::JsAnyExpression(expression) => Some(expression),
            JsAnyBinaryLikeLeftExpression::JsPrivateName(_) => None,
        }
    }
}

impl Format<JsFormatContext> for JsAnyBinaryLikeLeftExpression {
    fn fmt(&self, f: &mut JsFormatter) -> FormatResult<()> {
        match self {
            JsAnyBinaryLikeLeftExpression::JsAnyExpression(expression) => {
                write![f, [expression.format()]]
            }
            JsAnyBinaryLikeLeftExpression::JsPrivateName(private_name) => {
                write![f, [private_name.format()]]
            }
        }
    }
}

impl From<JsAnyInProperty> for JsAnyBinaryLikeLeftExpression {
    fn from(property: JsAnyInProperty) -> Self {
        match property {
            JsAnyInProperty::JsAnyExpression(expression) => {
                JsAnyBinaryLikeLeftExpression::JsAnyExpression(expression)
            }
            JsAnyInProperty::JsPrivateName(private_name) => {
                JsAnyBinaryLikeLeftExpression::JsPrivateName(private_name)
            }
        }
    }
}
