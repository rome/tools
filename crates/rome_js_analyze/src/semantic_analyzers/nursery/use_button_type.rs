use crate::react::{is_react_create_element, ReactCreateElementCall};
use crate::semantic_services::Semantic;
use rome_analyze::{context::RuleContext, declare_rule, Rule, RuleCategory, RuleDiagnostic};
use rome_console::codespan::Severity;
use rome_console::markup;
use rome_js_syntax::{
    JsCallExpression, JsObjectExpression, JsStringLiteralExpression, JsxAnyElementName,
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
        recommended: false,
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
    const CATEGORY: RuleCategory = RuleCategory::Lint;

    type Query = Semantic<UseButtonTypeQuery>;
    type State = UseButtonTypeState;
    type Signals = Option<Self::State>;

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
                        for attribute in attributes {
                            let attribute = attribute.as_jsx_attribute()?;
                            let jsx_name = attribute.name().ok()?;
                            let jsx_name = jsx_name.as_jsx_name()?;
                            let name = jsx_name.value_token().ok()?;
                            let name = name.text_trimmed();
                            if name == "type" {
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
                }
                None
            }
            UseButtonTypeQuery::JsCallExpression(call_expression) => {
                let model = ctx.model();
                if let Some(react_create_element) = is_react_create_element(call_expression, model)
                {
                    let ReactCreateElementCall {
                        element_type,
                        props,
                        ..
                    } = react_create_element;

                    // first argument needs to be a string
                    let first_argument = element_type
                        .as_js_any_expression()?
                        .as_js_any_literal_expression()?
                        .as_js_string_literal_expression()?;

                    // case sensitive is important, <button> is different from <Button>
                    if first_argument.inner_string_text().ok()? == "button" {
                        return if let Some(props) = props {
                            let members = props.members();

                            for member in members {
                                let member = member.ok()?;
                                let property = member.as_js_property_object_member()?;
                                let property_name = property.name().ok()?;
                                let property_value = property.value().ok()?;

                                let property_name = property_name.as_js_literal_member_name()?;
                                // we found the correct member, we can bail
                                if property_name.name().ok()? == "type" {
                                    let value = property_value
                                        .as_js_any_literal_expression()?
                                        .as_js_string_literal_expression()?;

                                    if !ALLOWED_BUTTON_TYPES
                                        .contains(&&*value.inner_string_text().ok()?)
                                    {
                                        return Some(UseButtonTypeState {
                                            node: UseButtonTypeNode::from(value.clone()),
                                            missing_prop: false,
                                        });
                                    }
                                }
                            }

                            // if we are here, it means that we haven't found the property "type" and
                            // we have to return a diagnostic
                            Some(UseButtonTypeState {
                                node: UseButtonTypeNode::from(props),
                                missing_prop: false,
                            })
                        } else {
                            Some(UseButtonTypeState {
                                node: UseButtonTypeNode::from(first_argument.clone()),
                                missing_prop: true,
                            })
                        };
                    }
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
                "Provide a correct "<Emphasis>"type"</Emphasis>" prop for the "<Emphasis>"button"</Emphasis>" element."
            }).to_owned()
        };
        Some(RuleDiagnostic::new(
            state.node.syntax().text_trimmed_range(),
            message
        )
            .footer(Severity::Note, markup! {
                "The default  "<Emphasis>"type"</Emphasis>" of a button is "<Emphasis>"submit"</Emphasis>", which causes the submission of a form when placed inside a `form` element. "
                "This is likely not the behaviour that you want inside a React application."
            })
            .footer_help(
            markup! {

                "Allowed button types are: "<Emphasis>"submit"</Emphasis>", "<Emphasis>"button"</Emphasis>" or "<Emphasis>"reset"</Emphasis>""
            }
        ))
    }
}

/// Checks whether the current element is a button
///
/// Case sensitive is important, `<button>` is different from `<Button>`
fn is_button(name: &JsxAnyElementName) -> Option<bool> {
    Some(match name {
        JsxAnyElementName::JsxName(name) => {
            let name = name.value_token().ok()?;
            name.text_trimmed() == "button"
        }
        _ => false,
    })
}
