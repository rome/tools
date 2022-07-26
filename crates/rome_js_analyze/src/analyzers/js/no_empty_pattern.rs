use rome_analyze::{context::RuleContext, declare_rule, Ast, Rule, RuleCategory, RuleDiagnostic};
use rome_console::markup;
use rome_js_syntax::{JsArrayBindingPattern, JsObjectBindingPattern};
use rome_rowan::{declare_node_union, AstNode, AstSeparatedList};

declare_rule! {
    /// Disallows empty destructuring patterns.
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// var {} = foo;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// var {a: {}} = foo;
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// function foo({}) {}
    /// ```
    ///
    /// ### Valid
    /// The following cases are valid because they create new bindings.
    ///
    /// ```js
    /// var {a = {}} = foo;
    /// var {a, b = {}} = foo;
    /// var {a = []} = foo;
    /// function foo({a = {}}) {}
    /// function foo({a = []}) {}
    /// var [a] = foo;
    /// ```
    pub(crate) NoEmptyPattern {
        version: "0.7.0",
        name: "noEmptyPattern",
        recommended: true
    }
}

impl Rule for NoEmptyPattern {
    const CATEGORY: RuleCategory = RuleCategory::Lint;

    type Query = Ast<JsAnyBindPatternLike>;
    type State = ();
    type Signals = Option<Self::State>;

    fn run(ctx: &RuleContext<Self>) -> Option<Self::State> {
        use JsAnyBindPatternLike::*;
        match ctx.query() {
            JsArrayBindingPattern(array) => {
                if array.elements().len() == 0 {
                    Some(())
                } else {
                    None
                }
            }
            JsObjectBindingPattern(object) => {
                if object.properties().len() == 0 {
                    Some(())
                } else {
                    None
                }
            }
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        let node_type = match node {
            JsAnyBindPatternLike::JsArrayBindingPattern(_) => "array",
            JsAnyBindPatternLike::JsObjectBindingPattern(_) => "object",
        };

        Some(RuleDiagnostic::warning(
            node.range(),
            markup! {
                "Unexpected empty "{node_type}" pattern."
            },
        ))
    }
}

declare_node_union! {
    /// enum of `JsObjectBindingPattern` and `JsArrayBindingPattern`
    pub(crate) JsAnyBindPatternLike = JsArrayBindingPattern | JsObjectBindingPattern
}
