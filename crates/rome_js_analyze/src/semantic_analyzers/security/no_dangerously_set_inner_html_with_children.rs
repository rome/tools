use crate::react::{ReactApiCall, ReactCreateElementCall};
use crate::semantic_services::Semantic;
use rome_analyze::context::RuleContext;
use rome_analyze::{declare_rule, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_js_semantic::SemanticModel;
use rome_js_syntax::{
    JsCallExpression, JsPropertyObjectMember, JsSyntaxNode, JsxAttribute, JsxElement,
    JsxSelfClosingElement,
};
use rome_rowan::{declare_node_union, AstNode, AstNodeList, TextRange};

declare_rule! {
    /// Report when a DOM element or a component uses both `children` and `dangerouslySetInnerHTML` prop.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```jsx,expect_diagnostic
    /// function createMarkup() {
    ///     return { __html: 'child' }
    /// }
    /// <Component dangerouslySetInnerHTML={createMarkup()}>"child1"</Component>
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// function createMarkup() {
    ///     return { __html: 'child' }
    /// }
    /// <Component dangerouslySetInnerHTML={createMarkup()} children="child1" />
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// React.createElement('div', { dangerouslySetInnerHTML: { __html: 'HTML' } }, 'children')
    /// ```
    pub(crate) NoDangerouslySetInnerHtmlWithChildren {
        version: "0.10.0",
        name: "noDangerouslySetInnerHtmlWithChildren",
        recommended: true,
    }
}

declare_node_union! {
    pub(crate) DangerousProp = JsxAttribute | JsPropertyObjectMember
}
/// The kind of children
enum ChildrenKind {
    /// As prop, e.g.
    /// ```jsx
    /// <Component children="child" />
    /// ```
    Prop(DangerousProp),
    /// As direct descendent, e.g.
    /// ```jsx
    /// <ComponentA><ComponentB /> </ComponentA>
    /// ```
    Direct(JsSyntaxNode),
}

impl ChildrenKind {
    fn text_trimmed_range(&self) -> TextRange {
        match self {
            ChildrenKind::Prop(prop) => prop.syntax().text_trimmed_range(),
            ChildrenKind::Direct(node) => node.text_trimmed_range(),
        }
    }
}

pub(crate) struct RuleState {
    /// The `dangerouslySetInnerHTML` prop
    dangerous_prop: DangerousProp,

    /// The kind of `children` found
    children_kind: ChildrenKind,
}

declare_node_union! {
    pub(crate) JsAnyCreateElement = JsxElement | JsxSelfClosingElement | JsCallExpression
}

impl JsAnyCreateElement {
    /// If checks if the element has direct children (no children prop)
    fn has_children(&self, model: &SemanticModel) -> Option<JsSyntaxNode> {
        match self {
            JsAnyCreateElement::JsxElement(element) => {
                if !element.children().is_empty() {
                    Some(element.children().syntax().clone())
                } else {
                    None
                }
            }
            JsAnyCreateElement::JsxSelfClosingElement(_) => None,
            JsAnyCreateElement::JsCallExpression(expression) => {
                let react_create_element =
                    ReactCreateElementCall::from_call_expression(expression, model)?;

                react_create_element
                    .children
                    .map(|children| children.syntax().clone())
            }
        }
    }
    fn find_dangerous_prop(&self, model: &SemanticModel) -> Option<DangerousProp> {
        match self {
            JsAnyCreateElement::JsxElement(element) => {
                let opening_element = element.opening_element().ok()?;

                opening_element
                    .find_attribute_by_name("dangerouslySetInnerHTML")
                    .ok()?
                    .map(DangerousProp::from)
            }
            JsAnyCreateElement::JsxSelfClosingElement(element) => element
                .find_attribute_by_name("dangerouslySetInnerHTML")
                .ok()?
                .map(DangerousProp::from),
            JsAnyCreateElement::JsCallExpression(call_expression) => {
                let react_create_element =
                    ReactCreateElementCall::from_call_expression(call_expression, model)?;

                react_create_element
                    .find_prop_by_name("dangerouslySetInnerHTML")
                    .map(DangerousProp::from)
            }
        }
    }

    fn find_children_prop(&self, model: &SemanticModel) -> Option<DangerousProp> {
        match self {
            JsAnyCreateElement::JsxElement(element) => {
                let opening_element = element.opening_element().ok()?;

                opening_element
                    .find_attribute_by_name("children")
                    .ok()?
                    .map(DangerousProp::from)
            }
            JsAnyCreateElement::JsxSelfClosingElement(element) => element
                .find_attribute_by_name("children")
                .ok()?
                .map(DangerousProp::from),
            JsAnyCreateElement::JsCallExpression(call_expression) => {
                let react_create_element =
                    ReactCreateElementCall::from_call_expression(call_expression, model)?;

                react_create_element
                    .find_prop_by_name("children")
                    .map(DangerousProp::from)
            }
        }
    }
}

impl Rule for NoDangerouslySetInnerHtmlWithChildren {
    type Query = Semantic<JsAnyCreateElement>;
    type State = RuleState;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let model = ctx.model();

        if let Some(dangerous_prop) = node.find_dangerous_prop(model) {
            if let Some(children_node) = node.has_children(model) {
                return Some(RuleState {
                    children_kind: ChildrenKind::Direct(children_node),
                    dangerous_prop,
                });
            } else if let Some(children_prop) = node.find_children_prop(model) {
                return Some(RuleState {
                    children_kind: ChildrenKind::Prop(children_prop),
                    dangerous_prop,
                });
            }
        }

        None
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(RuleDiagnostic::new(
            rule_category!(),
            state.dangerous_prop.syntax().text_trimmed_range(),
            markup! {
                "Avoid passing both "<Emphasis>"children"</Emphasis>" and the "<Emphasis>"dangerouslySetInnerHTML"</Emphasis>" prop."
            },
        ).detail(state.children_kind.text_trimmed_range(), markup! {
            "This is the source of the children prop"
        }).note(
            markup! {
                "Setting HTML content will inadvertently override any passed children in React"
            }
        ))
    }
}
