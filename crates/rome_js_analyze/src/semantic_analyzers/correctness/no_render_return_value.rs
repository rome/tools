use crate::semantic_services::Semantic;
use rome_analyze::context::RuleContext;
use rome_analyze::{declare_rule, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_js_semantic::SemanticModel;
use rome_js_syntax::JsSyntaxKind::JS_IMPORT;
use rome_js_syntax::{
    JsAnyExpression, JsCallExpression, JsExpressionStatement, JsIdentifierBinding,
    JsIdentifierExpression, JsStaticMemberExpression,
};
use rome_rowan::{declare_node_union, AstNode};

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
        let is_react_render = match callee {
            JsAnyExpression::JsStaticMemberExpression(static_member) => {
                is_react_render(PossibleReactRender::from(static_member), model)
            }
            JsAnyExpression::JsIdentifierExpression(identifier_expression) => {
                is_react_render(PossibleReactRender::from(identifier_expression), model)
            }
            _ => return None,
        }?;
        if is_react_render {
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

declare_node_union! {
    pub(crate) PossibleReactRender = JsStaticMemberExpression | JsIdentifierExpression
}

fn is_react_render(node: PossibleReactRender, model: &SemanticModel) -> Option<bool> {
    let result = match node {
        PossibleReactRender::JsStaticMemberExpression(node) => {
            let object = node.object().ok()?;
            let member = node.member().ok()?;
            let member = member.as_js_name()?;
            let identifier = object.as_js_identifier_expression()?.name().ok()?;

            let maybe_from_react = identifier.syntax().text_trimmed() == "ReactDOM"
                && member.syntax().text_trimmed() == "render";

            if maybe_from_react {
                let identifier_binding = model.declaration(&identifier);
                if let Some(binding_identifier) = identifier_binding {
                    let binding_identifier =
                        JsIdentifierBinding::cast_ref(binding_identifier.syntax())?;
                    for ancestor in binding_identifier.syntax().ancestors() {
                        if ancestor.kind() == JS_IMPORT {
                            return Some(
                                binding_identifier.syntax().text_trimmed()
                                    == identifier.syntax().text_trimmed(),
                            );
                        }
                    }
                }
            }
            maybe_from_react
        }
        PossibleReactRender::JsIdentifierExpression(identifier) => {
            let maybe_react_render = identifier.syntax().text_trimmed() == "render";
            let name = identifier.name().ok()?;
            if maybe_react_render {
                let declaration = model.declaration(&name);
                if let Some(declaration) = declaration {
                    let identifier_binding = JsIdentifierBinding::cast_ref(declaration.syntax())?;
                    for ancestor in identifier_binding.syntax().ancestors() {
                        if ancestor.kind() == JS_IMPORT {
                            return Some(
                                identifier_binding.syntax().text_trimmed()
                                    == identifier.syntax().text_trimmed(),
                            );
                        }
                    }
                }
            }

            maybe_react_render
        }
    };

    Some(result)
}
