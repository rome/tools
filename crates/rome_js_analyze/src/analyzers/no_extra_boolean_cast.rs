use rome_analyze::{
    context::RuleContext, declare_rule, ActionCategory, Ast, Rule, RuleCategory, RuleDiagnostic,
};
use rome_console::markup;
use rome_diagnostics::Applicability;
use rome_js_factory::make;
use rome_js_syntax::{
    JsAnyExpression, JsAnyStatement, JsCallExpression, JsConditionalExpression, JsDoWhileStatement,
    JsForStatement, JsForStatementFields, JsIfStatement, JsNewExpression, JsSyntaxKind,
    JsSyntaxNode, JsUnaryExpression, JsWhileStatement, T,
};
use rome_rowan::{declare_node_union, AstNode, AstNodeExt, SyntaxNodeCast};

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
    type State = ();
    type Signals = Option<Self::State>;

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let n = ctx.query();
        let syntax = n.syntax();
        let parent_syntax = syntax.parent()?;

        let in_boolean_cast_context =
            if let Some(true) = is_in_boolean_context(syntax, &parent_syntax) {
                true
            } else if let Some(true) = is_boolean_constructor_call(&parent_syntax) {
                true
            } else if let Some(true) = is_negation(&parent_syntax) {
                true
            } else if let Some(true) = is_boolean_call(&parent_syntax) {
                true
            } else {
                false
            };

        if in_boolean_cast_context {
            if is_negation(syntax).unwrap_or_default()
                && is_negation(
                    JsUnaryExpression::cast(syntax.clone())?
                        .argument()
                        .ok()?
                        .syntax(),
                )
                .unwrap_or_default()
            {
                Some(())
            } else {
                None
            }
        } else {
            None
        }
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

    fn action(ctx: &RuleContext<Self>, _: &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query();
        let root = ctx.root();
        Some(JsRuleAction {
            category: ActionCategory::QuickFix,
            applicability: Applicability::MaybeIncorrect,
            message: markup! { "Use a while loop" }.to_owned(),
            root,
        })
    }
}
