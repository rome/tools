use crate::semantic_services::Semantic;
use rome_analyze::{
    context::RuleContext, declare_rule, Rule, RuleDiagnostic,
};
use rome_console::markup;
use rome_js_semantic::{Reference, ReferencesExtensions};
use rome_js_syntax::JsIdentifierBinding;

declare_rule! {
    /// Succinct description of the rule.
    ///
    /// Put context and details about the rule.
    /// As a starting point, you can take the description of the corresponding _ESLint_ rule (if any).
    ///
    /// Try to stay consistent with the descriptions of implemented rules.
    ///
    /// Add a link to the corresponding ESLint rule (if any):
    ///
    /// Source: https://eslint.org/docs/latest/rules/rule-name
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// var a = 1;
    /// a = 2;
    /// ```
    ///
    /// ## Valid
    ///
    /// ```js
    /// var a = 1;
    /// ```
    ///
    pub(crate) NoStaticOnlyClass {
        version: "next",
        name: "noStaticOnlyClass",
        recommended: false,
    }
}

impl Rule for NoStaticOnlyClass {
    type Query = Semantic<JsIdentifierBinding>;
    type State = Reference;
    type Signals = Vec<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let binding = ctx.query();
        let model = ctx.model();

        binding.all_references(model).collect()
    }

    fn diagnostic(_: &RuleContext<Self>, reference: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                reference.syntax().text_trimmed_range(),
                markup! {
                    "Variable is read here."
                },
            )
            .note(markup! {
                "This note will give you more information."
            }),
        )
    }
}
