use crate::react::{is_react_create_element, ReactCreateElementCall};
use crate::semantic_services::Semantic;
use crate::JsRuleAction;
use rome_analyze::context::RuleContext;
use rome_analyze::{declare_rule, ActionCategory, Rule, RuleCategory, RuleDiagnostic};
use rome_console::{markup, MarkupBuf};
use rome_diagnostics::Applicability;
use rome_js_factory::make::{jsx_attribute_list, jsx_self_closing_element};
use rome_js_syntax::{
    JsCallExpression, JsPropertyObjectMember, JsxAnyAttribute, JsxAttribute, JsxElement,
    JsxSelfClosingElement,
};
use rome_rowan::{declare_node_union, AstNode, AstNodeList, BatchMutationExt};

declare_rule! {
    /// This rules prevents void elements (AKA self-closing elements) from having children.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```jsx,expect_diagnostic
    /// <br>invalid child</br>
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <img alt="some text" children={"some child"} />
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// React.createElement('img', {}, 'child')
    /// ```
    pub(crate) NoVoidElementsWithChildren {
        version: "0.10.0",
        name: "noVoidElementsWithChildren",
        recommended: false,
    }
}

declare_node_union! {
    pub(crate) NoVoidElementsWithChildrenQuery = JsxElement | JsCallExpression | JsxSelfClosingElement
}

/// Returns true if the name of the element belong to a self-closing element
fn is_void_dom_element(element_name: &str) -> bool {
    matches!(
        element_name,
        "area"
            | "base"
            | "br"
            | "col"
            | "embed"
            | "hr"
            | "img"
            | "input"
            | "keygen"
            | "link"
            | "menuitem"
            | "meta"
            | "param"
            | "source"
            | "track"
            | "wbr"
    )
}

pub(crate) struct NoVoidElementsWithChildrenState {
    /// The name of the element that triggered the rule
    element_name: String,
    /// If the current element has children props in style `<Component>'children'</Component>
    children_cause: bool,
    /// If the current element has the prop `dangerouslySetInnerHTML`
    dangerous_prop_case: Option<UnwelcomedProp>,
    /// If the current element has the prop `children`
    children_prop: Option<UnwelcomedProp>,
    /// An instance of [ReactCreateElementCall]
    create_react_element: Option<ReactCreateElementCall>,
}

declare_node_union! {
    pub(crate) UnwelcomedProp = JsxAttribute | JsPropertyObjectMember
}

impl UnwelcomedProp {
    fn as_jsx_attribute(&self) -> Option<&JsxAttribute> {
        match self {
            UnwelcomedProp::JsxAttribute(attribute) => Some(attribute),
            UnwelcomedProp::JsPropertyObjectMember(_) => None,
        }
    }
}

impl NoVoidElementsWithChildrenState {
    fn new(element_name: impl Into<String>) -> Self {
        Self {
            element_name: element_name.into(),
            children_cause: false,
            dangerous_prop_case: None,
            children_prop: None,
            create_react_element: None,
        }
    }

    fn with_children_cause(&mut self, cause: bool) {
        self.children_cause = cause;
    }

    fn with_dangerous_prop_cause(&mut self, attribute: UnwelcomedProp) {
        self.dangerous_prop_case = Some(attribute);
    }

    fn with_children_prop_cause(&mut self, attribute: UnwelcomedProp) {
        self.children_prop = Some(attribute);
    }

    fn with_create_react_element(&mut self, react_create_element: ReactCreateElementCall) {
        self.create_react_element = Some(react_create_element);
    }

    fn diagnostic_message(&self) -> MarkupBuf {
        let children_cause = self.children_cause || self.children_prop.is_some();
        match (children_cause, self.dangerous_prop_case.as_ref()) {
            (true, Some(_)) => {
                (markup! {
                    <Emphasis>{self.element_name}</Emphasis>" is a void element tag and must not have "<Emphasis>"children"</Emphasis>
                    ", or the "<Emphasis>"dangerouslySetInnerHTML"</Emphasis>" prop."
                }).to_owned()
            }
            (true, None) => {
                (markup! {
                    <Emphasis>{self.element_name}</Emphasis>" is a void element tag and must not have "<Emphasis>"children"</Emphasis>"."
                }).to_owned()
            }
            (false, Some(_)) => {
                (markup! {
                    <Emphasis>{self.element_name}</Emphasis>" is a void element tag and must not have the "<Emphasis>"dangerouslySetInnerHTML"</Emphasis>" prop."
                }).to_owned()
            },
            _ => unreachable!("At least a cause must be set")

        }
    }

