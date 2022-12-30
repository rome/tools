use crate::semantic_services::Semantic;
use rome_analyze::{context::RuleContext, declare_rule, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_js_semantic::{Reference, ReferencesExtensions};
use rome_js_syntax::JsIdentifierBinding;

declare_rule! {
    /// Disallow duplicate class members.
    ///
    /// If there are declarations of the same name in class members,
    /// the last declaration overwrites other declarations silently.
    /// It can cause unexpected behaviors.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// class Foo {
    ///   bar() { }
    ///   bar() { }
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// class Foo {
    ///   bar() { }
    ///   get bar() { }
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// class Foo {
    ///   bar;
    ///   bar() { }
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// class Foo {
    ///   static bar() { }
    ///   static bar() { }
    /// }
    /// ```
    ///
    /// ## Valid
    ///
    /// ```js
    /// class Foo {
    ///   bar() { }
    ///   qux() { }
    /// }
    /// ```
    ///
    /// ```js
    /// class Foo {
    ///   set bar(value) { }
    ///   get bar() { }
    /// }
    /// ```
    ///
    /// ```js
    /// class Foo {
    ///   bar;
    ///   qux;
    /// }
    /// ```
    ///
    /// ```js
    /// class Foo {
    ///   bar;
    ///   qux() { }
    /// }
    /// ```
    ///
    /// ```js
    /// class Foo {
    ///   static bar() { }
    ///   bar() { }
    /// }
    /// ```
    ///
    pub(crate) NoDuplicateClassMembers {
        version: "next",
        name: "noDuplicateClassMembers",
        recommended: true,
    }
}

impl Rule for NoDuplicateClassMembers {
    type Query = Semantic<JsIdentifierBinding>;
    type State = Reference;
    type Signals = Vec<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Vec<Self::State> {
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
                    "Variable is read here"
                },
            )
            .note(markup! {
                "This note will give you more information"
            }),
        )
    }
}
