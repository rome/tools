use crate::formatter_traits::{FormatOptionalTokenAndNode, FormatTokenAndNode};
use crate::{
    concat_elements, empty_element, format_elements, group_elements, hard_group_elements,
    hard_line_break, if_group_breaks, if_group_fits_on_single_line, indent, join_elements,
    soft_line_break_or_space, space_token, FormatElement, FormatResult, Formatter, ToFormatElement,
};
use rslint_syntax::{AstNode, JsSyntaxKind, SyntaxNode, SyntaxNodeExt, SyntaxToken};
use rslint_syntax::{
    JsAnyExpression, JsBinaryExpression, JsBinaryExpressionFields, JsLogicalExpression,
    JsLogicalExpressionFields,
};
use std::fmt::Debug;

/// This function is charge to flat binaryish expressions that have the same precedence of their operators
///
/// This means that expressions like `some && thing && elsewhere` are entitled to fall in the same group.
///
/// Instead, if we encounter something like `some && thing  || elsewhere && thing`, we will great two groups:
/// `[some, thing]` and `[elsewhere, thing]`, each group will be grouped altogether.
///
///
/// Let's take for example:
///
/// ```ignore
/// some && thing && elsewhere && happy
/// ```
///
/// These expressions have a nested  nodes, which is roughly something like this:
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
    syntax_node: &SyntaxNode,
    formatter: &Formatter,
) -> FormatResult<FormatElement> {
    let mut flatten_nodes = FlattenItems::new(syntax_node.clone());

    flatten_expressions(&mut flatten_nodes, syntax_node.clone(), formatter, None)?;
    flatten_nodes.into_format_element()
}

// this function is responsible to resource the tree and flatten logical/binary expressions
// that have the same operator
fn flatten_expressions(
    flatten_items: &mut FlattenItems,
    syntax_node: SyntaxNode,
    formatter: &Formatter,
    previous_operator: Option<SyntaxToken>,
) -> FormatResult<()> {
    match syntax_node.kind() {
        JsSyntaxKind::JS_BINARY_EXPRESSION => {
            let binary_expression = JsBinaryExpression::cast(syntax_node).unwrap();
            let JsBinaryExpressionFields {
                left,
                right,
                operator,
            } = binary_expression.as_fields();
            let right = right?;
            let operator = operator?;
            let left = left?;
            let has_comments =
                right.syntax().contains_comments() || operator.has_trailing_comments();

            // In order to flatten the expression, we have to check if the left node
            // is a binary expression with the same operator.
            // If the two operators are not the same, we stop the flattening.
            if should_flatten_binary_expression(&binary_expression)? {
                flatten_expressions(
                    flatten_items,
                    left.syntax().clone(),
                    formatter,
                    Some(operator),
                )?;
                flatten_items.items.push(FlattenItem::Binary(
                    binary_expression,
                    FlattenItemFormatted {
                        node_element: right.format(formatter)?,
                        operator_element: previous_operator.format_or_empty(formatter)?,
                    },
                    has_comments.into(),
                ))
            } else {
                let (left_item, right_item) =
                    split_binaryish_to_flatten_items(SplitToElementParams {
                        formatter,
                        left,
                        previous_operator,
                        right,
                        operator,
                    })?;
                flatten_items.items.push(left_item);
                flatten_items.items.push(right_item);
            };
        }
        JsSyntaxKind::JS_LOGICAL_EXPRESSION => {
            let logical_expression = JsLogicalExpression::cast(syntax_node).unwrap();

            let JsLogicalExpressionFields {
                left,
                right,
                operator,
            } = logical_expression.as_fields();
            let right = right?;
            let operator = operator?;
            let left = left?;
            let has_comments =
                right.syntax().contains_comments() || operator.has_trailing_comments();

            // In order to flatten the expression, we have to check if the left node
            // is a binary expression with the same operator.
            // If the two operators are not the same, we stop the flattening.
            if should_flatten_logical_expression(&logical_expression)? {
                flatten_expressions(
                    flatten_items,
                    left.syntax().clone(),
                    formatter,
                    Some(operator),
                )?;
                flatten_items.items.push(FlattenItem::Logical(
                    logical_expression,
                    FlattenItemFormatted {
                        node_element: right.format(formatter)?,
                        operator_element: previous_operator.format_or_empty(formatter)?,
                    },
                    has_comments.into(),
                ));
            } else {
                let (left_item, right_item) =
                    split_binaryish_to_flatten_items(SplitToElementParams {
                        formatter,
                        left,
                        previous_operator,
                        right,
                        operator,
                    })?;
                flatten_items.items.push(left_item);
                flatten_items.items.push(right_item);
            }
        }
        _ => {
            let (formatted, has_comments) = if let Some(previous_operator) = previous_operator {
                let formatted = format_elements![
                    syntax_node.to_format_element(formatter)?,
                    space_token(),
                    previous_operator.format(formatter)?
                ];
                (
                    formatted,
                    previous_operator.has_leading_comments()
                        || previous_operator.has_trailing_comments(),
                )
            } else {
                (
                    syntax_node.to_format_element(formatter)?,
                    syntax_node.contains_comments(),
                )
            };

            flatten_items.items.push(FlattenItem::Node(
                syntax_node,
                formatted,
                has_comments.into(),
            ));
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
            node.operator_kind()? == binary_expression.operator_kind()?
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
            node.operator_kind()? == logical_expression.operator_kind()?
        }

        _ => false,
    };

    Ok(should_flatten)
}

