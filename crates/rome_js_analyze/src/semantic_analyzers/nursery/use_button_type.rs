use crate::semantic_services::Semantic;
use crate::shared::react::{is_react_create_element, PossibleCreateElement};
use rome_analyze::{context::RuleContext, declare_rule, Rule, RuleCategory, RuleDiagnostic};
use rome_console::codespan::Severity;
use rome_console::markup;
use rome_js_syntax::{
    JsAnyExpression, JsCallExpression, JsStringLiteralExpression, JsxOpeningElement, JsxString,
};
use rome_rowan::{declare_node_union, AstNode, AstNodeList};

declare_rule! {
    /// Enforces the correct usage of the attribute `type` for the element `button`
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

pub(crate) enum UseButtonTypeState {
    Initializer(JsxString),
    Element(JsxOpeningElement),
    Literal(JsStringLiteralExpression),
    Property(JsStringLiteralExpression),
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
                if name.syntax().text_trimmed() == "button" {
                    let attributes = opening_element.attributes();
                    if attributes.len() == 0 {
                        return Some(UseButtonTypeState::Element(opening_element.clone()));
                    } else {
                        for attribute in attributes {
                            let attribute = attribute.as_jsx_attribute()?;
                            let jsx_name = attribute.name().ok()?;
                            let jsx_name = jsx_name.as_jsx_name()?;
                            let name = jsx_name.syntax().text_trimmed();
                            if name == "type" {
                                let initializer = attribute.initializer()?.value().ok()?;
                                let initializer = initializer.as_jsx_string()?;
                                if !ALLOWED_BUTTON_TYPES
                                    .contains(&&*initializer.inner_string_text().ok()?)
                                {
                                    return Some(UseButtonTypeState::Initializer(
                                        initializer.clone(),
                                    ));
                                }
                            }
                        }
                    }
                }
                None
            }
            UseButtonTypeQuery::JsCallExpression(call_expression) => {
                let model = ctx.model();
                let callee = call_expression.callee().ok()?;
                let is_create_element = match callee {
                    JsAnyExpression::JsStaticMemberExpression(static_member) => {
                        is_react_create_element(PossibleCreateElement::from(static_member), model)
                    }
                    JsAnyExpression::JsIdentifierExpression(identifier_expression) => {
                        is_react_create_element(
                            PossibleCreateElement::from(identifier_expression),
                            model,
                        )
                    }
                    _ => return None,
                }?;

                if is_create_element {
                    let arguments = call_expression.arguments().ok()?.args();
                    let mut arguments = arguments.into_iter();
                    if let Some(first_argument) = arguments.next() {
                        // first argument needs to be a string
                        let first_argument = first_argument.ok()?;
                        let first_argument = first_argument
                            .as_js_any_expression()?
                            .as_js_any_literal_expression()?
                            .as_js_string_literal_expression()?;

                        if first_argument.inner_string_text().ok()? == "button" {
                            let second_argument = arguments.next();

                            if let Some(second_argument) = second_argument {
                                let second_argument = second_argument.ok()?;
                                let members = second_argument
                                    .as_js_any_expression()?
                                    .as_js_object_expression()?
                                    .members();

                                for member in members {
                                    let member = member.ok()?;
                                    let property = member.as_js_property_object_member()?;
                                    let property_name = property.name().ok()?;
                                    let property_value = property.value().ok()?;

                                    let property_name =
                                        property_name.as_js_literal_member_name()?;
                                    // we found the correct member, we can bail
                                    if property_name.name().ok()? == "type" {
                                        let value = property_value
                                            .as_js_any_literal_expression()?
                                            .as_js_string_literal_expression()?;

                                        if !ALLOWED_BUTTON_TYPES
                                            .contains(&&*value.inner_string_text().ok()?)
                                        {
                                            return Some(UseButtonTypeState::Property(
                                                value.clone(),
                                            ));
                                        }
                                    }
                                }

                                // if we are here, it means that we haven't found the property "type"
                            } else {
                                return Some(UseButtonTypeState::Literal(first_argument.clone()));
                            }
                        }
                    }
                }

                None
            }
        }
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let range = match state {
            UseButtonTypeState::Initializer(string) => string.syntax().text_trimmed_range(),
            UseButtonTypeState::Element(element) => element.syntax().text_trimmed_range(),
            UseButtonTypeState::Property(property) => property.syntax().text_trimmed_range(),
            UseButtonTypeState::Literal(literal) => literal.syntax().text_trimmed_range(),
        };

        Some(RuleDiagnostic::new(
            range,
            markup! {
                "Provide an explicit "<Emphasis>"type"</Emphasis>" prop on "<Emphasis>"button"</Emphasis>" elements."
            },
        )
            .footer(Severity::Note, markup! {
                "The default type of a button is "<Emphasis>"submit"</Emphasis>", which causes a page reload and is not a typical behavior in a React application."
            })
            .footer_help(
            markup! {

                "Allowed button types are: "<Emphasis>"submit"</Emphasis>", "<Emphasis>"button"</Emphasis>" or "<Emphasis>"reset"</Emphasis>""
            }
        ))
    }
}
