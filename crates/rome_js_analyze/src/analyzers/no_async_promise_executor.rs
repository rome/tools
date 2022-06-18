use rome_analyze::{
    context::RuleContext, declare_rule, ActionCategory, Rule, RuleCategory, RuleDiagnostic,
};
use rome_console::markup;
use rome_diagnostics::Applicability;
use rome_js_factory::make;
use rome_js_syntax::{
    JsAnyExpression, JsAnyStatement, JsArrowFunctionExpression, JsForStatement,
    JsForStatementFields, JsFunctionExpression, JsNewExpression, JsNewExpressionFields, T,
};
use rome_rowan::{declare_node_union, AstNode, AstNodeExt, AstSeparatedList};

use crate::JsRuleAction;

declare_rule! {
    /// Disallows using an async function as a Promise executor.
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
    pub(crate) NoAsyncPromiseExecutor = "noAsyncPromiseExecutor"
}

impl Rule for NoAsyncPromiseExecutor {
    const CATEGORY: RuleCategory = RuleCategory::Lint;

    type Query = JsNewExpression;
    type State = JsAnyFunctionExpressionLike;

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let node = ctx.query();
        let JsNewExpressionFields {
            new_token: _,
            callee,
            type_arguments: _,
            arguments,
        } = node.as_fields();
        let callee = callee.ok()?;
        let is_promise_constructor = callee
            .as_js_identifier_expression()
            .and_then(|ident| ident.name().ok())
            .map_or(false, |name| name.syntax().text() == "Promise");
        if !is_promise_constructor {
            return None;
        }

        // get first argument of the `Promise` constructor
        let first_arg = arguments?.args().iter().next()?.ok()?;

        if let Some(expr) = first_arg.as_js_any_expression() {
            match expr {
                JsAnyExpression::JsFunctionExpression(func) => func
                    .async_token()
                    .map(|_| JsAnyFunctionExpressionLike::JsFunctionExpression(func.clone())),
                JsAnyExpression::JsArrowFunctionExpression(func) => func
                    .async_token()
                    .map(|_| JsAnyFunctionExpressionLike::JsArrowFunctionExpression(func.clone())),
                _ => None,
            }
        } else {
            None
        }
    }

    fn diagnostic(_: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(RuleDiagnostic::warning(
            state.range(),
            markup! {
                "Promise executor functions should not be async."
            },
        ))
    }
}

declare_node_union! {
    pub(crate) JsAnyFunctionExpressionLike = JsFunctionExpression | JsArrowFunctionExpression
}
