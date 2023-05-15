use rome_analyze::context::RuleContext;
use rome_analyze::{declare_rule, ActionCategory, Ast, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_diagnostics::Applicability;
use rome_js_syntax::{TsTypeConstraintClause};
use rome_rowan::{AstNode, AstNodeList, BatchMutationExt, Direction, SyntaxElement};

use crate::JsRuleAction;

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

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        // let node = ctx.query();
        None
    }
}
