use crate::{aria_services::Aria, JsRuleAction};
use rome_analyze::{context::RuleContext, declare_rule, ActionCategory, Rule, RuleDiagnostic};
use rome_aria::{roles::AriaRoleDefinition, AriaRoles};
use rome_console::markup;
use rome_diagnostics::Applicability;
use rome_js_syntax::{
    jsx_ext::AnyJsxElement, AnyJsLiteralExpression, AnyJsxAttributeValue, JsxAttribute,
    JsxAttributeList,
};
use rome_rowan::{AstNode, BatchMutationExt};

declare_rule! {
    /// Enforce explicit `role` property is not the same as implicit/default role property on an element.
    ///
    /// ESLint (eslint-plugin-jsx-a11y) Equivalent: [no-redundant-roles](https://github.com/evcohen/eslint-plugin-jsx-a11y/blob/main/docs/rules/no-redundant-roles.md)
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```jsx,expect_diagnostic
    /// <article role='article'></article>
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <button role='button'></button>
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <h1 role='heading' aria-level='1'>title</h1>
    /// ```
    ///
    /// ## Valid
    ///
    /// ```jsx
    /// <article role='presentation'></article>
    /// ```
    ///
    /// ```jsx
    /// <Button role='button'></Button>
    /// ```
    ///
    /// ```jsx
    /// <span></span>
    /// ```
    ///
    pub(crate) NoRedundantRoles {
        version: "next",
        name: "noRedundantRoles",
        recommended: true,
    }
}

pub(crate) struct RuleState {
    redundant_attribute: JsxAttribute,
    redundant_attribute_value: AnyJsxAttributeValue,
    element_name: String,
}

impl Rule for NoRedundantRoles {
    type Query = Aria<AnyJsxElement>;
    type State = RuleState;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let aria_roles = ctx.aria_roles();

        let (element_name, attributes) = get_element_name_and_attributes(node)?;
        let attribute_name_to_values = ctx.extract_attributes(&attributes)?;
        let implicit_role =
            aria_roles.get_implicit_role(&element_name, &attribute_name_to_values)?;

        let role_attribute = node.find_attribute_by_name("role")?;
        let role_attribute_value = role_attribute.initializer()?.value().ok()?;
        let explicit_role = get_explicit_role(aria_roles, &role_attribute_value)?;

        let is_redundant = implicit_role.type_name() == explicit_role.type_name();
        if is_redundant {
            return Some(RuleState {
                redundant_attribute: role_attribute,
                redundant_attribute_value: role_attribute_value,
                element_name: element_name.to_string(),
            });
        }
        None
    }

    fn diagnostic(_: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let binding = state.redundant_attribute_value.inner_text_value().ok()??;
        let role_attribute = binding.text();
        let element = state.element_name.to_string();
        Some(RuleDiagnostic::new(
            rule_category!(),
            state.redundant_attribute_value.range(),
            markup! {
                "Using the role attribute '"{role_attribute}"' on the '"{element}"' element is redundant."
            },
        ))
    }

    fn action(ctx: &RuleContext<Self>, state: &Self::State) -> Option<JsRuleAction> {
        let mut mutation = ctx.root().begin();
        mutation.remove_node(state.redundant_attribute.clone());
        Some(JsRuleAction {
            category: ActionCategory::QuickFix,
            applicability: Applicability::MaybeIncorrect,
            message: markup! { "Remove the "<Emphasis>"role"</Emphasis>" attribute." }.to_owned(),
            mutation,
        })
    }
}

fn get_element_name_and_attributes(node: &AnyJsxElement) -> Option<(String, JsxAttributeList)> {
    match node {
        AnyJsxElement::JsxOpeningElement(elem) => {
            let token = elem.name().ok()?;
            let element_name = token.as_jsx_name()?.value_token().ok()?;
            let trimmed_element_name = element_name.text_trimmed().to_string();
            Some((trimmed_element_name, elem.attributes()))
        }
        AnyJsxElement::JsxSelfClosingElement(elem) => {
            let token = &elem.name().ok()?;
            let element_name = token.as_jsx_name()?.value_token().ok()?;
            let trimmed_element_name = element_name.text_trimmed().to_string();
            Some((trimmed_element_name, elem.attributes()))
        }
    }
}

fn get_explicit_role(
    aria_roles: &AriaRoles,
    role_attribute_value: &AnyJsxAttributeValue,
) -> Option<&'static dyn AriaRoleDefinition> {
    let text = match role_attribute_value {
        AnyJsxAttributeValue::JsxString(val) => val.inner_string_text().ok()?,
        AnyJsxAttributeValue::JsxExpressionAttributeValue(val) => match val.expression().ok()? {
            rome_js_syntax::AnyJsExpression::AnyJsLiteralExpression(
                AnyJsLiteralExpression::JsStringLiteralExpression(expr),
            ) => expr.inner_string_text().ok()?,
            rome_js_syntax::AnyJsExpression::JsTemplateExpression(expr) => {
                let first_template_element = expr.elements().into_iter().next()?;
                let first_element = first_template_element
                    .as_js_template_chunk_element()?
                    .template_chunk_token()
                    .ok()?;
                first_element.token_text_trimmed()
            }
            _ => return None,
        },
        _ => return None,
    };

    // If a role attribute has multiple values, the first valid value (specified role) will be used.
    // Check: https://www.w3.org/TR/2014/REC-wai-aria-implementation-20140320/#mapping_role
    let explicit_role = text.split(' ').find_map(|role| aria_roles.get_role(role))?;
    Some(explicit_role)
}
