use crate::aria_services::Aria;
use rome_analyze::{context::RuleContext, declare_rule, Rule, RuleDiagnostic};
use rome_aria::AriaRoles;
use rome_console::markup;
use rome_js_syntax::{
    jsx_ext::AnyJsxElement, AnyJsExpression, AnyJsLiteralExpression, AnyJsxAttributeValue,
    JsNumberLiteralExpression, JsStringLiteralExpression, JsUnaryExpression, TextRange,
};
use rome_rowan::{declare_node_union, AstNode, AstNodeList};

declare_rule! {
    /// Enforce that `tabIndex` is not assigned to non-interactive HTML elements.
    ///
    /// When using the tab key to navigate a webpage, limit it to interactive elements.
    /// You don't need to add tabindex to items in an unordered list as assistive technology can navigate through the HTML.
    /// Keep the tab ring small, which is the order of elements when tabbing, for a more efficient and accessible browsing experience.
    ///
    /// ESLint (eslint-plugin-jsx-a11y) Equivalent: [no-noninteractive-tabindex](https://github.com/jsx-eslint/eslint-plugin-jsx-a11y/blob/main/docs/rules/no-noninteractive-tabindex.md)
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```jsx,expect_diagnostic
    /// <div tabIndex="0" />
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <div role="article" tabIndex="0" />
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <article tabIndex="0" />
    /// ```
    ///
    /// ## Valid
    ///
    /// ```jsx
    /// <div />
    /// ```
    ///
    /// ```jsx
    /// <MyButton tabIndex={0} />
    /// ```
    ///
    /// ```jsx
    /// <article tabIndex="-1" />
    /// ```
    ///
    pub(crate) NoNoninteractiveTabindex {
        version: "next",
        name: "noNoninteractiveTabindex",
        recommended: false,
    }
}

declare_node_union! {
    /// Subset of expressions supported by this rule.
    ///
    /// ## Examples
    ///
    /// - `JsStringLiteralExpression` &mdash; `"5"`
    /// - `JsNumberLiteralExpression` &mdash; `5`
    /// - `JsUnaryExpression` &mdash; `+5` | `-5`
    ///
    pub(crate) AnyNumberLikeExpression = JsStringLiteralExpression | JsNumberLiteralExpression | JsUnaryExpression
}

impl AnyNumberLikeExpression {
    /// Returns the value of a number-like expression; it returns the expression
    /// text for literal expressions. However, for unary expressions, it only
    /// returns the value for signed numeric expressions.
    pub(crate) fn value(&self) -> Option<String> {
        match self {
            AnyNumberLikeExpression::JsStringLiteralExpression(string_literal) => {
                return Some(string_literal.inner_string_text().ok()?.to_string());
            }
            AnyNumberLikeExpression::JsNumberLiteralExpression(number_literal) => {
                return Some(number_literal.value_token().ok()?.to_string());
            }
            AnyNumberLikeExpression::JsUnaryExpression(unary_expression) => {
                if unary_expression.is_signed_numeric_literal().ok()? {
                    return Some(unary_expression.text());
                }
            }
        }
        None
    }
}

pub(crate) struct RuleState {
    attribute_range: TextRange,
    element_name: String,
}

impl Rule for NoNoninteractiveTabindex {
    type Query = Aria<AnyJsxElement>;
    type State = RuleState;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        if !node.is_element() {
            return None;
        }

        let element_name = node.name().ok()?.as_jsx_name()?.value_token().ok()?;
        let aria_roles = ctx.aria_roles();

        if aria_roles.is_not_interactive_element(element_name.text_trimmed()) {
            let tabindex_attribute = node.find_attribute_by_name("tabIndex")?;
            let tabindex_attribute_value = tabindex_attribute.initializer()?.value().ok()?;
            if attribute_has_negative_tabindex(&tabindex_attribute_value)? {
                return None;
            }

            let role_attribute = node.find_attribute_by_name("role");
            let Some(role_attribute) = role_attribute else {
                    return Some(RuleState {
                        attribute_range: tabindex_attribute.range(),
                        element_name: element_name.text_trimmed().to_string(),
                    })
                };

            let role_attribute_value = role_attribute.initializer()?.value().ok()?;
            if attribute_has_interactive_role(&role_attribute_value, aria_roles)? {
                return None;
            }

            return Some(RuleState {
                attribute_range: tabindex_attribute.range(),
                element_name: element_name.text_trimmed().to_string(),
            });
        }
        None
    }

    fn diagnostic(_: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                state.attribute_range,
                markup! {
                "The HTML element "<Emphasis>{{&state.element_name}}</Emphasis>" is non-interactive. Do not use "<Emphasis>"tabIndex"</Emphasis>"."

                },
            )
            .note(markup! {
                "Adding non-interactive elements to the keyboard navigation flow can confuse users."
            }),
        )
    }
}

/// Verifies if number string is an integer less than 0.
/// Non-integer numbers are considered valid.
fn is_negative_tabindex(number_like_string: &str) -> bool {
    let number_string_result = number_like_string.trim().parse::<i32>();
    match number_string_result {
        Ok(number) => number < 0,
        Err(_) => true,
    }
}

/// Checks if the given tabindex attribute value has negative integer or not.
fn attribute_has_negative_tabindex(
    tabindex_attribute_value: &AnyJsxAttributeValue,
) -> Option<bool> {
    match tabindex_attribute_value {
        AnyJsxAttributeValue::JsxString(jsx_string) => {
            let value = jsx_string.inner_string_text().ok()?.to_string();
            Some(is_negative_tabindex(&value))
        }
        AnyJsxAttributeValue::JsxExpressionAttributeValue(value) => {
            let expression = value.expression().ok()?;
            let expression_value =
                AnyNumberLikeExpression::cast_ref(expression.syntax())?.value()?;
            Some(is_negative_tabindex(&expression_value))
        }
        _ => None,
    }
}

/// Checks if the given role attribute value is interactive or not based on ARIA roles.
fn attribute_has_interactive_role(
    role_attribute_value: &AnyJsxAttributeValue,
    aria_roles: &AriaRoles,
) -> Option<bool> {
    let role_attribute_value = match role_attribute_value {
        AnyJsxAttributeValue::JsxString(string) => string.inner_string_text().ok(),
        AnyJsxAttributeValue::JsxExpressionAttributeValue(expression) => {
            match expression.expression().ok()? {
                AnyJsExpression::AnyJsLiteralExpression(
                    AnyJsLiteralExpression::JsStringLiteralExpression(string),
                ) => string.inner_string_text().ok(),
                AnyJsExpression::JsTemplateExpression(template) => {
                    if template.elements().len() == 1 {
                        template
                            .elements()
                            .iter()
                            .next()?
                            .as_js_template_chunk_element()?
                            .template_chunk_token()
                            .ok()
                            .map(|t| t.token_text_trimmed())
                    } else {
                        None
                    }
                }
                _ => None,
            }
        }
        _ => None,
    }?;
    Some(aria_roles.is_role_interactive(role_attribute_value.text()))
}
