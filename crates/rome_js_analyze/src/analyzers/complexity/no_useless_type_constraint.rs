use rome_analyze::context::RuleContext;
use rome_analyze::{declare_rule, Ast, Rule, RuleDiagnostic};
use rome_console::markup;

use rome_js_syntax::TsTypeConstraintClause;
use rome_rowan::AstNode;

declare_rule! {
    /// Disallow comparing against `-0`
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```ts,expect_diagnostic
    ///
    /// interface FooAny<T extends any> {}
    ///
    /// type BarAny<T extends any> = {};
    ///
    /// class BazAny<T extends any> {
    ///   quxAny<U extends any>() {}
    /// }
    ///
    /// const QuuxAny = <T extends any>() => {};
    ///
    /// function QuuzAny<T extends any>() {}
    /// ```
    ///
    /// ### Valid
    ///
    /// ```ts
    /// interface Foo<T> {}
    ///
    /// type Bar<T> = {};
    ///```
    pub(crate) NoUselessTypeConstraint {
        version: "0.7.0",
        name: "noUselessTypeConstraint",
        recommended: true,
    }
}

impl Rule for NoUselessTypeConstraint {
    type Query = Ast<TsTypeConstraintClause>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(_ctx: &RuleContext<Self>) -> Option<Self::State> {
        Some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        let ty = node.ty().ok()?;

        if ty.as_ts_any_type().is_none() && ty.as_ts_unknown_type().is_none() {
            return None;
        }

        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.syntax().text_trimmed_range(),
                markup! {
                    "Useless type constraint."
                },
            )
            .note("Constraining the type to `any` or `unknown` doesn't have any effect."),
        )
    }
}
