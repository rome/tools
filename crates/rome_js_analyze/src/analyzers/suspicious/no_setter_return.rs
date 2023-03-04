use rome_analyze::context::RuleContext;
use rome_analyze::{declare_rule, Ast, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_js_syntax::{JsReturnStatement, JsSetterClassMember, JsSetterObjectMember};
use rome_rowan::{declare_node_union, AstNode};

use crate::control_flow::AnyJsControlFlowRoot;

declare_rule! {
    /// Disallow returning a value from a setter
    ///
    /// While returning a value from a setter does not produce an error, the returned value is being ignored. Therefore, returning a value from a setter is either unnecessary or a possible error.
    ///
    /// Only returning without a value is allowed, as it’s a control flow statement.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// class A {
    ///     set foo(x) {
    ///         return x;
    ///     }
    /// }
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const b = {
    ///     set foo(x) {
    ///         return x;
    ///     },
    /// };
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// const c = {
    ///     set foo(x) {
    ///         if (x) {
    ///             return x;
    ///         }
    ///     },
    /// };
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// // early-return
    /// class A {
    ///     set foo(x) {
    ///         if (x) {
    ///             return;
    ///         }
    ///     }
    /// }
    /// ```
    ///
    /// ```js
    /// // not a setter
    /// class B {
    ///   set(x) {
    ///     return x;
    ///   }
    /// }
    /// ```
    ///
    /// ```
    pub(crate) NoSetterReturn {
        version: "11.0.0",
        name: "noSetterReturn",
        recommended: true,
    }
}

declare_node_union! {
    pub(crate) JsSetterMember = JsSetterClassMember | JsSetterObjectMember
}

impl Rule for NoSetterReturn {
    type Query = Ast<JsReturnStatement>;
    type State = JsSetterMember;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let ret = ctx.query();
        // Do not take arg-less returns into account
        let _arg = ret.argument()?;
        let setter = ret
            .syntax()
            .ancestors()
            .find(|x| AnyJsControlFlowRoot::can_cast(x.kind()))
            .and_then(JsSetterMember::cast);
        setter
    }

    fn diagnostic(ctx: &RuleContext<Self>, setter: &Self::State) -> Option<RuleDiagnostic> {
        let ret = ctx.query();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                ret.range(),
                markup! {
                    "The setter should not "<Emphasis>"return"</Emphasis>" a value."
                },
            )
            .detail(setter.range(), "The setter is here:")
            .note("Returning a value from a setter is ignored."),
        )
    }
}
