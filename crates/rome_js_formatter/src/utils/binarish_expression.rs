use crate::formatter_traits::{FormatOptionalTokenAndNode, FormatTokenAndNode};
use crate::{
    empty_element, format_elements, group_elements, hard_group_elements, hard_line_break,
    join_elements, soft_block_indent, soft_line_break_or_space, soft_line_indent_or_space,
    space_token, token, FormatElement, FormatResult, Formatter, ToFormatElement,
};
use rome_js_syntax::{
    AstNode, JsAnyExpression, JsAnyInProperty, JsBinaryExpression, JsBinaryExpressionFields,
    JsBinaryOperator, JsInExpressionFields, JsInstanceofExpression, JsInstanceofExpressionFields,
    JsLogicalExpression, JsLogicalExpressionFields, JsLogicalOperator, JsSyntaxKind, SyntaxNode,
    SyntaxNodeExt, SyntaxToken,
};
use std::cmp::Ordering;
use std::fmt::Debug;

/// This function is charge to flat binaryish expressions that have the same precedence of their operators
///
/// This means that expressions like `some && thing && elsewhere` are entitled to fall in the same group.
///
/// Instead, if we encounter something like `some && thing  || elsewhere && thing`, we will creat two groups:
/// `[some, thing]` and `[elsewhere, thing]`, each group will be grouped altogether.
///
///
/// Let's take for example:
///
/// ```ignore
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
/// ```ignore
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
/// The flattening of the groups is done with recursion, during the recursions we want to be careful of:
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
/// ```ignore
/// some &&
/// thing &&
/// ```
///
/// So what we do is to "borrow" (no Rust reference) the operator "&&" that belongs to the "parent" -
/// or, if want to see it from a recursion point of view, the previous node that we visited -
/// in our case `elsewhere &&`. We then take it's operator token and pass it down.
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
///
pub fn format_binaryish_expression(
    expression: &JsAnyExpression,
    formatter: &Formatter,
) -> FormatResult<FormatElement> {
    let mut flatten_nodes = FlattenItems::new(expression.syntax().clone(), formatter);

    flatten_expressions(&mut flatten_nodes, expression, formatter, None)?;
    flatten_nodes.into_format_element()
}

// this function is responsible to resource the tree and flatten logical/binary expressions
// that have the same operator
fn flatten_expressions(
    flatten_items: &mut FlattenItems,
    expression: &JsAnyExpression,
    formatter: &Formatter,
    parent_operator: Option<SyntaxToken>,
) -> FormatResult<()> {
    match expression {
        JsAnyExpression::JsBinaryExpression(binary_expression) => {
            let JsBinaryExpressionFields {
                left,
                right,
                operator_token,
            } = binary_expression.as_fields();
            let should_flatten = should_flatten_binary_expression(&binary_expression)?;

            let payload = BinaryLikeExpression {
                left: left?,
                right: right?,
                operator_token: operator_token?,
                parent_operator,
                operator: BinarishOperator::Binary(binary_expression.operator()?),
            };

            if should_flatten {
                flatten_items.push_flattened_binary_like_expression(payload)?;
            } else {
                flatten_items.push_binary_like_expression(payload)?;
            }
        }
        JsAnyExpression::JsLogicalExpression(logical_expression) => {
            let JsLogicalExpressionFields {
                left,
                right,
                operator_token,
            } = logical_expression.as_fields();

            let should_flatten = should_flatten_logical_expression(&logical_expression)?;
            let payload = BinaryLikeExpression {
                left: left?,
                operator_token: operator_token?,
                operator: BinarishOperator::Logical(logical_expression.operator()?),
                right: right?,
                parent_operator,
            };

            if should_flatten {
                flatten_items.push_flattened_binary_like_expression(payload)?;
            } else {
                flatten_items.push_binary_like_expression(payload)?;
            }
        }
        JsAnyExpression::JsInstanceofExpression(instanceof_expression) => {
            let JsInstanceofExpressionFields {
                left,
                right,
                instanceof_token,
            } = instanceof_expression.as_fields();

            let should_flatten = should_flatten_instanceof_expression(&instanceof_expression)?;
            let payload = BinaryLikeExpression {
                left: left?,
                operator_token: instanceof_token?,
                operator: BinarishOperator::Instanceof,
                right: right?,
                parent_operator,
            };

            if should_flatten {
                flatten_items.push_flattened_binary_like_expression(payload)?;
            } else {
                flatten_items.push_binary_like_expression(payload)?;
            }
        }
        JsAnyExpression::JsInExpression(in_expression) => {
            let JsInExpressionFields {
                property,
                in_token,
                object,
            } = in_expression.as_fields();

            let property = property?;

            if let JsAnyInProperty::JsAnyExpression(JsAnyExpression::JsInExpression(
                in_expression,
            )) = property.clone()
            {
                flatten_items.push_flattened_binary_like_expression(BinaryLikeExpression {
                    left: JsAnyExpression::JsInExpression(in_expression),
                    operator_token: in_token?,
                    operator: BinarishOperator::In,
                    right: object?,
                    parent_operator,
                })?;
            } else {
                flatten_items.push_binary_like_expression(BinaryLikeExpression {
                    left: property,
                    operator_token: in_token?,
                    operator: BinarishOperator::In,
                    right: object?,
                    parent_operator,
                })?;
            }
        }
        _ => {
            let (formatted, has_comments) = if let Some(parent_operator) = parent_operator {
                let formatted = format_elements![
                    expression.to_format_element(formatter)?,
                    space_token(),
                    parent_operator.format(formatter)?
                ];

                (
                    formatted,
                    parent_operator.has_leading_comments()
                        || parent_operator.has_trailing_comments(),
                )
            } else {
                (
                    expression.to_format_element(formatter)?,
                    expression.syntax().contains_comments(),
                )
            };

            flatten_items
                .items
                .push(FlattenItem::other(formatted, has_comments.into()));
        }
    }

    Ok(())
}

