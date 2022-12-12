use rome_analyze::context::RuleContext;
use rome_analyze::{declare_rule, ActionCategory, Ast, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_diagnostics::Applicability;
use rome_js_syntax::JsWithStatement;

use rome_rowan::{AstNode, BatchMutationExt};

use crate::JsRuleAction;

declare_rule! {
    /// Disallow `with` statements in non-strict contexts.
    ///
    /// The `with` statement is potentially problematic because it adds members of an object to the current
    /// scope, making it impossible to tell what a variable inside the block actually refers to.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```cjs,expect_diagnostic
    /// function f() {
    ///   with (point) {
    ///     r = Math.sqrt(x * x + y * y); // is r a member of point?
    ///   }
    /// }
    /// ```
    pub(crate) NoWith {
        version: "12.0.0",
        name: "noWith",
        recommended: true,
    }
}

impl Rule for NoWith {
    type Query = Ast<JsWithStatement>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(_ctx: &RuleContext<Self>) -> Option<Self::State> {
        Some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        Some(RuleDiagnostic::new(
            rule_category!(),
            node.range(),
            markup! {
                "Unexpected use of "<Emphasis>"with"</Emphasis>" statement."
            },
        ).note(
            "The with statement is potentially problematic because it adds members of an object to the current\nscope, making it impossible to tell what a variable inside the block actually refers to."
        ))
    }
}
