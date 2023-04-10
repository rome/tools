use rome_analyze::{context::RuleContext, declare_rule, Ast, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_js_syntax::{AnyJsExpression, JsCallExpression};
use rome_rowan::AstNode;

declare_rule! {
    /// Prefer for...of statement instead of Array.forEach.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// els.forEach(el => {
    ///   el
    /// })
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// els['forEach'](el => {
    ///   el
    /// })
    /// ```
    ///
    /// ## Valid
    ///
    /// ```js
    /// for (const el of els) {
    ///   el
    /// }
    /// ```
    ///
    pub(crate) NoForEach {
        version: "next",
        name: "noForEach",
        recommended: false,
    }
}

impl Rule for NoForEach {
    type Query = Ast<JsCallExpression>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();

        let is_for_each = match node.callee().ok()?.omit_parentheses() {
            AnyJsExpression::JsStaticMemberExpression(expression) => {
                expression
                    .member()
                    .ok()?
                    .as_js_name()?
                    .value_token()
                    .ok()?
                    .text_trimmed()
                    == "forEach"
            }
            AnyJsExpression::JsComputedMemberExpression(expression) => {
                expression
                    .member()
                    .ok()?
                    .as_any_js_literal_expression()?
                    .as_js_string_literal_expression()?
                    .inner_string_text()
                    .ok()?
                    .text()
                    == "forEach"
            }
            _ => return None,
        };

        is_for_each.then(|| ())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();

        Some(RuleDiagnostic::new(
            rule_category!(),
            node.syntax().text_trimmed_range(),
            markup! {
                "Prefer for...of instead of Array.forEach"
            },
        ))
    }
}
