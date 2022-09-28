use rome_analyze::{
    context::RuleContext, declare_rule, ActionCategory, Ast, Rule, RuleCategory, RuleDiagnostic,
};
use rome_console::markup;
use rome_diagnostics::Applicability;
use rome_js_syntax::{
    JsAnyExpression, JsCallArgumentList, JsCallArguments, JsCallExpression,
    JsConditionalExpression, JsDoWhileStatement, JsForStatement, JsIfStatement, JsNewExpression,
    JsSyntaxKind, JsSyntaxNode, JsUnaryExpression, JsUnaryOperator, JsWhileStatement,
};
use rome_rowan::{AstNode, AstSeparatedList, BatchMutationExt, SyntaxNodeCast};

use crate::JsRuleAction;

pub enum ExtraBooleanCastType {
    /// !!x
    DoubleNegation,
    /// Boolean(x)
    BooleanCall,
}
declare_rule! {
    /// Disallow unnecessary boolean casts
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// if (!Boolean(foo)) {
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// while (!!foo) {}
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// let x = 1;
    /// do {
    /// 1 + 1;
    /// } while (Boolean(x));
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// for (; !!foo; ) {}
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// new Boolean(!!x);
    /// ```
    ///
    /// ### Valid
    /// ```js
    /// Boolean(!x);
    /// !x;
    /// !!x;
    /// ```

    pub(crate) NoExtraBooleanCast {
        version: "0.9.0",
        name: "noExtraBooleanCast",
        recommended: true,
    }
}

/// Check if this node is in the position of `test` slot of parent syntax node.
/// ## Example
/// ```js
/// if (!!x) {
///     ^^^ this is a boolean context
/// }
/// ```
fn is_in_boolean_context(node: &JsSyntaxNode) -> Option<bool> {
    let parent = node.parent()?;
    match parent.kind() {
        JsSyntaxKind::JS_IF_STATEMENT => {
            Some(parent.cast::<JsIfStatement>()?.test().ok()?.syntax() == node)
        }
        JsSyntaxKind::JS_DO_WHILE_STATEMENT => {
            Some(parent.cast::<JsDoWhileStatement>()?.test().ok()?.syntax() == node)
        }
        JsSyntaxKind::JS_WHILE_STATEMENT => {
            Some(parent.cast::<JsWhileStatement>()?.test().ok()?.syntax() == node)
        }
        JsSyntaxKind::JS_FOR_STATEMENT => {
            Some(parent.cast::<JsForStatement>()?.test()?.syntax() == node)
        }
        JsSyntaxKind::JS_CONDITIONAL_EXPRESSION => Some(
            parent
                .cast::<JsConditionalExpression>()?
                .test()
                .ok()?
                .syntax()
                == node,
        ),
        _ => None,
    }
}

/// Checks if the node is a `Boolean` Constructor Call
/// # Example
/// ```js
/// new Boolean(x);
/// ```
/// The checking algorithm of [JsNewExpression] is a little different from [JsCallExpression] due to
/// two nodes have different shapes
fn is_boolean_constructor_call(node: &JsSyntaxNode) -> Option<bool> {
    let callee = JsCallArgumentList::cast(node.clone())?
        .parent::<JsCallArguments>()?
        .parent::<JsNewExpression>()?
        .callee()
        .ok()?;
    if let JsAnyExpression::JsIdentifierExpression(ident) = callee {
        Some(ident.name().ok()?.syntax().text_trimmed() == "Boolean")
    } else {
        None
    }
}

/// Check if the SyntaxNode is a `Boolean` Call Expression
/// ## Example
/// ```js
/// Boolean(x)
/// ```
fn is_boolean_call(node: &JsSyntaxNode) -> Option<bool> {
    let callee = JsCallExpression::cast(node.clone())?.callee().ok()?;
    if let JsAnyExpression::JsIdentifierExpression(ident) = callee {
        Some(ident.name().ok()?.syntax().text_trimmed() == "Boolean")
    } else {
        None
    }
}

/// Check if the SyntaxNode is a Negate Unary Expression
/// ## Example
/// ```js
/// !!x
/// ```
fn is_negation(node: &JsSyntaxNode) -> Option<JsUnaryExpression> {
    let unary_expr = JsUnaryExpression::cast(node.clone())?;
    if unary_expr.operator().ok()? == JsUnaryOperator::LogicalNot {
        Some(unary_expr)
    } else {
        None
    }
}

impl Rule for NoExtraBooleanCast {
    const CATEGORY: RuleCategory = RuleCategory::Lint;

    type Query = Ast<JsAnyExpression>;
    type State = (JsAnyExpression, ExtraBooleanCastType);
    type Signals = Option<Self::State>;

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let n = ctx.query();
        let syntax = n.syntax().clone();
        let parent_syntax = syntax.parent()?;

