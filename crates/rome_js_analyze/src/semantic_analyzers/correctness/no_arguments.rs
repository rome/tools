use crate::{semantic_services::Semantic, JsRuleAction};
use rome_analyze::{context::RuleContext, declare_rule, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_js_syntax::JsReferenceIdentifier;
use rome_rowan::AstNode;

declare_rule! {
    /// Disallow the use of ```arguments```
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// function f() {
    ///    console.log(arguments);
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```cjs
    /// function f() {
    ///     let arguments = 1;
    ///     console.log(arguments);
    /// }
    /// ```
    pub(crate) NoArguments {
        version: "0.7.0",
        name: "noArguments",
        recommended: true,
    }
}

impl Rule for NoArguments {
    type Query = Semantic<JsReferenceIdentifier>;
    type State = ();
    type Signals = Option<Self::State>;

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let reference = ctx.query();
        let value_token = reference.value_token().ok()?;

        let name = value_token.text_trimmed();
        if name == "arguments" {
            let model = ctx.model();
            let declaration = model.declaration(reference);

            if declaration.is_none() {
                return Some(());
            }
        }

        None
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();

        Some(RuleDiagnostic::new(rule_category!(),
            node.syntax().text_trimmed_range(),
            markup! {
                "Use the "<Emphasis>"rest parameters"</Emphasis>" instead of "<Emphasis>"arguments"</Emphasis>"."
            },
        ).footer_note(markup! {<Emphasis>"arguments"</Emphasis>" does not have "<Emphasis>"Array.prototype"</Emphasis>" methods and can be inconvenient to use."}))
    }

    fn action(_: &RuleContext<Self>, _: &Self::State) -> Option<JsRuleAction> {
        None
    }
}