/// A binary expression can be "flatten" until we have binary expressions with the same operator.
///
/// Here we check, given a binary expression node, if its `left` field is a binary expression and its operator
/// is the same.
///
/// For example, given this code:
/// ```ignore
///  lorem - ipsum + dolor
/// ```
///
/// We will flatten until `lorem - ipsum`
fn should_flatten_binary_expression(node: &JsBinaryExpression) -> FormatResult<bool> {
    let JsBinaryExpressionFields { left, .. } = node.as_fields();

    let should_flatten = match left? {
        JsAnyExpression::JsBinaryExpression(binary_expression) => {
            node.operator()? == binary_expression.operator()?
        }

        _ => false,
    };

    Ok(should_flatten)
}

/// A logical expression can be "flatten" until we have logical expressions with the same operator
///
/// Here we check, given a logical expression node, if its `left` field is a logical expression and its operator
/// is the same.
///
/// For example, given this code:
/// ```ignore
///  lorem && ipsum || dolor
/// ```
///
/// We will flatten until `lorem && ipsum`
fn should_flatten_logical_expression(node: &JsLogicalExpression) -> FormatResult<bool> {
    let JsLogicalExpressionFields { left, .. } = node.as_fields();

    let should_flatten = match left? {
        JsAnyExpression::JsLogicalExpression(logical_expression) => {
            node.operator()? == logical_expression.operator()?
        }

        _ => false,
    };

    Ok(should_flatten)
}

/// The `JsInstanceofExpression` should be flatten if its left hand side is also a `JsInstanceofExpression`
fn should_flatten_instanceof_expression(node: &JsInstanceofExpression) -> FormatResult<bool> {
    let JsInstanceofExpressionFields { left, .. } = node.as_fields();

    Ok(matches!(left?, JsAnyExpression::JsInstanceofExpression(_)))
}

