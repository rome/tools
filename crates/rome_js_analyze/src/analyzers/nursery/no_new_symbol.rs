use crate::{semantic_services::Semantic, JsRuleAction};
use rome_analyze::{
    context::RuleContext, declare_rule, ActionCategory, Rule, RuleCategory, RuleDiagnostic,
};
use rome_console::markup;
use rome_diagnostics::Applicability;
use rome_js_factory::make;
use rome_js_syntax::{JsAnyExpression, JsCallExpression, JsNewExpression, JsNewExpressionFields};
use rome_rowan::{AstNode, AstNodeExt, BatchMutationExt};

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
        version: "0.10.0",
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

        if reference.has_name("Symbol") {
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

    fn action(ctx: &RuleContext<Self>, _: &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query();
        let mut mutation = ctx.root().begin();

        let call_expression = convert_new_expression_to_call_expression(node)?;
        mutation.replace_node(
            JsAnyExpression::JsNewExpression(node.clone()),
            JsAnyExpression::JsCallExpression(call_expression),
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
    let JsNewExpressionFields {
        new_token,
        callee,
        arguments,
        ..
    } = expr.as_fields();

    let new_token = new_token.ok()?;
    let mut callee = callee.ok()?;
    let arguments = arguments?;

    if new_token.has_leading_comments() || new_token.has_trailing_comments() {
        let first_token = callee.syntax().first_token()?;

        let mut trivia = vec![];
        trivia.extend(new_token.leading_trivia().pieces());
        trivia.extend(new_token.trailing_trivia().pieces());
        trivia.extend(first_token.leading_trivia().pieces());

        callee = callee.replace_token_discard_trivia(
            first_token.clone(),
            first_token.with_leading_trivia_pieces(trivia),
        )?;
    }

    Some(make::js_call_expression(callee, arguments).build())
}
