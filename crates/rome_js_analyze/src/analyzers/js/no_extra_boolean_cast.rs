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
use rome_rowan::{AstNode, AstNodeExt, AstSeparatedList, SyntaxNodeCast};

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
    /// ```
    ///
    /// ```js
    /// !x;
    /// ```
    ///
    /// ```js
    /// !!x;
    /// ```
    pub(crate) NoExtraBooleanCast = "noExtraBooleanCast"
}

fn is_in_boolean_context(node: &JsSyntaxNode, parent: &JsSyntaxNode) -> Option<bool> {
    let parent = parent.clone();
    match parent.kind() {
        JsSyntaxKind::JS_IF_STATEMENT => parent
            .cast::<JsIfStatement>()
            .and_then(|stmt| Some(stmt.test().ok()?.syntax() == node)),
        JsSyntaxKind::JS_DO_WHILE_STATEMENT => parent
            .cast::<JsDoWhileStatement>()
            .and_then(|stmt| Some(stmt.test().ok()?.syntax() == node)),
        JsSyntaxKind::JS_WHILE_STATEMENT => parent
            .cast::<JsWhileStatement>()
            .and_then(|stmt| Some(stmt.test().ok()?.syntax() == node)),
        JsSyntaxKind::JS_FOR_STATEMENT => parent
            .cast::<JsForStatement>()
            .and_then(|stmt| Some(stmt.test()?.syntax() == node)),
        JsSyntaxKind::JS_CONDITIONAL_EXPRESSION => parent
            .cast::<JsConditionalExpression>()
            .and_then(|expr| Some(expr.test().ok()?.syntax() == node)),
        _ => None,
    }
}

/// Checks if the node is a `Boolean` Constructor Call
/// ## Example
/// ```js
/// new Boolean(x);
/// ```
/// The checking algorithm of [JsNewExpression] is a little different from [JsCallExpression] due to
/// the ungram definition of [JsNewExpression] is different from [JsCallExpression], the arguments of [JsNewExpression] is optional
/// ```text,no_run
/// // new expression
/// JsNewExpression =
/// 'new'
/// callee: JsAnyExpression
/// type_arguments: TsTypeArguments?
/// arguments: JsCallArguments?
/// // call expression
/// JsCallExpression =
/// callee: JsAnyExpression
/// optional_chain: '?.'?
/// type_arguments: TsTypeArguments?
/// arguments: JsCallArguments
/// ```
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
fn is_negation(node: &JsSyntaxNode) -> Option<bool> {
	Some(JsUnaryExpression::cast(node.clone())?.operator().ok()? == JsUnaryOperator::LogicalNot)
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
        let parent_node_in_boolean_cast_context = is_in_boolean_context(&syntax, &parent_syntax)
            .unwrap_or_default()
            || is_boolean_constructor_call(&parent_syntax).unwrap_or_default()
            || is_negation(&parent_syntax).unwrap_or_default()
            || is_boolean_call(&parent_syntax).unwrap_or_default();
        // Convert `!!x` -> `x` if parent `SyntaxNode` in any boolean `Type Coercion` context
        if parent_node_in_boolean_cast_context {
            if is_negation(&syntax).unwrap_or_default() {
                let argument = JsUnaryExpression::cast(syntax)?.argument().ok()?;
                match argument {
                    JsAnyExpression::JsUnaryExpression(expr)
                        if expr.operator().ok()? == JsUnaryOperator::LogicalNot =>
                    {
                        return expr
                            .argument()
                            .ok()
                            .map(|argument| (argument, ExtraBooleanCastType::DoubleNegation));
                    }
                    _ => {
                        return None;
                    }
                }
            }

            // Convert `Boolean(x)` -> `x` if parent `SyntaxNode` in any boolean `Type Coercion` context
            // Only if `Boolean` Call Expression have one `JsAnyExpression` argument
            return JsCallExpression::cast(syntax).and_then(|expr| {
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
                        } else {
                            return None;
                        }
                    }
                } else {
                    return None;
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
            RuleDiagnostic::warning(
                node.range(),
                markup! {
                    ""{title}""
                },
            )
            .footer_note(note),
        )
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query();
        let (node_to_replace, extra_boolean_cast_type) = state;
        let message = match extra_boolean_cast_type {
            ExtraBooleanCastType::DoubleNegation => "Remove redundant double-negation",
            ExtraBooleanCastType::BooleanCall => "Remove redundant `Boolean` call",
        };
        let root = ctx
            .root()
            .replace_node(node.clone(), node_to_replace.clone())?;

        Some(JsRuleAction {
            category: ActionCategory::QuickFix,
            applicability: Applicability::MaybeIncorrect,
            message: markup! { ""{message}"" }.to_owned(),
            root,
        })
    }
}
