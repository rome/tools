use crate::aria_services::Aria;
use rome_analyze::{context::RuleContext, declare_rule, Rule, RuleDiagnostic};
use rome_aria::{roles::AriaRoleDefinition, AriaRoles};
use rome_console::markup;
use rome_js_syntax::{jsx_ext::AnyJsxElement, AnyJsLiteralExpression, AnyJsxAttributeValue};
use rome_rowan::AstNode;

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
    redundant_element: AnyJsxAttributeValue,
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

        if let AnyJsxElement::JsxOpeningElement(_) = &node {
            let token = &node.name_value_token()?;
            let element_name = token.text_trimmed();

            let defined_attributes = ctx.extract_defined_attributes(&node.attributes())?;
            let implicit_role = aria_roles.get_implicit_role(element_name, defined_attributes)?;

            let role_attribute = &node.find_attribute_by_name("role")?;
            let role_attribute_value = role_attribute.initializer()?.value().ok()?;
            let explicit_role = get_explicit_role(aria_roles, &role_attribute_value)?;

            let is_redundant = implicit_role.type_name() == explicit_role.type_name();
            if is_redundant {
                return Some(RuleState {
                    redundant_element: role_attribute_value,
                    element_name: element_name.to_string(),
                });
            }
        }
        None
    }

    fn diagnostic(_: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let binding = state.redundant_element.inner_text_value().ok()??;
        let role_attribute = binding.text();
        let element = state.element_name.to_string();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                state.redundant_element.range(),
                markup! {
                    "Using the role attribute '"{role_attribute}"' on the '"{element}"' element is redundant."
                },
            )
            .note(markup! {
                ""
            }),
        )
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

    let text = text.to_lowercase();
    let role = text.split(' ').collect::<Vec<&str>>();
    let role = role.first()?;
    aria_roles.get_role(role)
}
