use rome_analyze::context::RuleContext;
use rome_analyze::{declare_rule, ActionCategory, Ast, Rule, RuleDiagnostic};
use rome_console::markup;

use rome_diagnostics::Applicability;
use rome_js_syntax::TsTypeConstraintClause;
use rome_rowan::{AstNode, BatchMutationExt};

use crate::JsRuleAction;

declare_rule! {
    /// Disallow using `any` or `unknown` as type constraint.
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
    type State = TsTypeConstraintClause;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        let node = ctx.query();
        let ty = node.ty().ok()?;

        if ty.as_ts_any_type().is_some() || ty.as_ts_unknown_type().is_some() {
            Some(node.clone())
        } else {
            None
        }
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        state.ty().ok()?;

        Some(
            RuleDiagnostic::new(
                rule_category!(),
                state.syntax().text_trimmed_range(),
                markup! {
                    "Useless type constraint."
                },
            )
            .note("Constraining the type to `any` or `unknown` doesn't have any effect."),
        )
    }

    fn action(ctx: &RuleContext<Self>, node_replace: &Self::State) -> Option<JsRuleAction> {
        let mut mutation = ctx.root().begin();
        mutation.remove_node(node_replace.clone());

        Some(JsRuleAction {
            category: ActionCategory::QuickFix,
            applicability: Applicability::MaybeIncorrect,
            message: markup! { "Remove useless type constraint." }.to_owned(),
            mutation,
        })
    }
}