    fn action_message(&self) -> MarkupBuf {
        let children_cause = self.children_cause || self.children_prop.is_some();
        match (children_cause, self.dangerous_prop_case.as_ref()) {
            (true, Some(_)) => {
                (markup! {
                    "Remove the "<Emphasis>"children"</Emphasis>" and the "<Emphasis>"dangerouslySetInnerHTML"</Emphasis>" prop."
                }).to_owned()
            }
            (true, None) => {
                (markup! {
                   "Remove the "<Emphasis>"children"</Emphasis>"."
                }).to_owned()
            }
            (false, Some(_)) => {
                (markup! {
                  "Remove the "<Emphasis>"dangerouslySetInnerHTML"</Emphasis>" prop."
                }).to_owned()
            },
            _ => unreachable!("At least a cause must be set")

        }
    }
}

impl Rule for NoVoidElementsWithChildren {
    const CATEGORY: RuleCategory = RuleCategory::Lint;
    type Query = Semantic<NoVoidElementsWithChildrenQuery>;
    type State = NoVoidElementsWithChildrenState;
    type Signals = Option<Self::State>;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let model = ctx.model();

        match node {
            NoVoidElementsWithChildrenQuery::JsxElement(element) => {
                let opening_element = element.opening_element().ok()?;
                let name = opening_element.name().ok()?;
                let name = name.as_jsx_name()?.value_token().ok()?;
                let name = name.text_trimmed();
                if is_void_dom_element(name) {
                    let dangerous_prop = opening_element
                        .find_attribute_by_name("dangerouslySetInnerHTML")
                        .ok()?;
                    let has_children = !element.children().is_empty();
                    let children_prop = opening_element.find_attribute_by_name("children").ok()?;
                    if dangerous_prop.is_some() || has_children || children_prop.is_some() {
                        let mut state = NoVoidElementsWithChildrenState::new(name);
                        if let Some(dangerous_prop) = dangerous_prop {
                            state.with_dangerous_prop_cause(UnwelcomedProp::from(dangerous_prop));
                        }
                        if let Some(children_prop) = children_prop {
                            state.with_children_prop_cause(UnwelcomedProp::from(children_prop));
                        }
                        state.with_children_cause(has_children);

                        return Some(state);
                    }
                }
            }
            NoVoidElementsWithChildrenQuery::JsxSelfClosingElement(element) => {
                let name = element.name().ok()?;
                let name = name.as_jsx_name()?.value_token().ok()?;
                let name = name.text_trimmed();
                if is_void_dom_element(name) {
                    let dangerous_prop = element
                        .find_attribute_by_name("dangerouslySetInnerHTML")
                        .ok()?;
                    let children_prop = element.find_attribute_by_name("children").ok()?;
                    if dangerous_prop.is_some() || children_prop.is_some() {
                        let mut state = NoVoidElementsWithChildrenState::new(name);
                        if let Some(dangerous_prop) = dangerous_prop {
                            state.with_dangerous_prop_cause(UnwelcomedProp::from(dangerous_prop));
                        }
                        if let Some(children_prop) = children_prop {
                            state.with_children_prop_cause(UnwelcomedProp::from(children_prop));
                        }

                        return Some(state);
                    }
                }
            }
            NoVoidElementsWithChildrenQuery::JsCallExpression(call_expression) => {
                if let Some(react_create_element) = is_react_create_element(call_expression, model)
                {
                    let element_type = react_create_element
                        .element_type
                        .as_js_any_expression()?
                        .as_js_any_literal_expression()?
                        .as_js_string_literal_expression()?;

                    let element_name = element_type.inner_string_text().ok()?;
                    let element_name = element_name.text();
                    if is_void_dom_element(element_name) {
                        let has_children = react_create_element.children.is_some();
                        let dangerous_prop =
                            react_create_element.find_prop_by_name("dangerouslySetInnerHTML");
                        let children_prop = react_create_element.find_prop_by_name("children");

                        if dangerous_prop.is_some() || has_children || children_prop.is_some() {
                            let mut state = NoVoidElementsWithChildrenState::new(element_name);
                            if let Some(dangerous_prop) = dangerous_prop {
                                state.with_dangerous_prop_cause(UnwelcomedProp::from(
                                    dangerous_prop,
                                ));
                            }
                            if let Some(children_prop) = children_prop {
                                state.with_children_prop_cause(UnwelcomedProp::from(children_prop));
                            }
                            state.with_children_cause(has_children);
                            state.with_create_react_element(react_create_element);

                            return Some(state);
                        }
                    }
                }
            }
        }

