use crate::aria_services::Aria;
use rome_analyze::context::RuleContext;
use rome_analyze::{declare_rule, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_js_syntax::jsx_ext::JsxAnyElement;
use rome_js_syntax::JsxAttribute;
use rome_rowan::AstNode;

declare_rule! {
    /// Enforce that elements with ARIA roles must have all required attributes for that role
    pub(crate) UseAriaPropsForRole {
        version: "11.0.0",
        name: "useAriaPropsForRole",
        recommended: false,
    }
}

#[derive(Default, Debug)]
pub(crate) struct UseAriaPropsForRoleState {
    missing_aria_props: Vec<String>,
    attribute: Option<(JsxAttribute, String)>,
}

impl UseAriaPropsForRoleState {
    pub(crate) fn as_diagnostic(&self) -> Option<RuleDiagnostic> {
        if self.missing_aria_props.is_empty() {
            return None;
        }
        self.attribute.as_ref().map(|(attribute, role_name)| {
            let joined_attributes = &self.missing_aria_props.join(", "); 
            let description = format!("The element with the {role_name} ARIA role does not have the required ARIA attributes: {} ", joined_attributes);
            RuleDiagnostic::new(
                rule_category!(),
                attribute.range(),
                markup! {
                "The element with the "<Emphasis>{role_name}</Emphasis>" ARIA role does not have the required ARIA attributes."
                },
            )
            .description(description)
            .footer_list(markup! { "Missing aria props" }, &self.missing_aria_props)
        })
    }
}

impl Rule for UseAriaPropsForRole {
    type Query = Aria<JsxAnyElement>;
    type State = UseAriaPropsForRoleState;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let roles = ctx.aria_roles();

        let role_attribute = match node {
            JsxAnyElement::JsxSelfClosingElement(element) => {
                element.attributes().find_by_name("role").ok()?
            }
            JsxAnyElement::JsxOpeningElement(element) => {
                element.attributes().find_by_name("role").ok()?
            }
        }?;
        let name = role_attribute
            .initializer()?
            .value()
            .ok()?
            .as_jsx_string()?
            .inner_string_text()
            .ok()?;

        let role = roles.get_role(name.text());
        let mut missing_aria_props = vec![];
        if let Some(role) = role {
            let properties = role.properties();
            for (property_name, required) in properties {
                if *required {
                    let attribute = node.find_attribute_by_name(property_name);
                    if attribute.is_none() {
                        missing_aria_props.push(property_name.to_string());
                    }
                }
            }
            return Some(UseAriaPropsForRoleState {
                attribute: Some((role_attribute, name.text().to_string())),
                missing_aria_props,
            });
        }

        Some(UseAriaPropsForRoleState::default())
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        state.as_diagnostic()
    }
}
