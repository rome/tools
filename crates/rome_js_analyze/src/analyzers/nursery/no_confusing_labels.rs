use rome_analyze::context::RuleContext;
use rome_analyze::{declare_rule, Ast, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_js_syntax::{AnyJsStatement, JsLabeledStatement};

declare_rule! {
    /// Disallow labeled statements that are not loops.
    ///
    /// Labeled statements in JavaScript are used in conjunction with `break` and `continue` to control flow around multiple loops.
    /// Their use for other statements is suspicious and unfamiliar.
    ///
    /// Source: https://eslint.org/docs/latest/rules/no-labels
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// label: f();
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// label: {
    ///     f();
    ///     break label;
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// label: if (a) {
    ///     f()
    ///     break label;
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// label: switch (a) {
    ///     case 0:
    ///         break label;
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// outer: while (a) {
    ///     while(b) {
    ///         break outer;
    ///     }
    /// }
    /// ```
    pub(crate) NoConfusingLabels {
        version: "12.0.0",
        name: "noConfusingLabels",
        recommended: true,
    }
}

impl Rule for NoConfusingLabels {
    type Query = Ast<JsLabeledStatement>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let labeled_stmt = ctx.query();
        match labeled_stmt.body().ok()? {
            AnyJsStatement::JsDoWhileStatement(_)
            | AnyJsStatement::JsForInStatement(_)
            | AnyJsStatement::JsForOfStatement(_)
            | AnyJsStatement::JsForStatement(_)
            | AnyJsStatement::JsWhileStatement(_) => None,
            _ => Some(()),
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        let labeled_stmt = ctx.query();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                labeled_stmt.label_token().ok()?.text_trimmed_range(),
                markup! {
                    "Unexpected "<Emphasis>"label"</Emphasis>"."
                },
            )
            .note("Only loops should be labeled.\nThe use of labels for other statements is suspicious and unfamiliar."),
        )
    }
}
