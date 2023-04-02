use crate::react::{ReactApiCall, ReactCreateElementCall};
use crate::semantic_services::Semantic;
use rome_analyze::{context::RuleContext, declare_rule, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_js_syntax::{
    AnyJsxElementName, JsCallExpression, JsObjectExpression, JsStringLiteralExpression,
    JsxOpeningElement, JsxString,
};
use rome_rowan::{declare_node_union, AstNode, AstNodeList};

declare_rule! {
    /// Enforces the usage of the attribute `type` for the element `button`
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```jsx,expect_diagnostic
    /// <button>Do something</button>
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <button type="incorrectType">Do something</button>
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// React.createElement('button');
    /// ```
    ///
    /// ## Valid
    ///
    /// ```jsx
    /// <>
    ///     <button type="button">Do something</button>
    ///     <button type={buttonType}>Do something</button>
    /// </>
    /// ```
    pub(crate) UseButtonType {
        version: "0.10.0",
        name: "useButtonType",
        recommended: true,
    }
}

const ALLOWED_BUTTON_TYPES: [&str; 3] = ["submit", "button", "reset"];

declare_node_union! {
    pub(crate) UseButtonTypeQuery = JsxOpeningElement | JsCallExpression
}

declare_node_union! {
    pub(crate) UseButtonTypeNode = JsxString | JsxOpeningElement | JsStringLiteralExpression | JsObjectExpression
}

pub(crate) struct UseButtonTypeState {
    node: UseButtonTypeNode,
    missing_prop: bool,
}

impl Rule for UseButtonType {
    type Query = Semantic<UseButtonTypeQuery>;
    type State = UseButtonTypeState;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();

        match node {
            UseButtonTypeQuery::JsxOpeningElement(opening_element) => {
                let name = opening_element.name().ok()?;
                // we bail early the current tag is not a button; case sensitive is important
                if is_button(&name)? {
                    let attributes = opening_element.attributes();
                    if attributes.is_empty() {
                        return Some(UseButtonTypeState {
                            node: UseButtonTypeNode::from(opening_element.clone()),
                            missing_prop: true,
                        });
                    } else {
                        let type_attribute = opening_element.find_attribute_by_name("type").ok()?;

                        if let Some(attribute) = type_attribute {
                            let initializer = attribute.initializer()?.value().ok()?;
                            let initializer = initializer.as_jsx_string()?;
                            if !ALLOWED_BUTTON_TYPES
                                .contains(&&*initializer.inner_string_text().ok()?)
                            {
                                return Some(UseButtonTypeState {
                                    node: UseButtonTypeNode::from(initializer.clone()),
                                    missing_prop: false,
                                });
                            }
                        }
                    }
                }
                None
            }
            UseButtonTypeQuery::JsCallExpression(call_expression) => {
                let model = ctx.model();
                let react_create_element =
                    ReactCreateElementCall::from_call_expression(call_expression, model)?;

                // first argument needs to be a string
                let first_argument = react_create_element
                    .element_type
                    .as_any_js_expression()?
                    .as_any_js_literal_expression()?
                    .as_js_string_literal_expression()?;

                // case sensitive is important, <button> is different from <Button>
                if first_argument.inner_string_text().ok()?.text() == "button" {
                    return if let Some(props) = react_create_element.props.as_ref() {
                        let type_member = react_create_element.find_prop_by_name("type");
                        if let Some(member) = type_member {
                            let property_value = member.value().ok()?;
                            let value = property_value
                                .as_any_js_literal_expression()?
                                .as_js_string_literal_expression()?;

                            if !ALLOWED_BUTTON_TYPES.contains(&&*value.inner_string_text().ok()?) {
                                return Some(UseButtonTypeState {
                                    node: UseButtonTypeNode::from(value.clone()),
                                    missing_prop: false,
                                });
                            }
                        }

                        // if we are here, it means that we haven't found the property "type" and
                        // we have to return a diagnostic
                        Some(UseButtonTypeState {
                            node: UseButtonTypeNode::from(props.clone()),
                            missing_prop: false,
                        })
                    } else {
                        Some(UseButtonTypeState {
                            node: UseButtonTypeNode::from(first_argument.clone()),
                            missing_prop: true,
                        })
                    };
                }

                None
            }
        }
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let message = if state.missing_prop {
            (markup! {
                "Provide an explicit "<Emphasis>"type"</Emphasis>" prop for the "<Emphasis>"button"</Emphasis>" element."
            }).to_owned()
        } else {
            (markup!{
                "Provide a valid "<Emphasis>"type"</Emphasis>" prop for the "<Emphasis>"button"</Emphasis>" element."
            }).to_owned()
        };
        Some(RuleDiagnostic::new(rule_category!(),
            state.node.syntax().text_trimmed_range(),
            message
        )
            .note(markup! {
                "The default  "<Emphasis>"type"</Emphasis>" of a button is "<Emphasis>"submit"</Emphasis>", which causes the submission of a form when placed inside a `form` element. "
                "This is likely not the behaviour that you want inside a React application."
            })
            .note(
            markup! {

                "Allowed button types are: "<Emphasis>"submit"</Emphasis>", "<Emphasis>"button"</Emphasis>" or "<Emphasis>"reset"</Emphasis>""
            }
        ))
    }
}

/// Checks whether the current element is a button
///
/// Case sensitive is important, `<button>` is different from `<Button>`
fn is_button(name: &AnyJsxElementName) -> Option<bool> {
    Some(match name {
        AnyJsxElementName::JsxName(name) => {
            let name = name.value_token().ok()?;
            name.text_trimmed() == "button"
        }
        _ => false,
    })
}
