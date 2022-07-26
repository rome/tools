use crate::JsRuleAction;
use rome_analyze::{
    context::RuleContext, declare_rule, ActionCategory, Ast, Rule, RuleCategory, RuleDiagnostic,
};
use rome_console::markup;
use rome_diagnostics::Applicability;
use rome_js_factory::make;
use rome_js_syntax::{JsAnyExpression, JsInExpression, JsInstanceofExpression, T};
use rome_rowan::{declare_node_union, AstNode, AstNodeExt, BatchMutationExt};

declare_rule! {
    /// Disallow using unsafe negation.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// !1 in [1,2];
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// /**test*/!/** test*/1 instanceof [1,2];
    /// ```
    ///
    /// ### Valid
    /// ```js
    /// -1 in [1,2];
    /// ~1 in [1,2];
    /// typeof 1 in [1,2];
    /// void 1 in [1,2];
    /// delete 1 in [1,2];
    /// +1 instanceof [1,2];
    /// ```
    pub(crate) NoUnsafeNegation {
        version: "0.7.0",
        name: "noUnsafeNegation",
        recommended: true
    }
}

impl Rule for NoUnsafeNegation {
    const CATEGORY: RuleCategory = RuleCategory::Lint;

    type Query = Ast<JsInOrInstanceOfExpression>;
    type State = ();
    type Signals = Option<Self::State>;

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let node = ctx.query();
        match node {
            JsInOrInstanceOfExpression::JsInstanceofExpression(expr) => {
                let left = expr.left().ok()?;
                if let Some(unary) = left.as_js_unary_expression() {
                    match unary.operator().ok()? {
                        rome_js_syntax::JsUnaryOperator::LogicalNot => Some(()),
                        _ => None,
                    }
                } else {
                    None
                }
            }
            JsInOrInstanceOfExpression::JsInExpression(expr) => {
                let left = expr.property().ok()?;
                if let Some(rome_js_syntax::JsAnyExpression::JsUnaryExpression(unary)) =
                    left.as_js_any_expression()
                {
                    match unary.operator().ok()? {
                        rome_js_syntax::JsUnaryOperator::LogicalNot => Some(()),
                        _ => None,
                    }
                } else {
                    None
                }
            }
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        Some(RuleDiagnostic::warning(
            node.range(),
            markup! {
                "The negation operator is used unsafely on the left side of this binary expression."
            },
        ))
    }

    fn action(ctx: &RuleContext<Self>, _: &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query();
        let mut mutation = ctx.root().begin();

        // The action could be splitted to three steps
        // 1. Remove `!` operator of unary expression
        // 2. Wrap the expression with `()`, convert the expression to a `JsParenthesizedExpression`
        // 3. Replace the `JsParenthesizedExpression` to `JsUnaryExpression` by adding a `JsUnaryOperator::LogicalNot`
        match node {
            JsInOrInstanceOfExpression::JsInstanceofExpression(expr) => {
                let left = expr.left().ok()?;
                let unary_expression = left.as_js_unary_expression()?;
                let argument = unary_expression.argument().ok()?;
                let next_expr = expr
                    .clone()
                    .replace_node_discard_trivia(left.clone(), argument)?;
                let next_parenthesis_expression = make::js_parenthesized_expression(
                    make::token(T!['(']),
                    rome_js_syntax::JsAnyExpression::JsInstanceofExpression(next_expr),
                    make::token(T![')']),
                );
                let next_unary_expression = make::js_unary_expression(
                    unary_expression.operator_token().ok()?,
                    JsAnyExpression::JsParenthesizedExpression(next_parenthesis_expression),
                );
                mutation.replace_node(
                    JsAnyExpression::JsInstanceofExpression(expr.clone()),
                    JsAnyExpression::JsUnaryExpression(next_unary_expression),
                );
            }
            JsInOrInstanceOfExpression::JsInExpression(expr) => {
                let left = expr.property().ok()?;
                let unary_expression = left.as_js_any_expression()?.as_js_unary_expression()?;
                let argument = unary_expression.argument().ok()?;
                let next_expr = expr.clone().replace_node_discard_trivia(
                    left.clone(),
                    rome_js_syntax::JsAnyInProperty::JsAnyExpression(argument),
                )?;
                let next_parenthesis_expression = make::js_parenthesized_expression(
                    make::token(T!['(']),
                    rome_js_syntax::JsAnyExpression::JsInExpression(next_expr),
                    make::token(T![')']),
                );
                let next_unary_expression = make::js_unary_expression(
                    unary_expression.operator_token().ok()?,
                    JsAnyExpression::JsParenthesizedExpression(next_parenthesis_expression),
                );
                mutation.replace_node(
                    JsAnyExpression::JsInExpression(expr.clone()),
                    JsAnyExpression::JsUnaryExpression(next_unary_expression),
                );
            }
        }

        Some(JsRuleAction {
            category: ActionCategory::QuickFix,
            applicability: Applicability::MaybeIncorrect,
            message: markup! { "Wrap the expression with a parenthesis" }.to_owned(),
            mutation,
        })
    }
}

declare_node_union! {
    /// Enum for [JsInstanceofExpression] and [JsInExpression]
    #[allow(dead_code)]
    pub(crate) JsInOrInstanceOfExpression  = JsInstanceofExpression  | JsInExpression
}
