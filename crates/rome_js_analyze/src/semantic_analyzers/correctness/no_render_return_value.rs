use crate::react::{is_react_call_api, ReactLibrary};
use crate::semantic_services::Semantic;
use rome_analyze::context::RuleContext;
use rome_analyze::{declare_rule, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_js_syntax::{JsCallExpression, JsExpressionStatement};
use rome_rowan::AstNode;

declare_rule! {
    /// Prevent the usage of the return value of `React.render`.
    ///
    /// > `ReactDOM.render()` currently returns a reference to the root `ReactComponent` instance. However, using this return value is legacy
    /// and should be avoided because future versions of React may render components asynchronously in some cases.
    /// If you need a reference to the root `ReactComponent` instance, the preferred solution is to attach a [callback ref](https://reactjs.org/docs/refs-and-the-dom.html#callback-refs)
    /// to the root element.
    ///
    /// Source: [ReactDOM documentation](https://facebook.github.io/react/docs/react-dom.html#render)
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```jsx,expect_diagnostic
    /// const foo = ReactDOM.render(<div />, document.body);
    /// ```
    ///
    /// ### Valid
    ///
    /// ```jsx
    /// ReactDOM.render(<div />, document.body);
    /// ```
    pub(crate) NoRenderReturnValue {
        version: "0.10.0",
        name: "noRenderReturnValue",
        recommended: true,
    }
}

impl Rule for NoRenderReturnValue {
    type Query = Semantic<JsCallExpression>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let callee = node.callee().ok()?;
        let model = ctx.model();
        if is_react_call_api(&callee, model, ReactLibrary::ReactDOM, "render") {
            let parent = node.syntax().parent()?;

            if !JsExpressionStatement::can_cast(parent.kind()) {
                return Some(());
            }
        }
        None
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        Some(RuleDiagnostic::new(rule_category!(),
            node.syntax().text_trimmed_range(),
            markup! {
                "Do not depend on the value returned by the function "<Emphasis>"ReactDOM.render()"</Emphasis>"."
            },
        ).note(markup! {
"The returned value is legacy and future versions of react might return that value asynchronously."
"
Check the "<Hyperlink href="https://facebook.github.io/react/docs/react-dom.html#render">"React documentation"</Hyperlink>" for more information."

        })
        )
    }
}
