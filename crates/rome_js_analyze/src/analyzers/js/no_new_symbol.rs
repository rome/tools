use crate::{semantic_services::Semantic, JsRuleAction};
use rome_analyze::{context::RuleContext, declare_rule, Rule, RuleCategory, RuleDiagnostic};
use rome_console::markup;
use rome_js_syntax::JsNewExpression;
use rome_rowan::AstNode;

declare_rule! {
    /// Disallow `new` operators with the `Symbol` object
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// var foo = new Symbol('foo');
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// var bar = Symbol('bar');
    /// function baz() {
    ///     function Symbol() { }
    ///     new Symbol();
    /// }
    /// ```
    pub(crate) NoNewSymbol {
        version: "0.7.0",
        name: "noNewSymbol",
        recommended: true,
    }
}

impl Rule for NoNewSymbol {
    const CATEGORY: RuleCategory = RuleCategory::Lint;

    type Query = Semantic<JsNewExpression>;
    type State = ();
    type Signals = Option<Self::State>;

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let new_expression = ctx.query();
        let callee = new_expression.callee().ok()?;

        let ident = callee.as_js_identifier_expression()?;
        let reference = ident.name().ok()?;

        let name = ident.text();

        if name == "Symbol" {
            let model = ctx.model();
            let declaration = model.declaration(&reference);

            if declaration.is_none() {
                return Some(());
            }
        }

        None
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();

        Some(RuleDiagnostic::new(
            node.syntax().text_trimmed_range(),
            markup! {
                <Emphasis>"Symbol"</Emphasis>" cannot be called as a constructor."
            },
        ))
    }

    fn action(_: &RuleContext<Self>, _: &Self::State) -> Option<JsRuleAction> {
        None
    }
}