        // Check if parent `SyntaxNode` in any boolean `Type Coercion` context,
        // reference https://developer.mozilla.org/en-US/docs/Web/JavaScript/Reference/Global_Objects/Boolean
        let parent_node_in_boolean_cast_context = is_in_boolean_context(&syntax).unwrap_or(false)
            || is_boolean_constructor_call(&parent_syntax).unwrap_or(false)
            || is_negation(&parent_syntax).is_some()
            || is_boolean_call(&parent_syntax).unwrap_or(false);
        // Convert `!!x` -> `x` if parent `SyntaxNode` in any boolean `Type Coercion` context
        if parent_node_in_boolean_cast_context {
            if let Some(result) = is_double_negation_ignore_parenthesis(&syntax) {
                return Some(result);
            };

            // Convert `Boolean(x)` -> `x` if parent `SyntaxNode` in any boolean `Type Coercion` context
            // Only if `Boolean` Call Expression have one `JsAnyExpression` argument
            if let Some(expr) = JsCallExpression::cast(syntax.clone()) {
                let callee = expr.callee().ok()?;
                if let JsAnyExpression::JsIdentifierExpression(ident) = callee {
                    if ident.name().ok()?.syntax().text_trimmed() == "Boolean" {
                        let arguments = expr.arguments().ok()?;
                        let len = arguments.args().len();
                        if len == 1 {
                            return arguments
                                .args()
                                .into_iter()
                                .next()?
                                .ok()
                                .map(|item| item.into_syntax())
                                .and_then(JsAnyExpression::cast)
                                .map(|expr| (expr, ExtraBooleanCastType::BooleanCall));
                        }
                    }
                }
                return None;
            }

            // Convert `new Boolean(x)` -> `x` if parent `SyntaxNode` in any boolean `Type Coercion` context
            // Only if `Boolean` Call Expression have one `JsAnyExpression` argument
            return JsNewExpression::cast(syntax).and_then(|expr| {
                let callee = expr.callee().ok()?;
                if let JsAnyExpression::JsIdentifierExpression(ident) = callee {
                    if ident.name().ok()?.syntax().text_trimmed() == "Boolean" {
                        let arguments = expr.arguments()?;
                        let len = arguments.args().len();
                        if len == 1 {
                            return arguments
                                .args()
                                .into_iter()
                                .next()?
                                .ok()
                                .map(|item| item.into_syntax())
                                .and_then(JsAnyExpression::cast)
                                .map(|expr| (expr, ExtraBooleanCastType::BooleanCall));
                        }
                    }
                }
                None
            });
        }
        None
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        let (_, extra_boolean_cast_type) = state;
        let (title, note) = match extra_boolean_cast_type {
			ExtraBooleanCastType::DoubleNegation => ("Avoid redundant double-negation.", "It is not necessary to use double-negation when a value will already be coerced to a boolean."),
			ExtraBooleanCastType::BooleanCall => ("Avoid redundant `Boolean` call", "It is not necessary to use `Boolean` call when a value will already be coerced to a boolean."),
		};
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup! {
                    {title}
                },
            )
            .footer_note(note),
        )
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query();
        let mut mutation = ctx.root().begin();
        let (node_to_replace, extra_boolean_cast_type) = state;
        let message = match extra_boolean_cast_type {
            ExtraBooleanCastType::DoubleNegation => "Remove redundant double-negation",
            ExtraBooleanCastType::BooleanCall => "Remove redundant `Boolean` call",
        };
        mutation.replace_node(node.clone(), node_to_replace.clone());

        Some(JsRuleAction {
            category: ActionCategory::QuickFix,
            applicability: Applicability::MaybeIncorrect,
            message: markup! { {message} }.to_owned(),
            mutation,
        })
    }
}

/// Check if the SyntaxNode is a Double Negation. Including the edge case
/// ```js
/// !(!x)
/// ```
/// Return [Rule::State] `(JsAnyExpression, ExtraBooleanCastType)` if it is a DoubleNegation Expression
///
fn is_double_negation_ignore_parenthesis(
    syntax: &rome_rowan::SyntaxNode<rome_js_syntax::JsLanguage>,
) -> Option<(JsAnyExpression, ExtraBooleanCastType)> {
    if let Some(negation_expr) = is_negation(syntax) {
        let argument = negation_expr.argument().ok()?;
        match argument {
            JsAnyExpression::JsUnaryExpression(expr)
                if expr.operator().ok()? == JsUnaryOperator::LogicalNot =>
            {
                expr.argument()
                    .ok()
                    .map(|argument| (argument, ExtraBooleanCastType::DoubleNegation))
            }
            // Check edge case `!(!xxx)`
            JsAnyExpression::JsParenthesizedExpression(expr) => {
                expr.expression().ok().and_then(|expr| {
                    is_negation(expr.syntax()).and_then(|negation| {
                        Some((
                            negation.argument().ok()?,
                            ExtraBooleanCastType::DoubleNegation,
                        ))
                    })
                })
            }
            _ => None,
        }
    } else {
        None
    }
}