/// Parameters needed for [split_node_to_flatten_items].
///
/// Check the documentation of  [split_node_to_flatten_items] for a better explanation of the payload
struct SplitToElementParams<'a> {
    formatter: &'a Formatter,
    left: JsAnyExpression,
    right: JsAnyExpression,
    operator: SyntaxToken,
    previous_operator: Option<SyntaxToken>,
}

/// This function is usually code on the last binary/logical expression of the stack.
/// When called, it takes left and right and creates two [FlattenItem] with their relative ad-hoc format element
///
/// For example, given following example:
///
/// ```ignore
/// (this.something && this.else && this.is_false && this.is_true)
/// ```
/// This function will be called on the node `this.something && this.else`.
///
/// As this function will be called on nodes that are logical or binary expressions, the parameters will be:
/// - `left` expression
/// - `right` expression
/// - `operator` token
/// - the `previous_operator`, which is - in this example - is the operator before `&& this.is_false`
///
/// Having all these nodes allows us to create two flatten items that will be formatted like this:
/// `[ `left`, `&&` ]` and `[ `right`, `&&` ]`.
///
/// Doing so will us to correctly maintain the formatting of the whole algorithm.
fn split_binaryish_to_flatten_items(
    params: SplitToElementParams,
) -> FormatResult<(FlattenItem, FlattenItem)> {
    let SplitToElementParams {
        formatter,
        left,
        operator,
        previous_operator,
        right,
    } = params;

    let right_kind = &right.syntax().kind();
    let right_expression_should_group = right_expression_should_group(right_kind);

    let formatted_left = concat_elements([
        left.format(formatter)?,
        space_token(),
        operator.format(formatter)?,
    ]);
    let left_item = FlattenItem::Node(
        left.syntax().clone(),
        formatted_left,
        operator.has_trailing_comments().into(),
    );

    let (previous_operator, has_comments) = if let Some(previous_operator) = previous_operator {
        (
            format_elements![space_token(), previous_operator.format(formatter)?],
            // Here we care only about trailing comments that belong to the previous operator
            previous_operator.has_trailing_comments() || right.syntax().contains_comments(),
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

    // here we handle cases where the `right` part of a binary/logical expression should be as its
    // own group.
    // An example is `true && true || false && false`, there the `right` branch of the first
    // logical expression (`||`) is another logical expression.
    // In that case, we call `format_binaryish_expression` from scratch, with its own flatten items.
    let right_item = if right_expression_should_group {
        let formatted = format_binaryish_expression(right.syntax(), formatter)?;
        FlattenItem::Group(formatted, has_comments.into())
    } else {
        let formatted_right = concat_elements([right.format(formatter)?, previous_operator]);
        FlattenItem::Node(right.syntax().clone(), formatted_right, has_comments.into())
    };

    Ok((left_item, right_item))
}

/// This function tells the algorithm if the right part should be a separated group
fn right_expression_should_group(right_kind: &JsSyntaxKind) -> bool {
    matches!(right_kind, JsSyntaxKind::JS_LOGICAL_EXPRESSION)
}

/// Dirty a quick check if the group can potentially break
fn should_break(flatten_nodes: &[FlattenItem]) -> bool {
    // We don't want to have 1 + 2 to break, for example.
    // If there are any trailing comments, let's break.
    flatten_nodes.len() > 2
        || flatten_nodes
            .iter()
            .any(|node| node.has_comments() || matches!(node, FlattenItem::Group(..)))
}

fn is_inside_parenthesis(current_node: &SyntaxNode) -> bool {
    current_node.parent().map_or(false, |parent| {
        let kind = parent.kind();
        matches!(
            kind,
            JsSyntaxKind::JS_IF_STATEMENT
                | JsSyntaxKind::JS_DO_WHILE_STATEMENT
                | JsSyntaxKind::JS_WHILE_STATEMENT
                | JsSyntaxKind::JS_SWITCH_STATEMENT
        )
    })
}

/// This function checks whether the chain of logical/binary expressions **should not** be indented
///
/// There are some cases where the indentation is done by the parent, so if the parent is already doing
/// the indentation, then there's no need to do a second indentation.
fn should_not_indent_if_parent_indents(current_node: &SyntaxNode) -> bool {
    let parent = current_node.parent();
    let grand_parent = parent.as_ref().map_or_else(|| None, |p| p.parent());

    match (parent, grand_parent) {
        (Some(parent), _) => {
            matches!(parent.kind(), JsSyntaxKind::JS_RETURN_STATEMENT)
                || matches!(parent.kind(), JsSyntaxKind::JS_ARROW_FUNCTION_EXPRESSION)
        }
        _ => false,
    }
}

/// There are other cases where the parent decides to inline the the element; in
/// these cases the decide to actually break on a new line and indent it.
///
/// This function checks what the parents adheres to this behaviour
fn should_indent_if_parent_inlines(current_node: &SyntaxNode) -> bool {
    let parent = current_node.parent();
    let grand_parent = parent.as_ref().map_or_else(|| None, |p| p.parent());

    match (parent, grand_parent) {
        (Some(parent), Some(grand_parent)) => {
            matches!(parent.kind(), JsSyntaxKind::JS_INITIALIZER_CLAUSE)
                && matches!(grand_parent.kind(), JsSyntaxKind::JS_VARIABLE_DECLARATOR)
        }

        _ => false,
    }
}

#[derive(Debug)]
struct FlattenItems {
    pub current_node: SyntaxNode,
    pub items: Vec<FlattenItem>,
}

impl FlattenItems {
    pub fn new(current_node: SyntaxNode) -> Self {
        Self {
            current_node,
            items: Vec::new(),
        }
    }

    pub fn into_format_element(self) -> FormatResult<FormatElement> {
        let should_break = should_break(&self.items);
        let len = self.items.len();

        let mut groups: Vec<FormatElement> = self
            .items
            .into_iter()
            .enumerate()
            // groups not like ["something &&", "something &&" ]
            // we want to add a space between them in case they can't break
            .map(|(index, element)| {
                let element: FormatElement = element.into();
                // the last element doesn't need a space
                if index + 1 == len {
                    element
                } else {
                    format_elements![element, space_token()]
                }
            })
            .collect::<Vec<FormatElement>>();

        if !should_break {
            // we bail early if group doesn't need to be broken. We don't need to do further checks
            return Ok(hard_group_elements(join_elements(
                soft_line_break_or_space(),
                groups,
            )));
        }

        let is_inside_parenthesis = is_inside_parenthesis(&self.current_node);
        let should_not_indent = should_not_indent_if_parent_indents(&self.current_node);
        let should_ident_if_parent_inlines = should_indent_if_parent_inlines(&self.current_node);

        if is_inside_parenthesis {
            Ok(join_elements(soft_line_break_or_space(), groups))
        } else if should_not_indent {
            Ok(group_elements(join_elements(
                soft_line_break_or_space(),
                groups,
            )))
        } else if should_ident_if_parent_inlines {
            // in order to correctly break, we need to check if the parent created a group
            // that breaks or not. In order to do that , we need to create two conditional groups
            // that behave differently depending on the situation
            Ok(format_elements![
                // the parent has created a group that breaks, then we create an indentation
                if_group_breaks(indent(format_elements![
                    hard_line_break(),
                    group_elements(join_elements(soft_line_break_or_space(), groups.clone(),),)
                ])),
                // the group doesn't break, so we just normally group
                if_group_fits_on_single_line(group_elements(join_elements(
                    soft_line_break_or_space(),
                    groups,
                )))
            ])
        } else {
            // if none of the previous conditions is met,
            // we take take out the first element from the rest of group, then we hard group the "head"
            // and we indent the rest of the groups in a new line
            let rest = groups.split_off(1);
            let head = groups;

            Ok(format_elements![
                hard_group_elements(join_elements(soft_line_break_or_space(), head,)),
                group_elements(format_elements![
                    if_group_breaks(indent(format_elements![
                        hard_line_break(),
                        join_elements(soft_line_break_or_space(), rest.clone(),)
                    ],)),
                    if_group_fits_on_single_line(hard_group_elements(join_elements(
                        soft_line_break_or_space(),
                        rest,
                    )),)
                ])
            ])
        }
    }
}

struct FlattenItemFormatted {
    operator_element: FormatElement,
    node_element: FormatElement,
}

impl From<FlattenItemFormatted> for FormatElement {
    fn from(flatten_item_formatted: FlattenItemFormatted) -> Self {
        if flatten_item_formatted.operator_element.is_empty() {
            flatten_item_formatted.node_element
        } else {
            format_elements![
                flatten_item_formatted.node_element,
                space_token(),
                flatten_item_formatted.operator_element
            ]
        }
    }
}

enum WithComments {
    True,
    False,
}

impl From<&WithComments> for bool {
    fn from(w_c: &WithComments) -> Self {
        match w_c {
            WithComments::True => true,
            WithComments::False => false,
        }
    }
}
impl From<bool> for WithComments {
    fn from(b: bool) -> Self {
        match b {
            true => WithComments::True,
            false => WithComments::False,
        }
    }
}

impl Debug for WithComments {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WithComments::True => write!(f, "has comments"),
            WithComments::False => write!(f, "no comments"),
        }
    }
}