/// Small wrapper to identify the operation of an expression and deduce their precedence
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
enum BinarishOperator {
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
    parent_operator: BinarishOperator,
    node: &SyntaxNode,
    formatted_node: FormatElement,
) -> FormatResult<(FormatElement, bool)> {
    let compare_to = match JsAnyExpression::cast(node.clone()) {
        Some(JsAnyExpression::JsLogicalExpression(logical)) => {
            Some(BinarishOperator::Logical(logical.operator()?))
        }
        Some(JsAnyExpression::JsBinaryExpression(binary)) => {
            Some(BinarishOperator::Binary(binary.operator()?))
        }
        Some(JsAnyExpression::JsInstanceofExpression(_)) => Some(BinarishOperator::Instanceof),
        Some(JsAnyExpression::JsInExpression(_)) => Some(BinarishOperator::In),
        _ => None,
    };

    let operation_is_higher = if let Some(compare_to) = compare_to {
        match (parent_operator, compare_to) {
            (
                BinarishOperator::Logical(previous_operation),
                BinarishOperator::Logical(compare_to),
            ) => compare_to > previous_operation,

            (
                BinarishOperator::Binary(previous_operation),
                BinarishOperator::Binary(compare_to),
            ) => compare_to.compare_precedence(&previous_operation) == Ordering::Greater,
            // `instanceof` operator has higher precedence than `in` operator, so we apply parenthesis here
            (BinarishOperator::In, BinarishOperator::Instanceof) => true,
            // any other case where we have `instanceof` or `in` on the right, we apply parenthesis
            (_, BinarishOperator::Instanceof) | (_, BinarishOperator::In) => true,
            _ => false,
        }
    } else {
        false
    };

    let result = if operation_is_higher {
        let formatted = if node.contains_comments() {
            let (leading, content, trailing) = formatted_node.split_trivia();
            format_elements![
                leading,
                group_elements(format_elements![
                    token("("),
                    soft_block_indent(format_elements![content, trailing]),
                    token(")")
                ])
            ]
        } else {
            group_elements(format_elements![
                token("("),
                soft_block_indent(formatted_node),
                token(")"),
            ])
        };
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

fn is_inside_parenthesis(current_node: &SyntaxNode) -> bool {
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
fn should_not_indent_if_parent_indents(current_node: &SyntaxNode) -> bool {
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
fn should_indent_if_parent_inlines(current_node: &SyntaxNode) -> bool {
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

#[derive(Debug)]
struct FlattenItems<'f> {
    current_node: SyntaxNode,
    items: Vec<FlattenItem>,
    formatter: &'f Formatter,
}

struct BinaryLikeExpression<Left: AstNode + ToFormatElement + Clone> {
    /// Left hand side of the expression
    left: Left,
    /// The operator of the expression
    operator_token: SyntaxToken,
    /// Right hand side of the expression
    right: JsAnyExpression,
    /// The operation that belongs to the current node
    operator: BinarishOperator,
    parent_operator: Option<SyntaxToken>,
}

impl<'f> FlattenItems<'f> {
    pub fn new(current_node: SyntaxNode, formatter: &'f Formatter) -> Self {
        Self {
            current_node,
            items: Vec::new(),
            formatter,
        }
    }

    /// Generic function that used to create a [FlattenItem] out of a binaryish expression:
    ///
    /// Nodes that fit the requirements are:
    /// - `JsLogicalExpression`
    /// - `JsBinaryExpression`
    /// - `JsInstanceofExpression`
    /// - `JsInExpression`
    fn push_flattened_binary_like_expression(
        &mut self,
        payload: BinaryLikeExpression<JsAnyExpression>,
    ) -> FormatResult<()> {
        let BinaryLikeExpression {
            left,
            right,
            operator_token,
            operator,
            parent_operator,
        } = payload;

        flatten_expressions(self, &left, self.formatter, Some(operator_token))?;

        // Call lazily by storing the right syntax node instead?
        let has_comments = right.syntax().contains_comments();
        let right_formatted = right.format(self.formatter)?;

        let (formatted_node, _) =
            format_with_or_without_parenthesis(operator, right.syntax(), right_formatted)?;
        let operator_element = parent_operator.format_or_empty(self.formatter)?;

        let formatted = if operator_element.is_empty() {
            formatted_node
        } else {
            format_elements![formatted_node, space_token(), operator_element]
        };

        let flatten_item = FlattenItem::right(formatted, has_comments.into());

        self.items.push(flatten_item);

        Ok(())
    }

    fn push_binary_like_expression<Left>(
        &mut self,
        payload: BinaryLikeExpression<Left>,
    ) -> FormatResult<()>
    where
        Left: AstNode + ToFormatElement + Clone,
    {
        let BinaryLikeExpression {
            left,
            operator_token,
            operator,
            right,
            parent_operator,
        } = payload;

        // Format the left hand sie of the binarish expression
        let (left_node_formatted, _) = format_with_or_without_parenthesis(
            operator,
            left.syntax(),
            left.format(self.formatter)?,
        )?;

        let operator_has_trailing_comments = operator_token.has_trailing_comments();
        let formatted_left = format_elements![
            left_node_formatted,
            space_token(),
            operator_token.format(self.formatter)?,
            if operator_has_trailing_comments {
                hard_line_break()
            } else {
                empty_element()
            },
        ];
        let left_item = FlattenItem::other(formatted_left, operator_has_trailing_comments.into());

        // Format the parent operator
        let (formatted_parent_operator, parent_operator_has_comments) =
            if let Some(parent_operator) = parent_operator {
                let previous_operator_has_trailing_comments =
                    parent_operator.has_trailing_comments();
                (
                    format_elements![
                        space_token(),
                        parent_operator.format(self.formatter)?,
                        if previous_operator_has_trailing_comments {
                            hard_line_break()
                        } else {
                            empty_element()
                        }
                    ],
                    // Here we care only about trailing comments that belong to the previous operator
                    previous_operator_has_trailing_comments || right.syntax().contains_comments(),
                )
            } else {
                (
                    empty_element(),
                    // Here we want to check only leading comments;
                    // trailing comments will be added after the end of the whole expression.
                    // We want to handle cases like `lorem && (3 + 5 == 9) // comment`.
                    // This part is a signal to the formatter to tell it if the whole expression should break.
                    right.syntax().has_leading_comments(),
                )
            };

        // Format the right node
        let (right_node_formatted, parenthesized) = format_with_or_without_parenthesis(
            operator,
            right.syntax(),
            format_binaryish_expression(&right, self.formatter)?,
        )?;
        let formatted_right = format_elements![right_node_formatted, formatted_parent_operator];

        let right_item =
            if !parenthesized && matches!(right, JsAnyExpression::JsLogicalExpression(_)) {
                FlattenItem::group(formatted_right, parent_operator_has_comments.into())
            } else {
                FlattenItem::right(formatted_right, parent_operator_has_comments.into())
            };

        self.items.push(left_item);
        self.items.push(right_item);

        Ok(())
    }

    fn into_format_element(self) -> FormatResult<FormatElement> {
        let can_hard_group = can_hard_group(&self.items);
        let len = self.items.len();

        let mut groups = self
            .items
            .into_iter()
            .enumerate()
            // groups not like ["something &&", "something &&" ]
            // we want to add a space between them in case they don't break
            .map(|(index, element)| {
                let element: FormatElement = element.into();
                // the last element doesn't need a space
                if index + 1 == len {
                    element
                } else {
                    format_elements![element, space_token()]
                }
            });

        if can_hard_group {
            // we bail early if group doesn't need to be broken. We don't need to do further checks
            return Ok(hard_group_elements(join_elements(space_token(), groups)));
        }

        let formatted = if is_inside_parenthesis(&self.current_node) {
            join_elements(soft_line_break_or_space(), groups)
        } else if should_not_indent_if_parent_indents(&self.current_node) {
            group_elements(join_elements(soft_line_break_or_space(), groups))
        } else if should_indent_if_parent_inlines(&self.current_node) {
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

            format_elements![
                hard_group_elements(head),
                group_elements(soft_line_indent_or_space(rest))
            ]
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
    comments: Comments,
}

impl FlattenItem {
    fn right(formatted: FormatElement, comments: Comments) -> Self {
        Self {
            formatted,
            kind: FlattenItemKind::Right,
            comments,
        }
    }

    fn other(formatted: FormatElement, comments: Comments) -> Self {
        Self {
            formatted,
            kind: FlattenItemKind::Other,
            comments,
        }
    }

    fn group(formatted: FormatElement, comments: Comments) -> Self {
        Self {
            formatted,
            comments,
            kind: FlattenItemKind::Group,
        }
    }

    fn is_group(&self) -> bool {
        matches!(self.kind, FlattenItemKind::Group)
    }

    fn has_comments(&self) -> bool {
        matches!(self.comments, Comments::WithComments)
    }
}

#[derive(Debug)]
enum FlattenItemKind {
    /// The right hand side of a binary expression. May have a trailing operator if this is a nested binarish expression.
    Right,
    /// Used when the right side of a binary/logical expression is another binary/logical.
    /// When we have such cases we
    Group,
    /// nodes that don't need any special handling
    Other,
}

impl From<FlattenItem> for FormatElement {
    fn from(item: FlattenItem) -> Self {
        item.formatted
    }
}
