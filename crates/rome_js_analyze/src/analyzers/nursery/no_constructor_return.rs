use rome_analyze::context::RuleContext;
use rome_analyze::{declare_rule, Ast, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_js_syntax::{JsConstructorClassMember, JsReturnStatement};
use rome_rowan::AstNode;

use crate::control_flow::AnyJsControlFlowRoot;

declare_rule! {
    /// Disallow returning a value from a constructor
    ///
    /// While returning a value from a constructor does not produce an error, the returned value is being ignored. Therefore, returning a value from a constructor is either unnecessary or a possible error.
    ///
    /// Only returning without a value is allowed, as it’s a control flow statement.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// class A {
    ///     constructor() {
    ///         return 0;
    ///     }
    /// }
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js
    /// class A {
    ///     constructor() {}
    /// }
    /// ```
    ///
    /// ```js
    /// class B {
    ///     constructor(x) {
    ///         return;
    ///     }
    /// }
    /// ```
    ///
    /// ```
    pub(crate) NoConstructorReturn {
        version: "11.0.0",
        name: "noConstructorReturn",
        recommended: true,
    }
}

impl Rule for NoConstructorReturn {
    type Query = Ast<JsReturnStatement>;
    type State = JsConstructorClassMember;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let ret = ctx.query();
        // Do not take arg-less returns into account
        let _arg = ret.argument()?;
        let constructor = ret
            .syntax()
            .ancestors()
            .find(|x| AnyJsControlFlowRoot::can_cast(x.kind()))
            .and_then(JsConstructorClassMember::cast);
        constructor
    }

    fn diagnostic(ctx: &RuleContext<Self>, constructor: &Self::State) -> Option<RuleDiagnostic> {
        let ret = ctx.query();
        Some(RuleDiagnostic::new(
            rule_category!(),
            ret.range(),
            markup! {
                "The "<Emphasis>"constructor"</Emphasis>" should not "<Emphasis>"return"</Emphasis>" a value."
            },
        ).detail(
            constructor.range(),
            "The constructor is here:"
        ).note("Returning a value from a constructor is ignored."))
    }
}
