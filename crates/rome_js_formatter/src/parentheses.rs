//! JavaScript supports parenthesizing expressions, assignments, and TypeScript types.
//! Parenthesizing an expression can be desired to change the precedence of an expression or to ease
//! readability.
//!
//! Rome is opinionated about which parentheses to keep or where to insert parentheses.
//! It removes parentheses that aren't necessary to keep the same semantics as in the source document, nor aren't improving readability.
//! Rome also inserts parentheses around nodes where we believe that they're helpful to improve readability.
//!
//! The [NeedsParentheses] trait forms the foundation of Rome's parentheses formatting and is implemented
//! by all nodes supporting parentheses (expressions, assignments, and types). The trait's main method
//! is the [NeedsParentheses::needs_parentheses]
//! method that implements the rules when a node requires parentheses.
//! A node requires parentheses to:
//! * improve readability: `a << b << 3` is harder to read than `(a << b) << 3`
//! * form valid syntax: `class A extends 3 + 3 {}` isn't valid, but `class A extends (3 + 3) {}` is
//! * preserve operator precedence: `(a + 3) * 4` has a different meaning than `a + 3 * 4`
//!
//! The challenge of formatting parenthesized nodes is that a tree with parentheses and a tree without
//! parentheses (that have the same semantics) must result in the same output. For example,
//! formatting `(a + 3) + 5` must yield the same formatted output as `a + 3 + 5` or `a + (3 + 5)` or even
//! `(((a + 3) + 5))` even though all these trees differ by the number of parenthesized expressions.
//!
//! There are two measures taken by Rome to ensure formatting is stable regardless of the number of parenthesized nodes in a tree:
//!
//! ## Removing parenthesized nodes
//!
//! The JavaScript formatter [pre-processes](crate:JsFormatSyntaxRewriter] the input CST and removes all parenthesized expressions, assignments, and types except if:
//! * The parenthesized node has a syntax error (skipped token trivia, missing inner expression)
//! * The node has a directly preceding closure type cast comment
//! * The inner expression is an unknown node
//!
//! Removing the parenthesized nodes has the benefit that a input tree with parentheses and an input tree
//! without parentheses have the same structure for as far as the formatter is concerned and thus,
//! the formatter makes the same decisions for both trees.
//!
//! ## Parentheses insertion
//! The parentheses that get removed by the pre-processing step are re-inserted by the [crate::FormatNodeRule].
//! The rule inserts parentheses for each node where [crate::FormatNodeRule::needs_parentheses] returns true.

use crate::utils::{JsAnyBinaryLikeExpression, JsAnyBinaryLikeLeftExpression};

use rome_js_syntax::{
    JsAnyAssignment, JsAnyAssignmentPattern, JsAnyExpression, JsAnyFunctionBody,
    JsAnyLiteralExpression, JsArrowFunctionExpression, JsAssignmentExpression, JsBinaryExpression,
    JsBinaryOperator, JsComputedMemberAssignment, JsComputedMemberExpression,
    JsConditionalExpression, JsLanguage, JsParenthesizedAssignment, JsParenthesizedExpression,
    JsPrivateName, JsSequenceExpression, JsStaticMemberAssignment, JsStaticMemberExpression,
    JsSyntaxKind, JsSyntaxNode, JsSyntaxToken, TsConditionalType, TsIndexedAccessType,
    TsIntersectionTypeElementList, TsParenthesizedType, TsType, TsUnionTypeVariantList,
};
use rome_rowan::{declare_node_union, match_ast, AstNode, AstSeparatedList, SyntaxResult};

/// Node that may be parenthesized to ensure it forms valid syntax or to improve readability
pub trait NeedsParentheses: AstNode<Language = JsLanguage> {
    fn needs_parentheses(&self) -> bool {
        self.syntax()
            .parent()
            .map_or(false, |parent| self.needs_parentheses_with_parent(&parent))
    }

    /// Returns `true` if this node requires parentheses to form valid syntax or improve readability.
    ///
    /// Returns `false` if the parentheses can be omitted safely without changing semantics.
    fn needs_parentheses_with_parent(&self, parent: &JsSyntaxNode) -> bool;
}

impl NeedsParentheses for JsAnyLiteralExpression {
    #[inline]
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

