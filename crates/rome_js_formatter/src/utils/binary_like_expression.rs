use crate::prelude::*;
use rome_formatter::{format_args, write, Buffer, VecBuffer};

use rome_js_syntax::{
    JsAnyExpression, JsAnyInProperty, JsBinaryExpression, JsBinaryOperator, JsInExpression,
    JsInstanceofExpression, JsLanguage, JsLogicalExpression, JsLogicalOperator, JsPrivateName,
    JsSyntaxKind, JsSyntaxKind::*, JsSyntaxNode, JsSyntaxToken,
};

use rome_rowan::{AstNode, SyntaxResult};
use std::cmp::Ordering;
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
enum BinaryLikeOperator {
    Logical(JsLogicalOperator),
    Binary(JsBinaryOperator),
    Instanceof,
    In,
}

/// This function is in charge of formatting a node inside a binaryish expression with parenthesis or not
///
/// At the moment this logic is applied only to logical expressions.
///
/// A logical expressions should be decorated with parenthesis only if its previous operation has a lower
/// precedence.
///
/// For example:
///
/// ```ignore
/// foo && bar || lorem
/// ```
///
/// The logical expression `foo && bar` has higher precedence of `bar || lorem`. This means that
/// first `foo && bar` is computed and its result is then computed against `|| lorem`.
///
/// In order to make this distinction more obvious, we wrap `foo && bar` in parenthesis.
fn needs_parens(
    parent_operator: BinaryLikeOperator,
    node: &JsAnyBinaryLikeLeftExpression,
) -> FormatResult<bool> {
    let compare_to = match node {
        JsAnyBinaryLikeLeftExpression::JsAnyExpression(expression) => match expression {
            JsAnyExpression::JsLogicalExpression(logical) => {
                Some(BinaryLikeOperator::Logical(logical.operator()?))
            }
            JsAnyExpression::JsBinaryExpression(binary) => {
                Some(BinaryLikeOperator::Binary(binary.operator()?))
            }
            JsAnyExpression::JsInstanceofExpression(_) => Some(BinaryLikeOperator::Instanceof),
            JsAnyExpression::JsInExpression(_) => Some(BinaryLikeOperator::In),
            _ => None,
        },
        _ => None,
    };

    let result = if let Some(compare_to) = compare_to {
        match (parent_operator, compare_to) {
            (
                BinaryLikeOperator::Logical(previous_operation),
                BinaryLikeOperator::Logical(compare_to),
            ) => compare_to > previous_operation,

            (
                BinaryLikeOperator::Binary(previous_operation),
                BinaryLikeOperator::Binary(compare_to),
            ) => compare_to.compare_precedence(&previous_operation) == Ordering::Greater,
            // `instanceof` operator has higher precedence than `in` operator, so we apply parenthesis here
            (BinaryLikeOperator::In, BinaryLikeOperator::Instanceof) => true,
            // any other case where we have `instanceof` or `in` on the right, we apply parenthesis
            (_, BinaryLikeOperator::Instanceof) | (_, BinaryLikeOperator::In) => true,
            _ => false,
        }
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
        if needs_parens(parent_operator, sub_expression)? {
            write!(f, [format_parenthesized(sub_expression)])
        } else {
            write!(f, [sub_expression])
        }
    })
}

