use crate::{semantic_services::Semantic, JsRuleAction};
use rome_analyze::{context::RuleContext, declare_rule, ActionCategory, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_diagnostics::Applicability;
use rome_js_factory::make;
use rome_js_syntax::{global_identifier, AnyJsExpression, JsCallExpression, JsNewExpression};
use rome_rowan::{chain_trivia_pieces, AstNode, BatchMutationExt};

declare_rule! {
    /// Disallow `new` operators with the `Symbol` object.
    ///
    /// `Symbol` cannot be instantiated. This results in throwing a `TypeError`.
    ///
    /// Source: https://eslint.org/docs/latest/rules/no-new-symbol
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
        version: "0.10.0",
        name: "noNewSymbol",
        recommended: true,
    }
}

impl Rule for NoNewSymbol {
    type Query = Semantic<JsNewExpression>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let callee = ctx.query().callee().ok()?;
        let (reference, name) = global_identifier(&callee)?;
        if name.text() != "Symbol" {
            return None;
        }
        ctx.model().binding(&reference).is_none().then_some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        Some(RuleDiagnostic::new(
            rule_category!(),
            ctx.query().range(),
            markup! {
                <Emphasis>"Symbol"</Emphasis>" cannot be called as a constructor."
            },
        ))
    }

    fn action(ctx: &RuleContext<Self>, _: &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query();
        let call_expression = convert_new_expression_to_call_expression(node)?;
        let mut mutation = ctx.root().begin();
        mutation.replace_node_discard_trivia::<AnyJsExpression>(
            node.clone().into(),
            call_expression.into(),
        );
        Some(JsRuleAction {
            category: ActionCategory::QuickFix,
            applicability: Applicability::MaybeIncorrect,
            message: markup! { "Remove "<Emphasis>"new"</Emphasis>"." }.to_owned(),
            mutation,
        })
    }
}

fn convert_new_expression_to_call_expression(expr: &JsNewExpression) -> Option<JsCallExpression> {
    let new_token = expr.new_token().ok()?;
    let mut callee = expr.callee().ok()?;
    if new_token.has_leading_comments() || new_token.has_trailing_comments() {
        callee = callee.prepend_trivia_pieces(chain_trivia_pieces(
            new_token.leading_trivia().pieces(),
            new_token.trailing_trivia().pieces(),
        ))?;
    }
    Some(make::js_call_expression(callee, expr.arguments()?).build())
}