    #[inline]
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

impl NeedsParentheses for JsAnyExpression {
    fn needs_parentheses(&self) -> bool {
        match self {
            JsAnyExpression::ImportMeta(meta) => meta.needs_parentheses(),
            JsAnyExpression::JsAnyLiteralExpression(literal) => literal.needs_parentheses(),
            JsAnyExpression::JsArrayExpression(array) => array.needs_parentheses(),
            JsAnyExpression::JsArrowFunctionExpression(arrow) => arrow.needs_parentheses(),
            JsAnyExpression::JsAssignmentExpression(assignment) => assignment.needs_parentheses(),
            JsAnyExpression::JsAwaitExpression(await_expression) => {
                await_expression.needs_parentheses()
            }
            JsAnyExpression::JsBinaryExpression(binary) => binary.needs_parentheses(),
            JsAnyExpression::JsCallExpression(call) => call.needs_parentheses(),
            JsAnyExpression::JsClassExpression(class) => class.needs_parentheses(),
            JsAnyExpression::JsComputedMemberExpression(member) => member.needs_parentheses(),
            JsAnyExpression::JsConditionalExpression(conditional) => {
                conditional.needs_parentheses()
            }
            JsAnyExpression::JsFunctionExpression(function) => function.needs_parentheses(),
            JsAnyExpression::JsIdentifierExpression(identifier) => identifier.needs_parentheses(),
            JsAnyExpression::JsImportCallExpression(import_call) => import_call.needs_parentheses(),
            JsAnyExpression::JsInExpression(in_expression) => in_expression.needs_parentheses(),
            JsAnyExpression::JsInstanceofExpression(instanceof) => instanceof.needs_parentheses(),
            JsAnyExpression::JsLogicalExpression(logical) => logical.needs_parentheses(),
            JsAnyExpression::JsNewExpression(new) => new.needs_parentheses(),
            JsAnyExpression::JsObjectExpression(object) => object.needs_parentheses(),
            JsAnyExpression::JsParenthesizedExpression(parenthesized) => {
                parenthesized.needs_parentheses()
            }
            JsAnyExpression::JsPostUpdateExpression(update) => update.needs_parentheses(),
            JsAnyExpression::JsPreUpdateExpression(update) => update.needs_parentheses(),
            JsAnyExpression::JsSequenceExpression(sequence) => sequence.needs_parentheses(),
            JsAnyExpression::JsStaticMemberExpression(member) => member.needs_parentheses(),
            JsAnyExpression::JsSuperExpression(sup) => sup.needs_parentheses(),
            JsAnyExpression::JsTemplate(template) => template.needs_parentheses(),
            JsAnyExpression::JsThisExpression(this) => this.needs_parentheses(),
            JsAnyExpression::JsUnaryExpression(unary) => unary.needs_parentheses(),
            JsAnyExpression::JsUnknownExpression(unknown) => unknown.needs_parentheses(),
            JsAnyExpression::JsYieldExpression(yield_expression) => {
                yield_expression.needs_parentheses()
            }
            JsAnyExpression::JsxTagExpression(jsx) => jsx.needs_parentheses(),
            JsAnyExpression::NewTarget(target) => target.needs_parentheses(),
            JsAnyExpression::TsAsExpression(as_expression) => as_expression.needs_parentheses(),
            JsAnyExpression::TsNonNullAssertionExpression(non_null) => non_null.needs_parentheses(),
            JsAnyExpression::TsTypeAssertionExpression(type_assertion) => {
                type_assertion.needs_parentheses()
            }
            JsAnyExpression::TsInstantiationExpression(arguments) => arguments.needs_parentheses(),
        }
    }

