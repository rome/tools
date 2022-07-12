use rome_analyze::{
    context::RuleContext, declare_rule, ActionCategory, Ast, Rule, RuleCategory, RuleDiagnostic,
};
use rome_console::markup;
use rome_diagnostics::Applicability;
use rome_js_syntax::{
    JsAnyExpression, JsCallExpression, JsDoWhileStatement, JsForStatement, JsIfStatement,
    JsNewExpression, JsSyntaxKind, JsSyntaxNode, JsUnaryExpression, JsUnaryOperator,
    JsWhileStatement,
};
use rome_rowan::{AstNode, AstNodeExt, AstSeparatedList, SyntaxNodeCast};

use crate::JsRuleAction;

declare_rule! {
    /// Enforce the use of `while` loops instead of `for` loops when the
    /// initializer and update expressions are not needed
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// for (; x.running;) {
    ///     x.step();
    /// }
    /// ```
    pub(crate) NoExtraBooleanCast = "noExtraBooleanCast"
}

fn is_in_boolean_context(node: &JsSyntaxNode, parent: &JsSyntaxNode) -> Option<bool> {
    let parent = parent.clone();
    match parent.kind() {
        JsSyntaxKind::JS_IF_STATEMENT => {
            let stmt = parent.cast::<JsIfStatement>()?;
            Some(stmt.test().ok()?.syntax() == node)
        }
        JsSyntaxKind::JS_DO_WHILE_STATEMENT => {
            let stmt = parent.cast::<JsDoWhileStatement>()?;
            Some(stmt.test().ok()?.syntax() == node)
        }
        JsSyntaxKind::JS_WHILE_STATEMENT => {
            let stmt = parent.cast::<JsWhileStatement>()?;
            Some(stmt.test().ok()?.syntax() == node)
        }
        JsSyntaxKind::JS_FOR_STATEMENT => {
            let stmt = parent.cast::<JsForStatement>()?;
            Some(stmt.test()?.syntax() == node)
        }
        JsSyntaxKind::JS_CONDITIONAL_EXPRESSION => {
            let expr = parent.cast::<JsForStatement>()?;
            Some(expr.test()?.syntax() == node)
        }
        _ => None,
    }
}

fn is_boolean_constructor_call(node: &JsSyntaxNode) -> Option<bool> {
    JsNewExpression::cast(node.clone()).and_then(|expr| {
        let callee = expr.callee().ok()?;
        if let JsAnyExpression::JsIdentifierExpression(ident) = callee {
            Some(ident.name().ok()?.syntax().text_trimmed() == "Boolean")
        } else {
            None
        }
    })
}

fn is_boolean_call(node: &JsSyntaxNode) -> Option<bool> {
    JsCallExpression::cast(node.clone()).and_then(|expr| {
        let callee = expr.callee().ok()?;
        if let JsAnyExpression::JsIdentifierExpression(ident) = callee {
            Some(ident.name().ok()?.syntax().text_trimmed() == "Boolean")
        } else {
            None
        }
    })
}

fn is_negation(node: &JsSyntaxNode) -> Option<bool> {
    JsUnaryExpression::cast(node.clone()).and_then(|expr| {
        Some(matches!(
            expr.operator().ok()?,
            rome_js_syntax::JsUnaryOperator::LogicalNot
        ))
    })
}

impl Rule for NoExtraBooleanCast {
    const CATEGORY: RuleCategory = RuleCategory::Lint;

    type Query = Ast<JsAnyExpression>;
    type State = JsAnyExpression;
    type Signals = Option<Self::State>;

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let n = ctx.query();
        let syntax = n.syntax().clone();
        let parent_syntax = syntax.parent()?;

        let in_boolean_cast_context = is_in_boolean_context(&syntax, &parent_syntax)
            .unwrap_or_default()
            || is_boolean_constructor_call(&parent_syntax).unwrap_or_default()
            || is_negation(&parent_syntax).unwrap_or_default()
            || is_boolean_call(&parent_syntax).unwrap_or_default();

        if in_boolean_cast_context {
            if is_negation(&syntax).unwrap_or_default() {
                let argument = JsUnaryExpression::cast(syntax)?.argument().ok()?;
                match argument {
                    JsAnyExpression::JsUnaryExpression(expr)
                        if expr.operator().ok()? == JsUnaryOperator::LogicalNot =>
                    {
                        return expr.argument().ok();
                    }
                    _ => return None,
                }
            }
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
                                .and_then(JsAnyExpression::cast);
                        } else {
                            return None;
                        }
                    }
                } else {
                    return None;
                }
                None
            });
            // {
            //     return Some(());
            // }
            // if is_boolean_call(syntax).unwrap_or_default() {}
        }
        None
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();

        Some(RuleDiagnostic::warning(
            node.range(),
            markup! {
                "Use "<Emphasis>"while"</Emphasis>" loops instead of "<Emphasis>"for"</Emphasis>" loops."
            },
        ))
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query();
        let root = ctx.root().replace_node(node.clone(), state.clone())?;

        Some(JsRuleAction {
            category: ActionCategory::QuickFix,
            applicability: Applicability::MaybeIncorrect,
            message: markup! { "Use a while loop" }.to_owned(),
            root,
        })
    }
}
