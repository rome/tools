use crate::{semantic_services::Semantic, JsRuleAction};
use rome_analyze::{context::RuleContext, declare_rule, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_js_syntax::TsModuleDeclaration;
use rome_rowan::AstNode;

declare_rule! {
    /// Disallow the use of TypeScript's `namespace`s.
    ///
    /// Namespaces are an old way to organize your code in TypeScript.
    /// They are not recommended anymore and should be replaced by ES6 modules
    /// (the `import`/`export` syntax).
    ///
    /// Add a link to the corresponding ESLint rule (if any):
    ///
    /// Source: https://typescript-eslint.io/rules/no-namespace
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```ts,expect_diagnostic
    /// module foo {}
    /// declare module foo {}
    /// ```
    ///
    /// ```ts,expect_diagnostic
    /// namespace foo {}
    /// declare namespace foo {}
    /// ```
    ///
    /// ## Valid
    ///
    /// ```ts
    /// import foo from 'foo';
    /// export { bar };
    ///
    /// declare global {}
    ///
    /// declare module 'foo' {}
    /// ```
    ///
    pub(crate) NoNamespace {
        version: "next",
        name: "noNamespace",
        recommended: true,
    }
}

impl Rule for NoNamespace {
    type Query = Semantic<TsModuleDeclaration>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(_: &RuleContext<Self>) -> Self::Signals {
        Some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();

        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.syntax().text_trimmed_range(),
                markup! {
                    "TypeScript's namespaces are an oudated way to organize code."
                },
            )
            .note(markup! {
                "Prefer the ES6 modules (import/export) over namespaces."
            }),
        )
    }

    fn action(_: &RuleContext<Self>, _: &Self::State) -> Option<JsRuleAction> {
        None
    }
}
