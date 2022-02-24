use crate::formatter_traits::{FormatOptionalTokenAndNode, FormatTokenAndNode};
use crate::{
    concat_elements, empty_element, format_elements, group_elements, hard_group_elements,
    hard_line_break, if_group_breaks, if_group_fits_on_single_line, indent, join_elements,
    soft_line_break_or_space, space_token, FormatElement, FormatResult, Formatter, ToFormatElement,
};
use rslint_parser::ast::{
    JsAnyExpression, JsBinaryExpression, JsBinaryExpressionFields, JsLogicalExpression,
    JsLogicalExpressionFields,
};
use rslint_parser::{
    parse_expression, AstNode, AstToken, JsSyntaxKind, SyntaxNode, SyntaxNodeExt, SyntaxToken,
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
    let mut flatten_nodes = Vec::new();

    flatten_expressions(&mut flatten_nodes, syntax_node.to_owned(), formatter, None)?;

    dbg!(&flatten_nodes);
    let should_break = flatten_nodes.len() >= 3
        || flatten_nodes
            .first()
            .map_or_else(|| false, |node| node.has_comments());

    let groups = flatten_nodes
        .into_iter()
        .map(|item| item.into())
        .collect::<Vec<FormatElement>>();

    if should_break {
        Ok(group_elements(format_elements![
            if_group_fits_on_single_line(join_elements(space_token(), groups.clone())),
            // Sometimes a chain of logical/binary expressions is inside a `JsParenthesizedExpression`.
            // If so, we can can't force the parent to correctly apply the indentation.
            // Because of that, if the group breaks, we manually add an indentation.
            if_group_breaks(format_elements![indent(format_elements![
                hard_line_break(),
                join_elements(soft_line_break_or_space(), groups,),
            ]),])
        ]))
    } else {
        Ok(hard_group_elements(join_elements(
            space_token(),
            groups.clone(),
        )))
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

enum FlattenItem {
    Binary(JsBinaryExpression, FlattenItemFormatted, bool),
    Logical(JsLogicalExpression, FlattenItemFormatted, bool),
    // nodes that don't need any special handling
    Node(SyntaxNode, FormatElement, bool),
}

impl FlattenItem {
    pub fn has_comments(&self) -> bool {
        match self {
            FlattenItem::Binary(_, _, has_comments) => *has_comments,
            FlattenItem::Logical(_, _, has_comments) => *has_comments,
            FlattenItem::Node(_, _, has_comments) => *has_comments,
        }
    }
}

impl Debug for FlattenItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FlattenItem::Logical(_, formatted, has_comments) => {
                write!(
                    f,
                    "LogicalExpression: {:?} - {:?}\nHas comments: {}",
                    formatted.node_element, formatted.operator_element, has_comments
                )
            }
            FlattenItem::Binary(_, formatted, has_comments) => {
                write!(
                    f,
                    "BinaryExpression: {:?} - {:?}\nHas comments: {}",
                    formatted.node_element, formatted.operator_element, has_comments
                )
            }
            FlattenItem::Node(_, formatted, has_comments) => {
                write!(
                    f,
                    "Any other node: {:?}\nHas comments: {}",
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
        }
    }
}

// this function is responsible to resource the tree and flatten logical/binary expressions
// that have the same operator
fn flatten_expressions(
    parts: &mut Vec<FlattenItem>,
    syntax_node: SyntaxNode,
    formatter: &Formatter,
    previous_operator: Option<SyntaxToken>,
) -> FormatResult<()> {
    match syntax_node.kind() {
        JsSyntaxKind::JS_BINARY_EXPRESSION => {
            let binary_expression = JsBinaryExpression::cast(syntax_node.to_owned()).unwrap();
            let JsBinaryExpressionFields {
                left,
                right,
                operator,
            } = binary_expression.as_fields();

            // In order to flatten the expression, we have to check if the left node
            // is a binary expression with the same operator.
            // If the two operators are not the same, we stop the flattening.
            if should_flatten_binary_expression(&binary_expression)? {
                let right = right?;
                let operator = operator?;
                let has_comments =
                    right.syntax().contains_comments() || operator.has_trailing_comments();

                dbg!(has_comments);
                flatten_expressions(parts, left?.syntax().to_owned(), formatter, Some(operator))?;
                parts.push(FlattenItem::Binary(
                    binary_expression,
                    FlattenItemFormatted {
                        node_element: right.format(formatter)?,
                        operator_element: previous_operator.format_or_empty(formatter)?,
                    },
                    has_comments,
                ))
            } else {
                let operator = operator?;
                let (left_item, right_item) =
                    split_binaryish_to_flatten_items(SplitToElementParams {
                        formatter,
                        left: left?,
                        previous_operator,
                        right: right?,
                        operator,
                    })?;
                parts.push(left_item);
                parts.push(right_item);
            };
        }
        JsSyntaxKind::JS_LOGICAL_EXPRESSION => {
            let logical_expression = JsLogicalExpression::cast(syntax_node.to_owned()).unwrap();

            let JsLogicalExpressionFields {
                left,
                right,
                operator,
            } = logical_expression.as_fields();

            // In order to flatten the expression, we have to check if the left node
            // is a binary expression with the same operator.
            // If the two operators are not the same, we stop the flattening.
            if should_flatten_logical_expression(&logical_expression)? {
                let right = right?;
                let operator = operator?;
                let has_comments =
                    right.syntax().contains_comments() || operator.has_trailing_comments();

                dbg!(has_comments, &right.syntax());
                flatten_expressions(parts, left?.syntax().to_owned(), formatter, Some(operator))?;
                parts.push(FlattenItem::Logical(
                    logical_expression,
                    FlattenItemFormatted {
                        node_element: right.format(formatter)?,
                        operator_element: previous_operator.format_or_empty(formatter)?,
                    },
                    has_comments,
                ));
            } else {
                let (left_item, right_item) =
                    split_binaryish_to_flatten_items(SplitToElementParams {
                        formatter,
                        left: left?,
                        previous_operator,
                        right: right?,
                        operator: operator?,
                    })?;
                parts.push(left_item);
                parts.push(right_item);
            }
        }
        _ => {
            let has_comments = syntax_node.contains_comments();
            let formatted = syntax_node.to_format_element(formatter)?;
            let formatted = previous_operator.map_or_else(
                || syntax_node.to_format_element(formatter),
                |operator| {
                    Ok(format_elements![
                        formatted,
                        space_token(),
                        operator.format(formatter)?
                    ])
                },
            )?;
            parts.push(FlattenItem::Node(syntax_node, formatted, has_comments));
        }
    }

    Ok(())
}

/// A binary expression can be "flatten" until we have binary expressions with the same operator.
///
/// Here we check, given a binary expression node, if its `left` field is a binary expression and its operator
/// is the same
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
/// is the same
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
struct SplitToElementParams<'a, Node: AstNode + FormatTokenAndNode> {
    formatter: &'a Formatter,
    left: Node,
    right: Node,
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
fn split_binaryish_to_flatten_items<Node: AstNode + FormatTokenAndNode>(
    params: SplitToElementParams<Node>,
) -> FormatResult<(FlattenItem, FlattenItem)> {
    let SplitToElementParams {
        formatter,
        left,
        operator,
        previous_operator,
        right,
    } = params;
    let formatted_left = concat_elements([
        left.format(formatter)?,
        space_token(),
        operator.format(formatter)?,
    ]);
    let left_item = FlattenItem::Node(
        left.syntax().to_owned(),
        formatted_left,
        operator.has_trailing_comments(),
    );

    let (previous_operator, has_comments) = if let Some(previous_operator) = previous_operator {
        (
            format_elements![space_token(), previous_operator.format(formatter)?],
            previous_operator.has_trailing_comments(),
        )
    } else {
        dbg!("right {}", right.syntax().contains_comments());
        (empty_element(), right.syntax().contains_comments())
    };

    let formatted_right = concat_elements([right.format(formatter)?, previous_operator]);
    let right_item = FlattenItem::Node(right.syntax().to_owned(), formatted_right, has_comments);

    Ok((left_item, right_item))
}