    fn needs_parentheses_with_parent(&self, parent: &JsSyntaxNode) -> bool {
        match self {
            JsAnyExpression::ImportMeta(meta) => meta.needs_parentheses_with_parent(parent),
            JsAnyExpression::JsAnyLiteralExpression(literal) => {
                literal.needs_parentheses_with_parent(parent)
            }
            JsAnyExpression::JsArrayExpression(array) => {
                array.needs_parentheses_with_parent(parent)
            }
            JsAnyExpression::JsArrowFunctionExpression(arrow) => {
                arrow.needs_parentheses_with_parent(parent)
            }
            JsAnyExpression::JsAssignmentExpression(assignment) => {
                assignment.needs_parentheses_with_parent(parent)
            }
            JsAnyExpression::JsAwaitExpression(await_expression) => {
                await_expression.needs_parentheses_with_parent(parent)
            }
            JsAnyExpression::JsBinaryExpression(binary) => {
                binary.needs_parentheses_with_parent(parent)
            }
            JsAnyExpression::JsCallExpression(call) => call.needs_parentheses_with_parent(parent),
            JsAnyExpression::JsClassExpression(class) => {
                class.needs_parentheses_with_parent(parent)
            }
            JsAnyExpression::JsComputedMemberExpression(member) => {
                member.needs_parentheses_with_parent(parent)
            }
            JsAnyExpression::JsConditionalExpression(conditional) => {
                conditional.needs_parentheses_with_parent(parent)
            }
            JsAnyExpression::JsFunctionExpression(function) => {
                function.needs_parentheses_with_parent(parent)
            }
            JsAnyExpression::JsIdentifierExpression(identifier) => {
                identifier.needs_parentheses_with_parent(parent)
            }
            JsAnyExpression::JsImportCallExpression(import_call) => {
                import_call.needs_parentheses_with_parent(parent)
            }
            JsAnyExpression::JsInExpression(in_expression) => {
                in_expression.needs_parentheses_with_parent(parent)
            }
            JsAnyExpression::JsInstanceofExpression(instanceof) => {
                instanceof.needs_parentheses_with_parent(parent)
            }
            JsAnyExpression::JsLogicalExpression(logical) => {
                logical.needs_parentheses_with_parent(parent)
            }
            JsAnyExpression::JsNewExpression(new) => new.needs_parentheses_with_parent(parent),
            JsAnyExpression::JsObjectExpression(object) => {
                object.needs_parentheses_with_parent(parent)
            }
            JsAnyExpression::JsParenthesizedExpression(parenthesized) => {
                parenthesized.needs_parentheses_with_parent(parent)
            }
            JsAnyExpression::JsPostUpdateExpression(update) => {
                update.needs_parentheses_with_parent(parent)
            }
            JsAnyExpression::JsPreUpdateExpression(update) => {
                update.needs_parentheses_with_parent(parent)
            }
            JsAnyExpression::JsSequenceExpression(sequence) => {
                sequence.needs_parentheses_with_parent(parent)
            }
            JsAnyExpression::JsStaticMemberExpression(member) => {
                member.needs_parentheses_with_parent(parent)
            }
            JsAnyExpression::JsSuperExpression(sup) => sup.needs_parentheses_with_parent(parent),
            JsAnyExpression::JsTemplate(template) => template.needs_parentheses_with_parent(parent),
            JsAnyExpression::JsThisExpression(this) => this.needs_parentheses_with_parent(parent),
            JsAnyExpression::JsUnaryExpression(unary) => {
                unary.needs_parentheses_with_parent(parent)
            }
            JsAnyExpression::JsUnknownExpression(unknown) => {
                unknown.needs_parentheses_with_parent(parent)
            }
            JsAnyExpression::JsYieldExpression(yield_expression) => {
                yield_expression.needs_parentheses_with_parent(parent)
            }
            JsAnyExpression::JsxTagExpression(jsx) => jsx.needs_parentheses_with_parent(parent),
            JsAnyExpression::NewTarget(target) => target.needs_parentheses_with_parent(parent),
            JsAnyExpression::TsAsExpression(as_expression) => {
                as_expression.needs_parentheses_with_parent(parent)
            }
            JsAnyExpression::TsNonNullAssertionExpression(non_null) => {
                non_null.needs_parentheses_with_parent(parent)
            }
            JsAnyExpression::TsTypeAssertionExpression(type_assertion) => {
                type_assertion.needs_parentheses_with_parent(parent)
            }
            JsAnyExpression::TsInstantiationExpression(expr) => {
                expr.needs_parentheses_with_parent(parent)
            }
        }
    }
}

declare_node_union! {
    pub(crate) JsAnyExpressionLeftSide = JsAnyExpression | JsPrivateName | JsAnyAssignmentPattern
}

impl NeedsParentheses for JsAnyExpressionLeftSide {
    fn needs_parentheses_with_parent(&self, parent: &JsSyntaxNode) -> bool {
        match self {
            JsAnyExpressionLeftSide::JsAnyExpression(expression) => {
                expression.needs_parentheses_with_parent(parent)
            }
            JsAnyExpressionLeftSide::JsPrivateName(_) => false,
            JsAnyExpressionLeftSide::JsAnyAssignmentPattern(assignment) => {
                assignment.needs_parentheses_with_parent(parent)
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
) -> JsAnyExpressionLeftSide {
    let mut current: JsAnyExpressionLeftSide = expression.clone().into();

    loop {
        match get_expression_left_side(&current) {
            None => {
                break current;
            }
            Some(left) => {
                current = left;
            }
        }
    }
}

/// Returns the left side of an expression (an expression where the first child is a `Node` or [None]
/// if the expression has no left side.
pub(crate) fn get_expression_left_side(
    current: &JsAnyExpressionLeftSide,
) -> Option<JsAnyExpressionLeftSide> {
    use JsAnyExpression::*;

    match current {
        JsAnyExpressionLeftSide::JsAnyExpression(expression) => {
            let left_expression = match expression {
                JsSequenceExpression(sequence) => sequence.left().ok(),
                JsStaticMemberExpression(member) => member.object().ok(),
                JsComputedMemberExpression(member) => member.object().ok(),
                JsTemplate(template) => template.tag(),
                JsNewExpression(new) => new.callee().ok(),
                JsCallExpression(call) => call.callee().ok(),
                JsConditionalExpression(conditional) => conditional.test().ok(),
                TsAsExpression(as_expression) => as_expression.expression().ok(),
                TsNonNullAssertionExpression(non_null) => non_null.expression().ok(),
                JsAssignmentExpression(assignment) => {
                    return assignment.left().ok().map(JsAnyExpressionLeftSide::from)
                }
                JsPostUpdateExpression(expression) => {
                    return expression.operand().ok().map(|assignment| {
                        JsAnyExpressionLeftSide::from(JsAnyAssignmentPattern::JsAnyAssignment(
                            assignment,
                        ))
                    })
                }
                expression => {
                    return JsAnyBinaryLikeExpression::cast(expression.syntax().clone()).and_then(
                        |binary_like| match binary_like.left().ok() {
                            Some(JsAnyBinaryLikeLeftExpression::JsAnyExpression(expression)) => {
                                Some(JsAnyExpressionLeftSide::from(expression))
                            }
                            Some(JsAnyBinaryLikeLeftExpression::JsPrivateName(name)) => {
                                Some(JsAnyExpressionLeftSide::from(name))
                            }
                            None => None,
                        },
                    );
                }
            };

            left_expression.map(JsAnyExpressionLeftSide::from)
        }
        JsAnyExpressionLeftSide::JsAnyAssignmentPattern(pattern) => {
            use JsAnyAssignment::*;

            let left = match pattern {
                JsAnyAssignmentPattern::JsAnyAssignment(assignment) => match assignment {
                    JsComputedMemberAssignment(computed) => {
                        return computed.object().ok().map(JsAnyExpressionLeftSide::from)
                    }
                    JsStaticMemberAssignment(member) => {
                        return member.object().ok().map(JsAnyExpressionLeftSide::from)
                    }

                    TsAsAssignment(parent) => parent.assignment().ok(),
                    TsNonNullAssertionAssignment(parent) => parent.assignment().ok(),
                    TsTypeAssertionAssignment(parent) => parent.assignment().ok(),
                    JsParenthesizedAssignment(_)
                    | JsIdentifierAssignment(_)
                    | JsUnknownAssignment(_) => None,
                },
                JsAnyAssignmentPattern::JsArrayAssignmentPattern(_)
                | JsAnyAssignmentPattern::JsObjectAssignmentPattern(_) => None,
            };

            left.map(|assignment| {
                JsAnyExpressionLeftSide::from(JsAnyAssignmentPattern::JsAnyAssignment(assignment))
            })
        }
        JsAnyExpressionLeftSide::JsPrivateName(_) => None,
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub(crate) enum FirstInStatementMode {
    /// Considers [JsExpressionStatement] and the body of [JsArrowFunctionExpression] as the first statement.
    ExpressionStatementOrArrow,

    /// Considers [JsExpressionStatement] and [JsExportDefaultExpressionClause] as the first statement.
    ExpressionOrExportDefault,
}

/// Returns `true` if this node is at the start of an expression (depends on the passed `mode`).
///
/// Traverses upwards the tree for as long as the `node` is the left most expression until the node isn't
/// the left most node or reached a statement.
pub(crate) fn is_first_in_statement(node: JsSyntaxNode, mode: FirstInStatementMode) -> bool {
    let mut current = node;

    while let Some(parent) = current.parent() {
        let parent = match parent.kind() {
            JsSyntaxKind::JS_EXPRESSION_STATEMENT => {
                return true;
            }

            JsSyntaxKind::JS_STATIC_MEMBER_EXPRESSION
            | JsSyntaxKind::JS_STATIC_MEMBER_ASSIGNMENT
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

            JsSyntaxKind::JS_COMPUTED_MEMBER_ASSIGNMENT => {
                let assignment = JsComputedMemberAssignment::unwrap_cast(parent);

                let is_object =
                    assignment.object().map(AstNode::into_syntax).as_ref() == Ok(&current);

                if is_object {
                    assignment.into_syntax()
                } else {
                    break;
                }
            }

            JsSyntaxKind::JS_ASSIGNMENT_EXPRESSION => {
                let assignment = JsAssignmentExpression::unwrap_cast(parent);

                let is_left = assignment.left().map(AstNode::into_syntax).as_ref() == Ok(&current);

                if is_left {
                    assignment.into_syntax()
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

/// Implements the shared logic for when parentheses are necessary for [JsPreUpdateExpression], [JsPostUpdateExpression], or [JsUnaryExpression] expressions.
/// Each expression may implement node specific rules, which is why calling `needs_parens` on the node is preferred.
pub(crate) fn unary_like_expression_needs_parentheses(
    expression: &JsSyntaxNode,
    parent: &JsSyntaxNode,
) -> bool {
    debug_assert!(matches!(
        expression.kind(),
        JsSyntaxKind::JS_PRE_UPDATE_EXPRESSION
            | JsSyntaxKind::JS_POST_UPDATE_EXPRESSION
            | JsSyntaxKind::JS_UNARY_EXPRESSION
    ));
    debug_assert_is_parent(expression, parent);

    if let Some(binary) = JsBinaryExpression::cast_ref(parent) {
        matches!(binary.operator(), Ok(JsBinaryOperator::Exponent))
            && binary.left().map(AstNode::into_syntax).as_ref() == Ok(expression)
    } else {
        update_or_lower_expression_needs_parentheses(expression, parent)
    }
}

/// Returns `true` if an expression with lower precedence than an update expression needs parentheses.
///
/// This is generally the case if the expression is used in a left hand side, or primary expression context.
pub(crate) fn update_or_lower_expression_needs_parentheses(
    expression: &JsSyntaxNode,
    parent: &JsSyntaxNode,
) -> bool {
    debug_assert_is_expression(expression);
    debug_assert_is_parent(expression, parent);

    match parent.kind() {
        JsSyntaxKind::JS_EXTENDS_CLAUSE => true,
        _ => match parent.kind() {
            JsSyntaxKind::TS_NON_NULL_ASSERTION_EXPRESSION => true,

            _ => {
                is_callee(expression, parent)
                    || is_member_object(expression, parent)
                    || is_tag(expression, parent)
            }
        },
    }
}

/// Returns `true` if `node< is the `object` of a [JsStaticMemberExpression] or [JsComputedMemberExpression]
pub(crate) fn is_member_object(node: &JsSyntaxNode, parent: &JsSyntaxNode) -> bool {
    debug_assert_is_expression(node);
    debug_assert_is_parent(node, parent);

    match_ast! {
        match parent {
            // Only allows expression in the `object` child.
            JsStaticMemberExpression(_) => true,
            JsStaticMemberAssignment(_) => true,
            JsComputedMemberExpression(member_expression) => {
                 member_expression
                    .object()
                    .map(AstNode::into_syntax)
                    .as_ref()
                    == Ok(node)
            },
            JsComputedMemberAssignment(assignment) => {
                assignment
                    .object()
                    .map(AstNode::into_syntax)
                    .as_ref()
                    == Ok(node)
            },
            _ => false,
        }
    }
}

/// Returns `true` if `node` is the `callee` of a [JsNewExpression] or [JsCallExpression].
pub(crate) fn is_callee(node: &JsSyntaxNode, parent: &JsSyntaxNode) -> bool {
    debug_assert_is_expression(node);
    debug_assert_is_parent(node, parent);

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
    match_ast! {
        match parent {
            JsConditionalExpression(conditional) => {
                conditional
                    .test()
                    .map(AstNode::into_syntax)
                    .as_ref()
                    == Ok(node)
            },
            _ => false
        }
    }
}

pub(crate) fn is_arrow_function_body(node: &JsSyntaxNode, parent: &JsSyntaxNode) -> bool {
    debug_assert_is_expression(node);

    match_ast! {
        match parent {
            JsArrowFunctionExpression(arrow) => {
                match arrow.body() {
                    Ok(JsAnyFunctionBody::JsAnyExpression(expression)) => {
                        expression.syntax() == node
                    }
                    _ => false,
                }
            },
            _ => false
        }
    }
}

/// Returns `true` if `node` is the `tag` of a [JsTemplate] expression
pub(crate) fn is_tag(node: &JsSyntaxNode, parent: &JsSyntaxNode) -> bool {
    debug_assert_is_expression(node);
    debug_assert_is_parent(node, parent);

    matches!(parent.kind(), JsSyntaxKind::JS_TEMPLATE)
}

/// Returns `true` if `node` is a spread `...node`
pub(crate) fn is_spread(node: &JsSyntaxNode, parent: &JsSyntaxNode) -> bool {
    debug_assert_is_expression(node);
    debug_assert_is_parent(node, parent);

    matches!(
        parent.kind(),
        JsSyntaxKind::JSX_SPREAD_CHILD
            | JsSyntaxKind::JS_SPREAD
            | JsSyntaxKind::JSX_SPREAD_ATTRIBUTE
    )
}

/// Returns `true` if a TS primary type needs parentheses
pub(crate) fn operator_type_or_higher_needs_parens(
    node: &JsSyntaxNode,
    parent: &JsSyntaxNode,
) -> bool {
    debug_assert_is_parent(node, parent);

    match parent.kind() {
        JsSyntaxKind::TS_ARRAY_TYPE
        | JsSyntaxKind::TS_TYPE_OPERATOR_TYPE
        | JsSyntaxKind::TS_REST_TUPLE_TYPE_ELEMENT
        | JsSyntaxKind::TS_OPTIONAL_TUPLE_TYPE_ELEMENT => true,
        JsSyntaxKind::TS_INDEXED_ACCESS_TYPE => {
            let indexed = TsIndexedAccessType::unwrap_cast(parent.clone());

            indexed.object_type().map(AstNode::into_syntax).as_ref() == Ok(node)
        }
        _ => false,
    }
}

/// Tests if `node` is the check type of a [TsConditionalType]
///
/// ```javascript
/// type s = A extends string ? string : number //  true for `A`, false for `string` and `number`
/// ```
pub(crate) fn is_check_type(node: &JsSyntaxNode, parent: &JsSyntaxNode) -> bool {
    debug_assert_is_parent(node, parent);

    match parent.kind() {
        JsSyntaxKind::TS_CONDITIONAL_TYPE => {
            let conditional = TsConditionalType::unwrap_cast(parent.clone());

            conditional.check_type().map(AstNode::into_syntax).as_ref() == Ok(node)
        }
        _ => false,
    }
}

/// Returns `true` if node is in a union or intersection type with more than one variant
///
/// ```javascript
/// type A = &string // -> false for `string` because `string` is the only variant
/// type B = string & number // -> true for `string` or `number`
/// type C = |string // -> false
/// type D = string | number // -> true
/// ```
pub(crate) fn is_in_many_type_union_or_intersection_list(
    node: &JsSyntaxNode,
    parent: &JsSyntaxNode,
) -> bool {
    debug_assert_is_parent(node, parent);

    match parent.kind() {
        JsSyntaxKind::TS_UNION_TYPE_VARIANT_LIST => {
            let list = TsUnionTypeVariantList::unwrap_cast(parent.clone());

            list.len() > 1
        }
        JsSyntaxKind::TS_INTERSECTION_TYPE_ELEMENT_LIST => {
            let list = TsIntersectionTypeElementList::unwrap_cast(parent.clone());

            list.len() > 1
        }
        _ => false,
    }
}

declare_node_union! {
    pub(crate) JsAnyParenthesized = JsParenthesizedExpression | JsParenthesizedAssignment | TsParenthesizedType
}

impl JsAnyParenthesized {
    pub(crate) fn l_paren_token(&self) -> SyntaxResult<JsSyntaxToken> {
        match self {
            JsAnyParenthesized::JsParenthesizedExpression(expression) => expression.l_paren_token(),
            JsAnyParenthesized::JsParenthesizedAssignment(assignment) => assignment.l_paren_token(),
            JsAnyParenthesized::TsParenthesizedType(ty) => ty.l_paren_token(),
        }
    }

    pub(crate) fn inner(&self) -> SyntaxResult<JsSyntaxNode> {
        match self {
            JsAnyParenthesized::JsParenthesizedExpression(expression) => {
                expression.expression().map(AstNode::into_syntax)
            }
            JsAnyParenthesized::JsParenthesizedAssignment(assignment) => {
                assignment.assignment().map(AstNode::into_syntax)
            }
            JsAnyParenthesized::TsParenthesizedType(ty) => ty.ty().map(AstNode::into_syntax),
        }
    }

    pub(crate) fn r_paren_token(&self) -> SyntaxResult<JsSyntaxToken> {
        match self {
            JsAnyParenthesized::JsParenthesizedExpression(expression) => expression.r_paren_token(),
            JsAnyParenthesized::JsParenthesizedAssignment(assignment) => assignment.r_paren_token(),
            JsAnyParenthesized::TsParenthesizedType(ty) => ty.r_paren_token(),
        }
    }
}

/// Returns `true` if `parent` is a [JsAnyBinaryLikeExpression] and `node` is the `left` or `right` of that expression.
pub(crate) fn is_binary_like_left_or_right(node: &JsSyntaxNode, parent: &JsSyntaxNode) -> bool {
    debug_assert_is_expression(node);
    debug_assert_is_parent(node, parent);

    JsAnyBinaryLikeExpression::can_cast(parent.kind())
}

impl NeedsParentheses for JsAnyAssignment {
    fn needs_parentheses(&self) -> bool {
        match self {
            JsAnyAssignment::JsComputedMemberAssignment(assignment) => {
                assignment.needs_parentheses()
            }
            JsAnyAssignment::JsIdentifierAssignment(assignment) => assignment.needs_parentheses(),
            JsAnyAssignment::JsParenthesizedAssignment(assignment) => {
                assignment.needs_parentheses()
            }
            JsAnyAssignment::JsStaticMemberAssignment(assignment) => assignment.needs_parentheses(),
            JsAnyAssignment::JsUnknownAssignment(assignment) => assignment.needs_parentheses(),
            JsAnyAssignment::TsAsAssignment(assignment) => assignment.needs_parentheses(),
            JsAnyAssignment::TsNonNullAssertionAssignment(assignment) => {
                assignment.needs_parentheses()
            }
            JsAnyAssignment::TsTypeAssertionAssignment(assignment) => {
                assignment.needs_parentheses()
            }
        }
    }

    fn needs_parentheses_with_parent(&self, parent: &JsSyntaxNode) -> bool {
        match self {
            JsAnyAssignment::JsComputedMemberAssignment(assignment) => {
                assignment.needs_parentheses_with_parent(parent)
            }
            JsAnyAssignment::JsIdentifierAssignment(assignment) => {
                assignment.needs_parentheses_with_parent(parent)
            }
            JsAnyAssignment::JsParenthesizedAssignment(assignment) => {
                assignment.needs_parentheses_with_parent(parent)
            }
            JsAnyAssignment::JsStaticMemberAssignment(assignment) => {
                assignment.needs_parentheses_with_parent(parent)
            }
            JsAnyAssignment::JsUnknownAssignment(assignment) => {
                assignment.needs_parentheses_with_parent(parent)
            }
            JsAnyAssignment::TsAsAssignment(assignment) => {
                assignment.needs_parentheses_with_parent(parent)
            }
            JsAnyAssignment::TsNonNullAssertionAssignment(assignment) => {
                assignment.needs_parentheses_with_parent(parent)
            }
            JsAnyAssignment::TsTypeAssertionAssignment(assignment) => {
                assignment.needs_parentheses_with_parent(parent)
            }
        }
    }
}

impl NeedsParentheses for JsAnyAssignmentPattern {
    fn needs_parentheses(&self) -> bool {
        match self {
            JsAnyAssignmentPattern::JsAnyAssignment(assignment) => assignment.needs_parentheses(),
            JsAnyAssignmentPattern::JsArrayAssignmentPattern(assignment) => {
                assignment.needs_parentheses()
            }
            JsAnyAssignmentPattern::JsObjectAssignmentPattern(assignment) => {
                assignment.needs_parentheses()
            }
        }
    }

    fn needs_parentheses_with_parent(&self, parent: &JsSyntaxNode) -> bool {
        match self {
            JsAnyAssignmentPattern::JsAnyAssignment(assignment) => {
                assignment.needs_parentheses_with_parent(parent)
            }
            JsAnyAssignmentPattern::JsArrayAssignmentPattern(assignment) => {
                assignment.needs_parentheses_with_parent(parent)
            }
            JsAnyAssignmentPattern::JsObjectAssignmentPattern(assignment) => {
                assignment.needs_parentheses_with_parent(parent)
            }
        }
    }
}

impl NeedsParentheses for TsType {
    fn needs_parentheses(&self) -> bool {
        match self {
            TsType::TsAnyType(ty) => ty.needs_parentheses(),
            TsType::TsArrayType(ty) => ty.needs_parentheses(),
            TsType::TsBigIntLiteralType(ty) => ty.needs_parentheses(),
            TsType::TsBigintType(ty) => ty.needs_parentheses(),
            TsType::TsBooleanLiteralType(ty) => ty.needs_parentheses(),
            TsType::TsBooleanType(ty) => ty.needs_parentheses(),
            TsType::TsConditionalType(ty) => ty.needs_parentheses(),
            TsType::TsConstructorType(ty) => ty.needs_parentheses(),
            TsType::TsFunctionType(ty) => ty.needs_parentheses(),
            TsType::TsImportType(ty) => ty.needs_parentheses(),
            TsType::TsIndexedAccessType(ty) => ty.needs_parentheses(),
            TsType::TsInferType(ty) => ty.needs_parentheses(),
            TsType::TsIntersectionType(ty) => ty.needs_parentheses(),
            TsType::TsMappedType(ty) => ty.needs_parentheses(),
            TsType::TsNeverType(ty) => ty.needs_parentheses(),
            TsType::TsNonPrimitiveType(ty) => ty.needs_parentheses(),
            TsType::TsNullLiteralType(ty) => ty.needs_parentheses(),
            TsType::TsNumberLiteralType(ty) => ty.needs_parentheses(),
            TsType::TsNumberType(ty) => ty.needs_parentheses(),
            TsType::TsObjectType(ty) => ty.needs_parentheses(),
            TsType::TsParenthesizedType(ty) => ty.needs_parentheses(),
            TsType::TsReferenceType(ty) => ty.needs_parentheses(),
            TsType::TsStringLiteralType(ty) => ty.needs_parentheses(),
            TsType::TsStringType(ty) => ty.needs_parentheses(),
            TsType::TsSymbolType(ty) => ty.needs_parentheses(),
            TsType::TsTemplateLiteralType(ty) => ty.needs_parentheses(),
            TsType::TsThisType(ty) => ty.needs_parentheses(),
            TsType::TsTupleType(ty) => ty.needs_parentheses(),
            TsType::TsTypeOperatorType(ty) => ty.needs_parentheses(),
            TsType::TsTypeofType(ty) => ty.needs_parentheses(),
            TsType::TsUndefinedType(ty) => ty.needs_parentheses(),
            TsType::TsUnionType(ty) => ty.needs_parentheses(),
            TsType::TsUnknownType(ty) => ty.needs_parentheses(),
            TsType::TsVoidType(ty) => ty.needs_parentheses(),
        }
    }

    fn needs_parentheses_with_parent(&self, parent: &JsSyntaxNode) -> bool {
        match self {
            TsType::TsAnyType(ty) => ty.needs_parentheses_with_parent(parent),
            TsType::TsArrayType(ty) => ty.needs_parentheses_with_parent(parent),
            TsType::TsBigIntLiteralType(ty) => ty.needs_parentheses_with_parent(parent),
            TsType::TsBigintType(ty) => ty.needs_parentheses_with_parent(parent),
            TsType::TsBooleanLiteralType(ty) => ty.needs_parentheses_with_parent(parent),
            TsType::TsBooleanType(ty) => ty.needs_parentheses_with_parent(parent),
            TsType::TsConditionalType(ty) => ty.needs_parentheses_with_parent(parent),
            TsType::TsConstructorType(ty) => ty.needs_parentheses_with_parent(parent),
            TsType::TsFunctionType(ty) => ty.needs_parentheses_with_parent(parent),
            TsType::TsImportType(ty) => ty.needs_parentheses_with_parent(parent),
            TsType::TsIndexedAccessType(ty) => ty.needs_parentheses_with_parent(parent),
            TsType::TsInferType(ty) => ty.needs_parentheses_with_parent(parent),
            TsType::TsIntersectionType(ty) => ty.needs_parentheses_with_parent(parent),
            TsType::TsMappedType(ty) => ty.needs_parentheses_with_parent(parent),
            TsType::TsNeverType(ty) => ty.needs_parentheses_with_parent(parent),
            TsType::TsNonPrimitiveType(ty) => ty.needs_parentheses_with_parent(parent),
            TsType::TsNullLiteralType(ty) => ty.needs_parentheses_with_parent(parent),
            TsType::TsNumberLiteralType(ty) => ty.needs_parentheses_with_parent(parent),
            TsType::TsNumberType(ty) => ty.needs_parentheses_with_parent(parent),
            TsType::TsObjectType(ty) => ty.needs_parentheses_with_parent(parent),
            TsType::TsParenthesizedType(ty) => ty.needs_parentheses_with_parent(parent),
            TsType::TsReferenceType(ty) => ty.needs_parentheses_with_parent(parent),
            TsType::TsStringLiteralType(ty) => ty.needs_parentheses_with_parent(parent),
            TsType::TsStringType(ty) => ty.needs_parentheses_with_parent(parent),
            TsType::TsSymbolType(ty) => ty.needs_parentheses_with_parent(parent),
            TsType::TsTemplateLiteralType(ty) => ty.needs_parentheses_with_parent(parent),
            TsType::TsThisType(ty) => ty.needs_parentheses_with_parent(parent),
            TsType::TsTupleType(ty) => ty.needs_parentheses_with_parent(parent),
            TsType::TsTypeOperatorType(ty) => ty.needs_parentheses_with_parent(parent),
            TsType::TsTypeofType(ty) => ty.needs_parentheses_with_parent(parent),
            TsType::TsUndefinedType(ty) => ty.needs_parentheses_with_parent(parent),
            TsType::TsUnionType(ty) => ty.needs_parentheses_with_parent(parent),
            TsType::TsUnknownType(ty) => ty.needs_parentheses_with_parent(parent),
            TsType::TsVoidType(ty) => ty.needs_parentheses_with_parent(parent),
        }
    }
}

fn debug_assert_is_expression(node: &JsSyntaxNode) {
    debug_assert!(
        JsAnyExpression::can_cast(node.kind()),
        "Expected {node:#?} to be an expression."
    )
}

pub(crate) fn debug_assert_is_parent(node: &JsSyntaxNode, parent: &JsSyntaxNode) {
    debug_assert!(
        node.parent().as_ref() == Some(parent),
        "Node {node:#?} is not a child of ${parent:#?}"
    )
}

#[cfg(test)]
pub(crate) mod tests {
    use super::NeedsParentheses;
    use crate::transform;
    use rome_diagnostics::file::FileId;
    use rome_js_syntax::{JsLanguage, SourceType};
    use rome_rowan::AstNode;

    pub(crate) fn assert_needs_parentheses_impl<
        T: AstNode<Language = JsLanguage> + std::fmt::Debug + NeedsParentheses,
    >(
        input: &'static str,
        index: Option<usize>,
        source_type: SourceType,
    ) {
        let parse = rome_js_parser::parse(input, FileId::zero(), source_type);

        let diagnostics = parse.diagnostics();
        assert!(
            diagnostics.is_empty(),
            "Expected input program to not have syntax errors but had {diagnostics:?}"
        );

        let root = parse.syntax();
        let (transformed, _) = transform(root);
        let matching_nodes: Vec<_> = transformed.descendants().filter_map(T::cast).collect();

        let node = if let Some(index) = index {
            matching_nodes.get(index).unwrap_or_else(|| {
                panic!("Out of bound index {index}, matching nodes are:\n{matching_nodes:#?}");
            })
        } else {
            match matching_nodes.len() {
                0 => {
                    panic!(
                        "Expected to find a '{}' node in '{input}' but found none.",
                        core::any::type_name::<T>(),
                    )
                }
                1 => matching_nodes.get(0).unwrap(),
                _ => {
                    panic!("Expected to find a single node matching '{}' in '{input}' but found multiple ones:\n {matching_nodes:#?}", core::any::type_name::<T>());
                }
            }
        };

        assert!(node.needs_parentheses());
    }

    pub(crate) fn assert_not_needs_parentheses_impl<
        T: AstNode<Language = JsLanguage> + std::fmt::Debug + NeedsParentheses,
    >(
        input: &'static str,
        index: Option<usize>,
        source_type: SourceType,
    ) {
        let parse = rome_js_parser::parse(input, FileId::zero(), source_type);

        let diagnostics = parse.diagnostics();
        assert!(
            diagnostics.is_empty(),
            "Expected input program to not have syntax errors but had {diagnostics:?}"
        );

        let root = parse.syntax();
        let (transformed, _) = transform(root);
        let matching_nodes: Vec<_> = transformed.descendants().filter_map(T::cast).collect();

        let node = if let Some(index) = index {
            matching_nodes.get(index).unwrap_or_else(|| {
                panic!("Out of bound index {index}, matching nodes are:\n{matching_nodes:#?}");
            })
        } else {
            match matching_nodes.len() {
                0 => {
                    panic!(
                        "Expected to find a '{}' node in '{input}' but found none.",
                        core::any::type_name::<T>(),
                    )
                }
                1 => matching_nodes.get(0).unwrap(),
                _ => {
                    panic!("Expected to find a single node matching '{}' in '{input}' but found multiple ones:\n {matching_nodes:#?}", core::any::type_name::<T>());
                }
            }
        };

        assert!(!node.needs_parentheses());
    }

    /// Helper macro to test the [NeedsParentheses] implementation of a node.
    ///
    /// # Example
    ///
    ///
    /// ```
    /// # use rome_js_formatter::assert_needs_parentheses;
    /// use rome_js_syntax::JsStaticMemberExpression;
    ///
    /// assert_needs_parentheses!("new (test().a)()", JsStaticMemberExpression);
    /// ```
    ///
    /// Asserts that [NeedsParentheses.needs_parentheses()] returns true for the only [JsStaticMemberExpression] in the program.
    ///
    /// ```
    /// # use rome_js_syntax::JsStaticMemberExpression;
    /// use rome_js_formatter::assert_needs_parentheses;
    ///
    /// assert_needs_parentheses!("new (test().a).b)()", JsStaticMemberExpression[1]);
    /// ```
    ///
    /// Asserts that [NeedsParentheses.needs_parentheses()] returns true for the second (in pre-order) [JsStaticMemberExpression] in the program.
    #[macro_export]
    macro_rules! assert_needs_parentheses {
        ($input:expr, $Node:ident) => {{
            $crate::assert_needs_parentheses!($input, $Node, rome_js_syntax::SourceType::ts())
        }};

        ($input:expr, $Node:ident[$index:expr]) => {{
            $crate::assert_needs_parentheses!(
                $input,
                $Node[$index],
                rome_js_syntax::SourceType::ts()
            )
        }};

        ($input:expr, $Node:ident, $source_type: expr) => {{
            $crate::parentheses::tests::assert_needs_parentheses_impl::<$Node>(
                $input,
                None,
                $source_type,
            )
        }};

        ($input:expr, $Node:ident[$index:expr], $source_type: expr) => {{
            $crate::parentheses::tests::assert_needs_parentheses_impl::<$Node>(
                $input,
                Some($index),
                $source_type,
            )
        }};
    }

    /// Helper macro to test the [NeedsParentheses] implementation of a node.
    ///
    /// # Example
    ///
    ///
    /// ```
    /// # use rome_js_syntax::JsStaticMemberExpression;
    /// use rome_js_formatter::assert_not_needs_parentheses;
    ///
    /// assert_not_needs_parentheses!("a.b", JsStaticMemberExpression);
    /// ```
    ///
    /// Asserts that [NeedsParentheses.needs_parentheses()] returns true for the only [JsStaticMemberExpression] in the program.
    ///
    /// ```
    /// # use rome_js_syntax::JsStaticMemberExpression;
    /// use rome_js_formatter::assert_not_needs_parentheses;
    ///
    /// assert_not_needs_parentheses!("a.b.c", JsStaticMemberExpression[0]);
    /// ```
    ///
    /// Asserts that [NeedsParentheses.needs_parentheses()] returns true for the first (in pre-order) [JsStaticMemberExpression] in the program.
    #[macro_export]
    macro_rules! assert_not_needs_parentheses {
        ($input:expr, $Node:ident) => {{
            $crate::assert_not_needs_parentheses!($input, $Node, rome_js_syntax::SourceType::ts())
        }};

        ($input:expr, $Node:ident[$index:expr]) => {{
            $crate::assert_not_needs_parentheses!(
                $input,
                $Node[$index],
                rome_js_syntax::SourceType::ts()
            )
        }};

        ($input:expr, $Node:ident[$index:expr], $source_type: expr) => {{
            $crate::parentheses::tests::assert_not_needs_parentheses_impl::<$Node>(
                $input,
                Some($index),
                $source_type,
            )
        }};

        ($input:expr, $Node:ident, $source_type: expr) => {{
            $crate::parentheses::tests::assert_not_needs_parentheses_impl::<$Node>(
                $input,
                None,
                $source_type,
            )
        }};
    }
}