        None
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        let range = match node {
            NoVoidElementsWithChildrenQuery::JsxElement(element) => {
                element.syntax().text_trimmed_range()
            }
            NoVoidElementsWithChildrenQuery::JsCallExpression(expression) => {
                expression.syntax().text_trimmed_range()
            }
            NoVoidElementsWithChildrenQuery::JsxSelfClosingElement(element) => {
                element.syntax().text_trimmed_range()
            }
        };
        Some(RuleDiagnostic::new(
            rule_category!(),
            range,
            state.message(),
        ))
        Some(RuleDiagnostic::new(range, state.diagnostic_message()))
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let node = ctx.query();
        let mut mutation = ctx.root().begin();

        match node {
            NoVoidElementsWithChildrenQuery::JsxElement(element) => {
                let opening_element = element.opening_element().ok()?;
                let closing_element = element.closing_element().ok()?;

                let mut new_attribute_list = Vec::new();
                // here we create a new list of attributes, ignoring the ones that needs to be
                // removed
                for attribute in opening_element.attributes() {
                    let children_prop = state
                        .children_prop
                        .as_ref()
                        .and_then(|prop| prop.as_jsx_attribute());

                    let dangerous_prop = state
                        .dangerous_prop_case
                        .as_ref()
                        .and_then(|prop| prop.as_jsx_attribute());

                    if let JsxAnyAttribute::JsxAttribute(attribute) = &attribute {
                        if let Some(children_prop) = children_prop {
                            if children_prop == attribute {
                                continue;
                            }
                        }

                        if let Some(dangerous_prop) = dangerous_prop {
                            if dangerous_prop == attribute {
                                continue;
                            }
                        }
                    }
                    new_attribute_list.push(attribute)
                }

                let new_attribute_list = jsx_attribute_list(new_attribute_list);

                let new_node = jsx_self_closing_element(
                    opening_element.l_angle_token().ok()?,
                    opening_element.name().ok()?,
                    new_attribute_list,
                    closing_element.slash_token().ok()?,
                    opening_element.r_angle_token().ok()?,
                )
                .build();
                mutation.replace_element(
                    element.clone().into_syntax().into(),
                    new_node.into_syntax().into(),
                );
            }
            NoVoidElementsWithChildrenQuery::JsCallExpression(_) => {
                // SAFETY: safe because create_react_element is always set in case of error around a call expression
                let create_react_element = state.create_react_element.as_ref().unwrap();
                if state.children_cause {
                    if let Some(children) = create_react_element.children.as_ref() {
                        mutation.remove_node(children.clone());
                    }
                }
                if let Some(children_prop) = state.children_prop.as_ref() {
                    mutation.remove_node(children_prop.clone());
                }
                if let Some(dangerous_prop_case) = state.dangerous_prop_case.as_ref() {
                    mutation.remove_node(dangerous_prop_case.clone());
                }
            }
            // self closing elements don't have inner children so we can safely just remove the props
            // that we don't need
            NoVoidElementsWithChildrenQuery::JsxSelfClosingElement(_) => {
                if let Some(children_prop) = state.children_prop.as_ref() {
                    mutation.remove_node(children_prop.clone());
                }
                if let Some(dangerous_prop_case) = state.dangerous_prop_case.as_ref() {
                    mutation.remove_node(dangerous_prop_case.clone());
                }
            }
        }

        Some(JsRuleAction {
            mutation,
            message: state.action_message(),
            category: ActionCategory::QuickFix,
            applicability: Applicability::MaybeIncorrect,
        })
    }
}
