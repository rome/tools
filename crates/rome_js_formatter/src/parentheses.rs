use crate::utils::{
    resolve_expression_syntax, JsAnyBinaryLikeExpression, JsAnyBinaryLikeLeftExpression,
};
use rome_js_syntax::{
    JsAnyExpression, JsAnyFunctionBody, JsAnyLiteralExpression, JsArrowFunctionExpression,
    JsBinaryExpression, JsBinaryOperator, JsComputedMemberExpression, JsConditionalExpression,
    JsLanguage, JsParenthesizedExpression, JsSequenceExpression, JsSyntaxKind, JsSyntaxNode,
    JsTemplate,
};
use rome_rowan::AstNode;

/// Node that may be parenthesized to ensure it forms valid syntax or to improve readability
pub trait NeedsParentheses: AstNode<Language = JsLanguage> {
    fn needs_parentheses(&self) -> bool {
        self.resolve_parent()
            .map_or(false, |parent| self.needs_parentheses_with_parent(&parent))
    }

    fn resolve_parent(&self) -> Option<JsSyntaxNode> {
        self.syntax().ancestors().skip(1).find(|parent| {
            !matches!(
                parent.kind(),
                JsSyntaxKind::JS_PARENTHESIZED_EXPRESSION
                    | JsSyntaxKind::JS_PARENTHESIZED_ASSIGNMENT
                    | JsSyntaxKind::TS_PARENTHESIZED_TYPE
            )
        })
    }

    /// Returns `true` if this node requires parentheses to form valid syntax or improve readability.
    ///
    /// Returns `false` if the parentheses can be omitted safely without changing semantics.
    fn needs_parentheses_with_parent(&self, parent: &JsSyntaxNode) -> bool;
}

/// Resolves the parent of the node that is not a parenthesized expression
pub(crate) fn resolve_expression_parent(node: &JsSyntaxNode) -> Option<JsSyntaxNode> {
    node.ancestors()
        .skip(1)
        .find(|parent| !JsParenthesizedExpression::can_cast(parent.kind()))
}

impl NeedsParentheses for JsAnyLiteralExpression {
    fn needs_parentheses(&self) -> bool {
        match self {
            JsAnyLiteralExpression::JsBigIntLiteralExpression(big_int) => {
                big_int.needs_parentheses()
            }
            JsAnyLiteralExpression::JsBooleanLiteralExpression(boolean) => {
                boolean.needs_parentheses()
            }
            JsAnyLiteralExpression::JsNullLiteralExpression(null_literal) => {
                null_literal.needs_parentheses()
            }
            JsAnyLiteralExpression::JsNumberLiteralExpression(number_literal) => {
                number_literal.needs_parentheses()
            }
            JsAnyLiteralExpression::JsRegexLiteralExpression(regex) => regex.needs_parentheses(),
            JsAnyLiteralExpression::JsStringLiteralExpression(string) => string.needs_parentheses(),
        }
    }

    fn needs_parentheses_with_parent(&self, parent: &JsSyntaxNode) -> bool {
        match self {
            JsAnyLiteralExpression::JsBigIntLiteralExpression(big_int) => {
                big_int.needs_parentheses_with_parent(parent)
            }
            JsAnyLiteralExpression::JsBooleanLiteralExpression(boolean) => {
                boolean.needs_parentheses_with_parent(parent)
            }
            JsAnyLiteralExpression::JsNullLiteralExpression(null_literal) => {
                null_literal.needs_parentheses_with_parent(parent)
            }
            JsAnyLiteralExpression::JsNumberLiteralExpression(number_literal) => {
                number_literal.needs_parentheses_with_parent(parent)
            }
            JsAnyLiteralExpression::JsRegexLiteralExpression(regex) => {
                regex.needs_parentheses_with_parent(parent)
            }
            JsAnyLiteralExpression::JsStringLiteralExpression(string) => {
                string.needs_parentheses_with_parent(parent)
            }
        }
    }
}

/// Returns the left most expression of `expression`.
///
/// For example, returns `a` for `(a ? b : c) + d` because it first resolves the
/// left hand expression of the binary expression, then resolves to the inner expression of the parenthesized
/// expression, and finally resolves to the test condition of the conditional expression.
pub(crate) fn resolve_left_most_expression(
    expression: &JsAnyExpression,
) -> JsAnyBinaryLikeLeftExpression {
    let mut current: JsAnyExpression = expression.clone();

    while let Some(left) = get_expression_left_side(&current) {
        match left {
            JsAnyBinaryLikeLeftExpression::JsAnyExpression(expression) => {
                current = expression;
            }
            left => {
                return left;
            }
        }
    }

    current.into()
}