fn format_parenthesized<'a, Inner>(inner: Inner) -> impl Format<JsFormatContext>
where
    Inner: Format<JsFormatContext> + 'a,
{
    format_with(move |f| {
        let mut buffer = VecBuffer::new(f.state_mut());
        write!(buffer, [inner])?;
        let formatted_node = buffer.into_element();
        let (leading, content, trailing) = formatted_node.split_trivia();

        f.write_element(leading)?;
        write![
            f,
            [group_elements(&format_args![
                token("("),
                soft_block_indent(&format_once(|f| {
                    f.write_element(content)?;
                    f.write_element(trailing)
                })),
                token(")")
            ])]
        ]
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
fn should_not_indent_if_parent_indents(current_node: &JsSyntaxNode) -> bool {
    let parent_kind = current_node.parent().map(|parent| parent.kind());

    matches!(
        parent_kind,
        Some(JsSyntaxKind::JS_RETURN_STATEMENT | JsSyntaxKind::JS_ARROW_FUNCTION_EXPRESSION)
    )
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
        let left_parenthesized = needs_parens(operator, &left)?;
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
            Comments::NoComments,
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

#[derive(Debug)]
enum Comments {
    WithComments,
    NoComments,
}

impl From<&Comments> for bool {
    fn from(comments: &Comments) -> Self {
        match comments {
            Comments::WithComments => true,
            Comments::NoComments => false,
        }
    }
}
impl From<bool> for Comments {
    fn from(b: bool) -> Self {
        match b {
            true => Comments::WithComments,
            false => Comments::NoComments,
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

                write!(f, [format_sub_expression(parent.operator()?, &right)])?;
                Ok(())
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
                                write!(f, [space_token(), operator.format()])?;
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
                        f.join_with(space_token()).entries(groups).finish()
                    } else if is_inside_parenthesis(current.syntax()) {
                        f.join_with(soft_line_break_or_space())
                            .entries(groups)
                            .finish()
                    } else if should_not_indent_if_parent_indents(current.syntax()) {
                        write!(
                            f,
                            [group_elements(&format_once(|f| {
                                f.join_with(soft_line_break_or_space())
                                    .entries(groups)
                                    .finish()
                            }))]
                        )
                    } else if should_indent_if_parent_inlines(current) {
                        write!(
                            f,
                            [soft_line_indent_or_space(&group_elements(&format_once(
                                |f| {
                                    f.join_with(soft_line_break_or_space())
                                        .entries(groups)
                                        .finish()
                                }
                            )))]
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
                            [group_elements(&soft_line_indent_or_space(&format_once(
                                |f| {
                                    f.join_with(soft_line_break_or_space())
                                        .entries(groups)
                                        .finish()
                                }
                            )))]
                        )
                    }
                });

                if *parenthesized {
                    write!(f, [format_parenthesized(content)])
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
    comments: Comments,
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
        comments: Comments,
    ) -> Self {
        Self {
            expression,
            operator,
            terminator: TrailingTerminator::None,
            comments,
        }
    }

    fn has_comments(&self) -> bool {
        matches!(self.comments, Comments::WithComments)
    }

    fn with_terminator(mut self, terminator: TrailingTerminator) -> Self {
        self.terminator = terminator;
        self
    }

    fn with_comments<I: Into<Comments>>(mut self, comments: I) -> Self {
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

#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone)]
pub(crate) enum JsAnyBinaryLikeExpression {
    JsLogicalExpression(JsLogicalExpression),
    JsBinaryExpression(JsBinaryExpression),
    JsInstanceofExpression(JsInstanceofExpression),
    JsInExpression(JsInExpression),
}

impl JsAnyBinaryLikeExpression {
    fn left(&self) -> SyntaxResult<JsAnyBinaryLikeLeftExpression> {
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

    fn operator(&self) -> SyntaxResult<BinaryLikeOperator> {
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

    fn right(&self) -> SyntaxResult<JsAnyExpression> {
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
}

impl AstNode for JsAnyBinaryLikeExpression {
    type Language = JsLanguage;
    fn can_cast(kind: JsSyntaxKind) -> bool
    where
        Self: Sized,
    {
        matches!(
            kind,
            JS_BINARY_EXPRESSION
                | JS_LOGICAL_EXPRESSION
                | JS_INSTANCEOF_EXPRESSION
                | JS_IN_EXPRESSION
        )
    }

    fn cast(syntax: JsSyntaxNode) -> Option<Self>
    where
        Self: Sized,
    {
        match syntax.kind() {
            JS_BINARY_EXPRESSION => {
                JsBinaryExpression::cast(syntax).map(JsAnyBinaryLikeExpression::JsBinaryExpression)
            }
            JS_LOGICAL_EXPRESSION => JsLogicalExpression::cast(syntax)
                .map(JsAnyBinaryLikeExpression::JsLogicalExpression),
            JS_INSTANCEOF_EXPRESSION => JsInstanceofExpression::cast(syntax)
                .map(JsAnyBinaryLikeExpression::JsInstanceofExpression),
            JS_IN_EXPRESSION => {
                JsInExpression::cast(syntax).map(JsAnyBinaryLikeExpression::JsInExpression)
            }
            _ => None,
        }
    }

    fn syntax(&self) -> &JsSyntaxNode {
        match self {
            JsAnyBinaryLikeExpression::JsLogicalExpression(logical) => logical.syntax(),
            JsAnyBinaryLikeExpression::JsBinaryExpression(binary) => binary.syntax(),
            JsAnyBinaryLikeExpression::JsInstanceofExpression(instanceof) => instanceof.syntax(),
            JsAnyBinaryLikeExpression::JsInExpression(in_expression) => in_expression.syntax(),
        }
    }

    fn into_syntax(self) -> JsSyntaxNode {
        match self {
            JsAnyBinaryLikeExpression::JsLogicalExpression(logical) => logical.into_syntax(),
            JsAnyBinaryLikeExpression::JsBinaryExpression(binary) => binary.into_syntax(),
            JsAnyBinaryLikeExpression::JsInstanceofExpression(instanceof) => {
                instanceof.into_syntax()
            }
            JsAnyBinaryLikeExpression::JsInExpression(in_expression) => in_expression.into_syntax(),
        }
    }
}

impl JsAnyBinaryLikeExpression {
    /// Determines if a binary like expression should be flattened or not. As a rule of thumb, an expression
    /// can be flattened if it is of the same kind as the left-hand side sub-expression and uses the same operator.
    fn can_flatten(&self) -> SyntaxResult<bool> {
        Ok(match self {
            JsAnyBinaryLikeExpression::JsLogicalExpression(logical) => match logical.left()? {
                JsAnyExpression::JsLogicalExpression(left) => {
                    left.operator()? == logical.operator()?
                }
                _ => false,
            },
            JsAnyBinaryLikeExpression::JsBinaryExpression(binary) => match binary.left()? {
                JsAnyExpression::JsBinaryExpression(left) => {
                    left.operator()? == binary.operator()?
                }
                _ => false,
            },
            JsAnyBinaryLikeExpression::JsInstanceofExpression(instance_of) => {
                matches!(
                    instance_of.left()?,
                    JsAnyExpression::JsInstanceofExpression(_)
                )
            }
            JsAnyBinaryLikeExpression::JsInExpression(in_expression) => matches!(
                in_expression.property()?,
                JsAnyInProperty::JsAnyExpression(JsAnyExpression::JsInExpression(_))
            ),
        })
    }
}

#[derive(Debug)]
enum JsAnyBinaryLikeLeftExpression {
    JsAnyExpression(JsAnyExpression),
    JsPrivateName(JsPrivateName),
}

impl JsAnyBinaryLikeLeftExpression {
    fn as_expression(&self) -> Option<&JsAnyExpression> {
        match self {
            JsAnyBinaryLikeLeftExpression::JsAnyExpression(expression) => Some(expression),
            JsAnyBinaryLikeLeftExpression::JsPrivateName(_) => None,
        }
    }
}

impl AstNode for JsAnyBinaryLikeLeftExpression {
    type Language = JsLanguage;
    fn can_cast(kind: JsSyntaxKind) -> bool
    where
        Self: Sized,
    {
        JsAnyExpression::can_cast(kind) || JsPrivateName::can_cast(kind)
    }

    fn cast(syntax: JsSyntaxNode) -> Option<Self>
    where
        Self: Sized,
    {
        if syntax.kind() == JS_PRIVATE_NAME {
            JsPrivateName::cast(syntax).map(|name| name.into())
        } else {
            JsAnyExpression::cast(syntax).map(|expr| expr.into())
        }
    }

    fn syntax(&self) -> &JsSyntaxNode {
        match self {
            JsAnyBinaryLikeLeftExpression::JsAnyExpression(expression) => expression.syntax(),
            JsAnyBinaryLikeLeftExpression::JsPrivateName(private_name) => private_name.syntax(),
        }
    }

    fn into_syntax(self) -> JsSyntaxNode {
        match self {
            JsAnyBinaryLikeLeftExpression::JsAnyExpression(expression) => expression.into_syntax(),
            JsAnyBinaryLikeLeftExpression::JsPrivateName(private_name) => {
                private_name.into_syntax()
            }
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

impl From<JsAnyExpression> for JsAnyBinaryLikeLeftExpression {
    fn from(expression: JsAnyExpression) -> Self {
        JsAnyBinaryLikeLeftExpression::JsAnyExpression(expression)
    }
}

impl From<JsPrivateName> for JsAnyBinaryLikeLeftExpression {
    fn from(private_name: JsPrivateName) -> Self {
        JsAnyBinaryLikeLeftExpression::JsPrivateName(private_name)
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
