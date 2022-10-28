use rome_analyze::context::RuleContext;
use rome_analyze::{declare_rule, Ast, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_js_syntax::TsAnyType;
use rome_rowan::AstNode;

declare_rule! {
    /// Disallow the `any` type usage
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```ts,expect_diagnostic
    /// let variable: any = 1;
    /// ```
    ///
    /// ```ts,expect_diagnostic
    /// class SomeClass {
    ///   message: Array<Array<any>>;
    /// }
    /// ```
    ///
    /// ```ts,expect_diagnostic
    /// function fn(param: Array<any>): void {}
    /// ```
    ///
    /// ### Valid
    ///
    /// ```ts
    /// let variable: number = 1;
    /// let variable2 = 1;
    /// ```
    ///
    /// ```ts
    /// class SomeClass {
    ///   message: Array<Array<unknown>>;
    /// }
    /// ```
    ///
    /// ```ts
    /// function fn(param: Array<Array<unknown>>): Array<unknown> {}
    /// ```
    ///
    /// ```
    pub(crate) NoExplicitAny {
        version: "10.0.0",
        name: "noExplicitAny",
        recommended: false,
    }
}

impl Rule for NoExplicitAny {
    type Query = Ast<TsAnyType>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(_: &RuleContext<Self>) -> Self::Signals {
        Some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        let diagnostic = RuleDiagnostic::new(
            rule_category!(),
            ctx.query().range(),
            markup! {"Unexpected "<Emphasis>"any"</Emphasis>". Specify a different type."}
                .to_owned(),
        );

        Some(diagnostic)
    }
}
