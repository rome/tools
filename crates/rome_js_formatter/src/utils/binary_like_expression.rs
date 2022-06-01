use crate::prelude::*;

use rome_js_syntax::{
    JsAnyExpression, JsAnyInProperty, JsBinaryExpression, JsBinaryOperator, JsInExpression,
    JsInstanceofExpression, JsLanguage, JsLogicalExpression, JsLogicalOperator, JsPrivateName,
    JsSyntaxKind, JsSyntaxKind::*, JsSyntaxNode, JsSyntaxToken,
};
use rome_rowan::{AstNode, SyntaxResult};
use std::cmp::Ordering;
use std::fmt::Debug;
use std::iter::FusedIterator;

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
    formatter: &JsFormatter,
) -> FormatResult<FormatElement> {
    let mut flatten_items = FlattenItems::default();
    let current_node = expression.syntax().clone();

    let post_order_binary_like_expressions = PostorderIterator::new(expression);
    let mut left: Option<JsAnyBinaryLikeExpression> = None;

    for parent in post_order_binary_like_expressions {
        let parent_operator = parent.operator_token()?;

        if let Some(left) = left {
            flatten_items.format_binary_expression_right_hand_side(
                left,
                Some(parent_operator),
                formatter,
            )?;
        } else {
            // Leaf binary like expression. Format the left hand side.
            // The right hand side gets formatted when traversing upwards in the tree.

            let left = parent.left()?;

            let has_comments = left.syntax().has_comments_direct();
            let formatted = formatted![formatter, [left]]?;

            flatten_items.items.push(FlattenItem::regular(
                formatted,
                Some(parent_operator),
                has_comments.into(),
            ));
        }

        left = Some(parent);
    }

    // Format the top most binary like expression
    if let Some(root) = left {
        flatten_items.format_binary_expression_right_hand_side(root, None, formatter)?;
    }

    flatten_items.take_format_element(&current_node, formatter)
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
fn format_with_or_without_parenthesis(
    parent_operator: BinaryLikeOperator,
    node: &JsSyntaxNode,
    formatted_node: FormatElement,
    formatter: &JsFormatter,
) -> FormatResult<(FormatElement, bool)> {
    let compare_to = match JsAnyExpression::cast(node.clone()) {
        Some(JsAnyExpression::JsLogicalExpression(logical)) => {
            Some(BinaryLikeOperator::Logical(logical.operator()?))
        }
        Some(JsAnyExpression::JsBinaryExpression(binary)) => {
            Some(BinaryLikeOperator::Binary(binary.operator()?))
        }
        Some(JsAnyExpression::JsInstanceofExpression(_)) => Some(BinaryLikeOperator::Instanceof),
        Some(JsAnyExpression::JsInExpression(_)) => Some(BinaryLikeOperator::In),
        _ => None,
    };

    let operation_is_higher = if let Some(compare_to) = compare_to {
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

    let result = if operation_is_higher {
        let (leading, content, trailing) = formatted_node.split_trivia();
        let formatted = formatted![
            formatter,
            [
                leading,
                group_elements(formatted![
                    formatter,
                    [
                        token("("),
                        soft_block_indent(formatted![formatter, [content, trailing]]?),
                        token(")")
                    ]
                ]?)
            ]
        ]?;

        (formatted, true)
    } else {
        (formatted_node, false)
    };

    Ok(result)
}

/// It tells if the expression can be hard grouped
fn can_hard_group(flatten_nodes: &[FlattenItem]) -> bool {
    // We don't want to have 1 + 2 to break, for example.
    // If there are any trailing comments, let's break.
    flatten_nodes.len() <= 2
        && flatten_nodes
            .iter()
            .all(|node| !node.has_comments() && !node.is_group())
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
fn should_indent_if_parent_inlines(current_node: &JsSyntaxNode) -> bool {
    let parent = current_node.parent();
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
}

impl FlattenItems {
    /// Formats the right hand side of a binary like expression
    fn format_binary_expression_right_hand_side(
        &mut self,
        expression: JsAnyBinaryLikeExpression,
        parent_operator: Option<JsSyntaxToken>,
        formatter: &JsFormatter,
    ) -> FormatResult<()> {
        let should_flatten = expression.can_flatten()?;

        if should_flatten {
            self.flatten_right_hand_side(expression, parent_operator, formatter)
        } else {
            self.format_new_binary_like_group(expression, parent_operator, formatter)
        }
    }

    /// Flattens the right hand operand of a binary like expression.
    fn flatten_right_hand_side(
        &mut self,
        binary_like_expression: JsAnyBinaryLikeExpression,
        parent_operator: Option<JsSyntaxToken>,
        formatter: &JsFormatter,
    ) -> FormatResult<()> {
        let right = binary_like_expression.right()?;
        let has_comments = right.syntax().has_comments_direct();
        let right_formatted = formatted![formatter, [right.format()]]?;

        let (formatted_node, _) = format_with_or_without_parenthesis(
            binary_like_expression.operator()?,
            right.syntax(),
            right_formatted,
            formatter,
        )?;

        let flatten_item =
            FlattenItem::regular(formatted_node, parent_operator, has_comments.into());
        self.items.push(flatten_item);

        Ok(())
    }

    /// The left hand-side expression and the current operator cannot be flattened.
    /// Format the left hand side by its own and potentially wrap it in parentheses before formatting
    /// the right-hand side of the current expression.
    fn format_new_binary_like_group(
        &mut self,
        binary_like_expression: JsAnyBinaryLikeExpression,
        parent_operator: Option<JsSyntaxToken>,
        formatter: &JsFormatter,
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

        let left_formatted = self.take_format_element(left.syntax(), formatter)?;
        let (left_formatted, _) =
            format_with_or_without_parenthesis(operator, left.syntax(), left_formatted, formatter)?;

        let operator_has_trailing_comments = operator_token.has_trailing_comments();
        let mut left_item = FlattenItem::regular(
            left_formatted,
            Some(operator_token),
            operator_has_trailing_comments.into(),
        );

        if operator_has_trailing_comments {
            left_item = left_item.with_terminator(TrailingTerminator::HardLineBreak);
        }

        self.items.push(left_item);

        let right = binary_like_expression.right()?;

        // Format the right node
        let (formatted_right, parenthesized) = format_with_or_without_parenthesis(
            operator,
            right.syntax(),
            formatted![formatter, [right.format()]]?,
            formatter,
        )?;

        let parent_operator_has_comments = parent_operator
            .as_ref()
            .map(|operator| operator.has_leading_comments());

        let mut right_item =
            if !parenthesized && matches!(right, JsAnyExpression::JsLogicalExpression(_)) {
                FlattenItem::group(formatted_right, parent_operator, Comments::NoComments)
            } else {
                FlattenItem::regular(formatted_right, parent_operator, Comments::NoComments)
            };

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

    fn take_format_element(
        &mut self,
        current_node: &JsSyntaxNode,
        formatter: &JsFormatter,
    ) -> FormatResult<FormatElement> {
        let can_hard_group = can_hard_group(&self.items);
        let len = self.items.len();

        let mut groups = self
            .items
            .drain(..)
            .enumerate()
            // groups not like ["something &&", "something &&" ]
            // we want to add a space between them in case they don't break
            .map(|(index, element)| {
                let operator = match &element.operator {
                    Some(operator) => {
                        // SAFETY: `syntax_token.format` never returns MissingToken.
                        formatted![formatter, [space_token(), operator.format()]].unwrap()
                    }
                    None => empty_element(),
                };

                let terminator = match &element.terminator {
                    // the last element doesn't need a space
                    TrailingTerminator::None if index + 1 == len => empty_element(),
                    TrailingTerminator::None => empty_element(),
                    TrailingTerminator::HardLineBreak => hard_line_break(),
                };

                format_elements![element.formatted, operator, terminator]
            });

        if can_hard_group {
            // we bail early if group doesn't need to be broken. We don't need to do further checks
            return Ok(join_elements(space_token(), groups));
        }

        let formatted = if is_inside_parenthesis(current_node) {
            join_elements(soft_line_break_or_space(), groups)
        } else if should_not_indent_if_parent_indents(current_node) {
            group_elements(join_elements(soft_line_break_or_space(), groups))
        } else if should_indent_if_parent_inlines(current_node) {
            // in order to correctly break, we need to check if the parent created a group
            // that breaks or not. In order to do that , we need to create two conditional groups
            // that behave differently depending on the situation
            soft_line_indent_or_space(group_elements(join_elements(
                soft_line_break_or_space(),
                groups,
            )))
        } else {
            // if none of the previous conditions is met,
            // we take take out the first element from the rest of group, then we hard group the "head"
            // and we indent the rest of the groups in a new line
            let head = groups.next().unwrap();
            let rest = join_elements(soft_line_break_or_space(), groups);

            format_elements![head, group_elements(soft_line_indent_or_space(rest))]
        };

        Ok(formatted)
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

#[derive(Debug)]
struct FlattenItem {
    kind: FlattenItemKind,
    formatted: FormatElement,
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
    fn regular(
        formatted: FormatElement,
        operator: Option<JsSyntaxToken>,
        comments: Comments,
    ) -> Self {
        Self {
            formatted,
            operator,
            kind: FlattenItemKind::Regular,
            terminator: TrailingTerminator::None,
            comments,
        }
    }

    fn group(
        formatted: FormatElement,
        operator: Option<JsSyntaxToken>,
        comments: Comments,
    ) -> Self {
        Self {
            formatted,
            comments,
            operator,
            terminator: TrailingTerminator::None,
            kind: FlattenItemKind::Group,
        }
    }

    fn is_group(&self) -> bool {
        matches!(self.kind, FlattenItemKind::Group)
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

#[derive(Debug)]
enum FlattenItemKind {
    Regular,
    /// Used when the right side of a binary/logical expression is another binary/logical.
    /// When we have such cases we
    Group,
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

impl Format for JsAnyBinaryLikeLeftExpression {
    type Context = JsFormatContext;

    fn format(&self, formatter: &JsFormatter) -> FormatResult<FormatElement> {
        match self {
            JsAnyBinaryLikeLeftExpression::JsAnyExpression(expression) => {
                formatted![formatter, [expression.format()]]
            }
            JsAnyBinaryLikeLeftExpression::JsPrivateName(private_name) => {
                formatted![formatter, [private_name.format()]]
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
