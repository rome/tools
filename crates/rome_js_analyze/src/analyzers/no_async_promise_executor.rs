use rome_analyze::{context::RuleContext, declare_rule, Rule, RuleCategory, RuleDiagnostic};
use rome_console::markup;
use rome_js_syntax::{
    JsAnyExpression, JsArrowFunctionExpression, JsFunctionExpression, JsNewExpression,
    JsNewExpressionFields,
};
use rome_rowan::{declare_node_union, AstNode, AstSeparatedList};

declare_rule! {
    /// Disallows using an async function as a Promise executor.
    ///
    /// ## Examples
    /// ### Valid
    ///
    /// ```js
    ///   new Promise((resolve, reject) => {})
    /// ```
    /// ```js
    ///   new Promise((resolve, reject) => {}, async function unrelated() {})
    /// ```
    /// ```js
    ///   new Foo(async (resolve, reject) => {})
    /// ```
    /// ```js
    /// new Foo((( (resolve, reject) => {} )))
    /// ```
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// new Promise(async function foo(resolve, reject) {})
    /// ```
    ///
    /// ```js,expect_diagnostic
    ///   new Promise(async (resolve, reject) => {})
    /// ```
    ///
    /// ```js,expect_diagnostic
    ///   new Promise(((((async () => {})))))
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
            get_async_function_expression_like(expr)
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

/// Check if the expression is async function expression like, include the edge case
///  ```js
/// ((((((async function () {}))))))
/// ```
fn get_async_function_expression_like(
    expr: &JsAnyExpression,
) -> Option<JsAnyFunctionExpressionLike> {
    match expr {
        JsAnyExpression::JsFunctionExpression(func) => func
            .async_token()
            .map(|_| JsAnyFunctionExpressionLike::JsFunctionExpression(func.clone())),
        JsAnyExpression::JsArrowFunctionExpression(func) => func
            .async_token()
            .map(|_| JsAnyFunctionExpressionLike::JsArrowFunctionExpression(func.clone())),
        JsAnyExpression::JsParenthesizedExpression(expr) => {
            let inner_expression = expr.expression().ok()?;
            get_async_function_expression_like(&inner_expression)
        }
        _ => None,
    }
}

declare_node_union! {
    pub(crate) JsAnyFunctionExpressionLike = JsFunctionExpression | JsArrowFunctionExpression
}
