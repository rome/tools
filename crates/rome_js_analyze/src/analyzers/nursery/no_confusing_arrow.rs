use rome_analyze::{context::RuleContext, declare_rule, Ast, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_js_syntax::JsArrowFunctionExpression;

declare_rule! {
    /// Disallow arrow functions where they could be confused with comparisons.
    ///
    /// Arrow functions (`=>`) are similar in syntax to some comparison operators (`>`, `<`, `<=`, and `>=`).
    /// This rule warns against using the arrow function syntax in places where it could be confused with a comparison operator.
    ///
    /// Source: https://eslint.org/docs/latest/rules/no-confusing-arrow
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// var x = a => 1 ? 2 : 3;
    /// var x = (a) => 1 ? 2 : 3;
    /// ```
    ///
    /// ## Valid
    ///
    /// ```js
    /// var x = a => (1 ? 2 : 3);
    /// var x = (a) => (1 ? 2 : 3);
    /// var x = (a) => {
    ///     return 1 ? 2 : 3;
    /// };
    /// var x = a => { return 1 ? 2 : 3; };
    /// ```
    ///
    pub(crate) NoConfusingArrow {
        version: "next",
        name: "noConfusingArrow",
        recommended: false,
    }
}

impl Rule for NoConfusingArrow {
    type Query = Ast<JsArrowFunctionExpression>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let arrow_fn = ctx.query();

        arrow_fn
            .body()
            .ok()?
            .as_any_js_expression()?
            .as_js_conditional_expression()
            .is_some()
            .then_some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        Some(RuleDiagnostic::new(
            rule_category!(),
            ctx.query().fat_arrow_token().ok()?.text_trimmed_range(),
            markup! {
                "Fat arrows can be confused with some comparison operators ("
                    <Emphasis>"<"</Emphasis>", "
                    <Emphasis>">"</Emphasis>", "
                    <Emphasis>"<="</Emphasis>", "
                    <Emphasis>">="</Emphasis>
                ")."
            },
        ))
    }

    // Formatter is not happy with the added parenthesis at the time of commit.
    // This should be fixed before enabling the action.
    //fn action(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<JsRuleAction> {
    //    let body_expr = ctx.query().body().ok()?.as_any_js_expression()?.clone();
    //
    //    let mut mutation = ctx.root().begin();
    //
    //    mutation.replace_node(
    //        body_expr.clone(),
    //        AnyJsExpression::from(make::js_parenthesized_expression(
    //            JsSyntaxToken::new_detached(JsSyntaxKind::L_PAREN, "(", [], []),
    //            body_expr,
    //            JsSyntaxToken::new_detached(JsSyntaxKind::R_PAREN, ")", [], []),
    //        )),
    //    );
    //
    //    Some(JsRuleAction {
    //        category: ActionCategory::QuickFix,
    //        applicability: Applicability::Always,
    //        message: markup! { "Wrap the function body in parenthesis." }.to_owned(),
    //        mutation,
    //    })
    //}
}