///
enum FlattenItem {
    Binary(JsBinaryExpression, FlattenItemFormatted, WithComments),
    Logical(JsLogicalExpression, FlattenItemFormatted, WithComments),

    /// Used when the right side of a binary/logical expression is another binary/logical.
    /// When we have such cases we
    Group(FormatElement, WithComments),

    // nodes that don't need any special handling
    Node(SyntaxNode, FormatElement, WithComments),
}

impl FlattenItem {
    pub fn has_comments(&self) -> bool {
        match self {
            FlattenItem::Binary(_, _, w_c) => w_c.into(),
            FlattenItem::Logical(_, _, w_c) => w_c.into(),
            FlattenItem::Node(_, _, w_c) => w_c.into(),
            FlattenItem::Group(_, w_c) => w_c.into(),
        }
    }
}

impl Debug for FlattenItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FlattenItem::Logical(_, formatted, has_comments) => {
                write!(
                    f,
                    "LogicalExpression: {:?} - {:?}\nHas comments: {:?}",
                    formatted.node_element, formatted.operator_element, has_comments
                )
            }
            FlattenItem::Binary(_, formatted, has_comments) => {
                write!(
                    f,
                    "BinaryExpression: {:?} - {:?}\nHas comments: {:?}",
                    formatted.node_element, formatted.operator_element, has_comments
                )
            }
            FlattenItem::Node(_, formatted, has_comments) => {
                write!(
                    f,
                    "Any other node: {:?}\nHas comments: {:?}",
                    formatted, has_comments
                )
            }
            FlattenItem::Group(formatted, has_comments) => {
                write!(
                    f,
                    "Right group: {:?}\nHas comments: {:?}",
                    formatted, has_comments
                )
            }
        }
    }
}

impl From<FlattenItem> for FormatElement {
    fn from(item: FlattenItem) -> Self {
        match item {
            FlattenItem::Binary(_, formatted, _) => formatted.into(),
            FlattenItem::Logical(_, formatted, _) => formatted.into(),
            FlattenItem::Node(_, element, _) => element,
            FlattenItem::Group(element, _) => element,
        }
    }
}