/// Returns the left side of an expression (an expression where the first child is a `Node` or [None]
/// if the expression has no left side.
pub(crate) fn get_expression_left_side(
    expression: &JsAnyExpression,
) -> Option<JsAnyBinaryLikeLeftExpression> {
    use JsAnyExpression::*;

    let left_expression = match expression {
        JsParenthesizedExpression(parenthesized) => parenthesized.expression().ok(),
        JsSequenceExpression(sequence) => sequence.left().ok(),
        JsStaticMemberExpression(member) => member.object().ok(),
        JsComputedMemberExpression(member) => member.object().ok(),
        JsTemplate(template) => template.tag().map(|tag| tag),
        JsNewExpression(new) => new.callee().ok(),
        JsCallExpression(call) => call.callee().ok(),
        JsConditionalExpression(conditional) => conditional.test().ok(),
        TsAsExpression(as_expression) => as_expression.expression().ok(),
        TsNonNullAssertionExpression(non_null) => non_null.expression().ok(),
        expression => {
            return JsAnyBinaryLikeExpression::cast(expression.syntax().clone())
                .and_then(|binary_like| binary_like.left().ok());
        }
    };

    left_expression.map(|left| left.into())
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub(crate) enum FirstInStatementMode {
    /// Only considers [JsExpressionStatement] as a statement
    ExpressionStatementOnly,

    /// Considers [JsExpressionStatement] and the body of [JsArrowFunctionExpression] as the first statement.
    ExpressionStatementOrArrow,

    /// Considers [JsExpressionStatement] and [JsExportDefaultExpressionClause] as the first statement.
    ExpressionOrExportDefault,
}

/// Returns `true` if this node is at the start of an expression (depends on the passed `mode`).
///
/// Traverses upwards the tree for as long as the `node` is the left most expression until the node isn't
/// the left most node or reached a statement.
pub(crate) fn is_first_in_statement(node: &JsSyntaxNode, mode: FirstInStatementMode) -> bool {
    debug_assert_is_expression(node);

    let mut current = node.clone();

    while let Some(parent) = current.parent() {
        let parent = match parent.kind() {
            JsSyntaxKind::JS_EXPRESSION_STATEMENT => {
                return true;
            }

            JsSyntaxKind::JS_PARENTHESIZED_EXPRESSION
            | JsSyntaxKind::JS_STATIC_MEMBER_EXPRESSION
            | JsSyntaxKind::JS_TEMPLATE
            | JsSyntaxKind::JS_CALL_EXPRESSION
            | JsSyntaxKind::JS_NEW_EXPRESSION
            | JsSyntaxKind::TS_AS_EXPRESSION
            | JsSyntaxKind::TS_NON_NULL_ASSERTION_EXPRESSION => parent,
            JsSyntaxKind::JS_SEQUENCE_EXPRESSION => {
                let sequence = JsSequenceExpression::unwrap_cast(parent);

                let is_left = sequence.left().map(AstNode::into_syntax).as_ref() == Ok(&current);

                if is_left {
                    sequence.into_syntax()
                } else {
                    break;
                }
            }

            JsSyntaxKind::JS_COMPUTED_MEMBER_EXPRESSION => {
                let member_expression = JsComputedMemberExpression::unwrap_cast(parent);

                let is_object = member_expression
                    .object()
                    .map(AstNode::into_syntax)
                    .as_ref()
                    == Ok(&current);

                if is_object {
                    member_expression.into_syntax()
                } else {
                    break;
                }
            }

            JsSyntaxKind::JS_CONDITIONAL_EXPRESSION => {
                let conditional = JsConditionalExpression::unwrap_cast(parent);

                if conditional.test().map(AstNode::into_syntax).as_ref() == Ok(&current) {
                    conditional.into_syntax()
                } else {
                    break;
                }
            }

            JsSyntaxKind::JS_ARROW_FUNCTION_EXPRESSION
                if mode == FirstInStatementMode::ExpressionStatementOrArrow =>
            {
                let arrow = JsArrowFunctionExpression::unwrap_cast(parent);

                let is_body = arrow.body().map_or(false, |body| match body {
                    JsAnyFunctionBody::JsAnyExpression(expression) => {
                        expression.syntax() == &current
                    }
                    _ => false,
                });

                if is_body {
                    return true;
                }

                break;
            }

            JsSyntaxKind::JS_EXPORT_DEFAULT_EXPRESSION_CLAUSE
                if mode == FirstInStatementMode::ExpressionOrExportDefault =>
            {
                return true;
            }

            kind if JsAnyBinaryLikeExpression::can_cast(kind) => {
                let binary_like = JsAnyBinaryLikeExpression::unwrap_cast(parent);

                let is_left = binary_like.left().map_or(false, |left| match left {
                    JsAnyBinaryLikeLeftExpression::JsAnyExpression(expression) => {
                        expression.syntax() == &current
                    }
                    _ => false,
                });

                if is_left {
                    binary_like.into_syntax()
                } else {
                    break;
                }
            }
            _ => break,
        };

        current = parent;
    }

    false
}

/// Returns `true` if the `expression` is in a position where only [`MemberExpression`s](https://tc39.es/ecma262/#prod-MemberExpression) are allowed.
pub(crate) fn is_in_member_expression_position(
    expression: &JsSyntaxNode,
    parent: &JsSyntaxNode,
) -> bool {
    debug_assert_is_expression(expression);

    match parent.kind() {
        JsSyntaxKind::TS_NON_NULL_ASSERTION_EXPRESSION => true,

        _ => {
            is_callee(expression, parent)
                || is_member_object(expression, parent)
                || is_tag(expression, parent)
        }
    }
}

pub(crate) fn update_expression_needs_parentheses(
    parent: &JsSyntaxNode,
    expression: &JsSyntaxNode,
) -> bool {
    debug_assert!(matches!(
        expression.kind(),
        JsSyntaxKind::JS_PRE_UPDATE_EXPRESSION
            | JsSyntaxKind::JS_POST_UPDATE_EXPRESSION
            | JsSyntaxKind::JS_UNARY_EXPRESSION
    ));

    match parent.kind() {
        JsSyntaxKind::JS_BINARY_EXPRESSION => {
            let binary = JsBinaryExpression::unwrap_cast(parent.clone());

            matches!(binary.operator(), Ok(JsBinaryOperator::Exponent))
                && binary.left().map(resolve_expression_syntax).as_ref() == Ok(expression)
        }
        _ => is_in_left_hand_side_position(expression, parent),
    }
}

/// Returns `true` if the expression is in a position where only [`LeftHandSideExpression`s](https://tc39.es/ecma262/#prod-LeftHandSideExpression) are allowed.
pub(crate) fn is_in_left_hand_side_position(
    expression: &JsSyntaxNode,
    parent: &JsSyntaxNode,
) -> bool {
    match parent.kind() {
        JsSyntaxKind::JS_EXTENDS_CLAUSE => true,
        _ => is_in_member_expression_position(expression, parent),
    }
}

pub(crate) fn is_in_assignment_expression_position(
    parent: &JsSyntaxNode,
    _expression: &JsSyntaxNode,
) -> bool {
    matches!(parent.kind(), JsSyntaxKind::JS_SPREAD)
}

pub(crate) fn unary_expression_needs_parentheses(
    expression: &JsSyntaxNode,
    parent: &JsSyntaxNode,
) -> bool {
    is_in_left_hand_side_position(expression, parent)
        || is_in_assignment_expression_position(parent, expression)
}

/// Returns `true` if `node< is the `object` of a [JsStaticMemberExpression] or [JsComputedMemberExpression]
pub(crate) fn is_member_object(node: &JsSyntaxNode, parent: &JsSyntaxNode) -> bool {
    debug_assert_is_expression(node);

    match parent.kind() {
        // Only allows expression in the <object
        JsSyntaxKind::JS_STATIC_MEMBER_EXPRESSION => true,
        JsSyntaxKind::JS_COMPUTED_MEMBER_EXPRESSION => {
            let member_expression = JsComputedMemberExpression::unwrap_cast(parent.clone());

            member_expression
                .object()
                .map(resolve_expression_syntax)
                .as_ref()
                == Ok(node)
        }
        _ => false,
    }
}

/// Returns `true` if `node` is the `callee` of a [JsNewExpression] or [JsCallExpression].
pub(crate) fn is_callee(node: &JsSyntaxNode, parent: &JsSyntaxNode) -> bool {
    debug_assert_is_expression(node);

    // It isn't necessary to test if the node is the `callee` because the nodes only
    // allow expressions in the `callee` position;
    matches!(
        parent.kind(),
        JsSyntaxKind::JS_CALL_EXPRESSION | JsSyntaxKind::JS_NEW_EXPRESSION
    )
}

/// Returns `true` if `node` is the `test` of a [JsConditionalExpression].
///
/// # Examples
///
/// ```text
/// is_conditional_test(`a`, `a ? b : c`) -> true
/// is_conditional_test(`b`, `a ? b : c`) -> false
/// ```
pub(crate) fn is_conditional_test(node: &JsSyntaxNode, parent: &JsSyntaxNode) -> bool {
    match parent.kind() {
        JsSyntaxKind::JS_CONDITIONAL_EXPRESSION => {
            let conditional = JsConditionalExpression::unwrap_cast(parent.clone());

            conditional.test().map(resolve_expression_syntax).as_ref() == Ok(node)
        }
        _ => false,
    }
}

/// Returns `true` if `node` is the `tag` of a [JsTemplate] expression
pub(crate) fn is_tag(node: &JsSyntaxNode, parent: &JsSyntaxNode) -> bool {
    match parent.kind() {
        JsSyntaxKind::JS_TEMPLATE => {
            let template = JsTemplate::unwrap_cast(parent.clone());

            template.tag().map(resolve_expression_syntax).as_ref() == Some(node)
        }
        _ => false,
    }
}

pub(crate) fn is_binary_like_left_or_right(node: &JsSyntaxNode, parent: &JsSyntaxNode) -> bool {
    debug_assert_is_expression(node);

    JsAnyBinaryLikeExpression::can_cast(parent.kind())
}

fn debug_assert_is_expression(node: &JsSyntaxNode) {
    debug_assert!(
        JsAnyExpression::can_cast(node.kind()),
        "Expected {node:#?} to be an expression."
    )
}

#[cfg(test)]
mod tests {

    #[macro_export]
    macro_rules! assert_needs_parentheses {
        ($input:expr, $Node:ident) => {{
            $crate::assert_needs_parentheses!($input, $Node, rome_js_syntax::SourceType::ts())
        }};

        ($input:expr, $Node:ident, $source_type: expr) => {{
            use rome_rowan::AstNode;
            let parse = rome_js_parser::parse($input, 0, $source_type);

            let diagnostics = parse.diagnostics();
            assert!(diagnostics.is_empty(), "Expected input program to not have syntax errors but had {diagnostics:?}");

            let root = parse.syntax();
            let matching_nodes: Vec<_> = root.descendants().filter_map($Node::cast).collect();

            let node = match matching_nodes.len() {
                0 => {
                    panic!(
                        "Expected to find a '{}' node in '{}' but found none.",
                        core::any::type_name::<$Node>(),
                        $input
                    )
                }
                1 => matching_nodes.into_iter().next().unwrap(),
                _ => {
                    panic!("Expected to find a single node matching '{}' in '{}' but found multiple ones\n: {matching_nodes:#?}", core::any::type_name::<$Node>(), $input);
                }
            };

            assert!(node.needs_parentheses())
        }};
    }

    #[macro_export]
    macro_rules! assert_not_needs_parentheses {
        ($input:expr, $Node:ident) => {{
            $crate::assert_not_needs_parentheses!($input, $Node, rome_js_syntax::SourceType::ts())
        }};

        ($input:expr, $Node:ident, $source_type: expr) => {{
            use rome_rowan::AstNode;
            let parse = rome_js_parser::parse($input, 0, $source_type);

            let diagnostics = parse.diagnostics();
            assert!(diagnostics.is_empty(), "Expected input program to not have syntax errors but had {diagnostics:?}");

            let root = parse.syntax();
            let matching_nodes: Vec<_> = root.descendants().filter_map($Node::cast).collect();

            let node = match matching_nodes.len() {
                0 => {
                    panic!(
                        "Expected to find a '{}' node in '{}' but found none.",
                        core::any::type_name::<$Node>(),
                        $input
                    )
                }
                1 => matching_nodes.into_iter().next().unwrap(),
                _ => {
                    panic!("Expected to find a single node matching '{}' in '{}' but found multiple ones\n: {matching_nodes:#?}", core::any::type_name::<$Node>(), $input);
                }
            };

            assert!(!node.needs_parentheses())
        }};
    }
}
