use crate::react::is_react_create_element;
use crate::semantic_services::Semantic;
use rome_analyze::context::RuleContext;
use rome_analyze::{declare_rule, Rule, RuleCategory, RuleDiagnostic};
use rome_console::{markup, MarkupBuf};
use rome_js_syntax::{JsCallExpression, JsxElement, JsxSelfClosingElement};
use rome_rowan::{declare_node_union, AstNode, AstNodeList};

declare_rule! {
    /// This rules prevents void elements (AKA self-closing elements) from having children.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// <br>invalid child</br>
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// <img alt="some text" children={"some child"} />
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

const VOID_DOM_ELEMENTS: [&str; 16] = [
    "area", "base", "br", "col", "embed", "hr", "img", "input", "keygen", "link", "menuitem",
    "meta", "param", "source", "track", "wbr",
];

pub(crate) struct NoVoidElementsWithChildrenState {
    /// The name of the element that triggered the rule
    element_name: String,
    /// If the current element has children props
    children_cause: bool,
    /// If the current element has the prop `dangerouslySetInnerHTML`
    dangerous_prop_case: bool,
}

impl NoVoidElementsWithChildrenState {
    fn new(element_name: impl Into<String>) -> Self {
        Self {
            element_name: element_name.into(),
            children_cause: false,
            dangerous_prop_case: false,
        }
    }

    fn with_children_cause(&mut self, cause: bool) {
        self.children_cause = cause;
    }

    fn with_dangerous_prop_cause(&mut self, cause: bool) {
        self.dangerous_prop_case = cause;
    }

    fn message(&self) -> MarkupBuf {
        match (self.children_cause, self.dangerous_prop_case) {
            (true, true) => {
                (markup! {
                    <Emphasis>{self.element_name}</Emphasis>" is a void element tag and must not have the "<Emphasis>"children"</Emphasis>
                    " or the"<Emphasis>"dangerouslySetInnerHTML"</Emphasis>" prop."
                }).to_owned()
            }
            (true, false) => {
                (markup! {
                    <Emphasis>{self.element_name}</Emphasis>" is a void element tag and must not have "<Emphasis>"children"</Emphasis>"."
                }).to_owned()
            }
            (false, true) => {
                (markup! {
                    <Emphasis>{self.element_name}</Emphasis>" is a void element tag and must not have the "<Emphasis>"dangerouslySetInnerHTML"</Emphasis>" prop."
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
                if VOID_DOM_ELEMENTS.contains(&name) {
                    let has_dangerous_prop = opening_element
                        .find_attribute_by_name("dangerouslySetInnerHTML")
                        .ok()?
                        .is_some();
                    let has_children = !element.children().is_empty();
                    let has_children_prop = opening_element
                        .find_attribute_by_name("children")
                        .ok()?
                        .is_some();
                    if has_dangerous_prop || has_children || has_children_prop {
                        let mut state = NoVoidElementsWithChildrenState::new(name);
                        state.with_dangerous_prop_cause(has_dangerous_prop);
                        state.with_children_cause(has_children || has_children_prop);

                        return Some(state);
                    }
                }
            }
            NoVoidElementsWithChildrenQuery::JsxSelfClosingElement(element) => {
                let name = element.name().ok()?;
                let name = name.as_jsx_name()?.value_token().ok()?;
                let name = name.text_trimmed();
                if VOID_DOM_ELEMENTS.contains(&name) {
                    let has_dangerous_prop = element
                        .find_attribute_by_name("dangerouslySetInnerHTML")
                        .ok()?
                        .is_some();
                    let has_children_prop =
                        element.find_attribute_by_name("children").ok()?.is_some();
                    if has_dangerous_prop || has_children_prop {
                        let mut state = NoVoidElementsWithChildrenState::new(name);
                        state.with_dangerous_prop_cause(has_dangerous_prop);
                        state.with_children_cause(has_children_prop);

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
                    if VOID_DOM_ELEMENTS.contains(&element_name) {
                        let has_children = react_create_element.children.is_some();
                        let has_dangerous_prop = react_create_element
                            .find_prop_by_name("dangerouslySetInnerHTML")
                            .is_some();
                        let has_children_prop =
                            react_create_element.find_prop_by_name("children").is_some();

                        if has_dangerous_prop || has_children || has_children_prop {
                            let mut state = NoVoidElementsWithChildrenState::new(element_name);
                            state.with_dangerous_prop_cause(has_dangerous_prop);
                            state.with_children_cause(has_children || has_children_prop);

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
        Some(RuleDiagnostic::new(range, state.message()))
    }
}
