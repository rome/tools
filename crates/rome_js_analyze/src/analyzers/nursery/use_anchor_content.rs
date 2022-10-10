use rome_analyze::context::RuleContext;
use rome_analyze::{declare_rule, Ast, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_js_syntax::{
    JsAnyExpression, JsAnyLiteralExpression, JsxAnyAttributeValue, JsxAnyChild, JsxAttribute,
    JsxElement, JsxSelfClosingElement,
};
use rome_rowan::{declare_node_union, AstNode};

declare_rule! {
    /// Enforce that anchor elements have content and that the content is accessible to screen readers.
    ///
    /// Accessible means that the content is not hidden using the `aria-hidden` attribute.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```jsx,expect_diagnostic
    /// <a />
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// <a></a>
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <a aria-hidden>content</a>
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <a><span aria-hidden="true">content</span></a
    /// ```
    ///
    /// ## Valid
    ///
    /// ```jsx
    /// <a>content</a>
    /// ```
    ///
    /// ```jsx
    /// <a dangerouslySetInnerHTML={{ __html: "foo" }} />
    /// ```
    ///
    /// ```jsx
    /// <a><TextWrapper aria-hidden={true} /> content</a>
    /// ```
    ///
    /// ```jsx
    /// <a><div aria-hidden="true"></div> content</a>
    /// ```
    pub(crate) UseAnchorContent {
        version: "10.0.0",
        name: "useAnchorContent",
        recommended: false,
    }
}

declare_node_union! {
    pub(crate) UseAnchorContentNode = JsxElement | JsxSelfClosingElement
}

impl UseAnchorContentNode {
    /// Check if the current element is an anchor
    fn is_anchor(&self) -> Option<bool> {
        Some(match self {
            UseAnchorContentNode::JsxElement(element) => {
                element.opening_element().ok()?.name().ok()?.text() == "a"
            }
            UseAnchorContentNode::JsxSelfClosingElement(element) => {
                element.name().ok()?.text() == "a"
            }
        })
    }

    /// Check if the a element has the `aria-hidden` attribute set to true.
    /// Return `None` if no `aria-hidden` attribute is present.
    fn is_hidden_from_screen_reader(&self) -> Option<bool> {
        Some(match self {
            UseAnchorContentNode::JsxElement(element) => {
                let aria_hidden_attribute = element
                    .opening_element()
                    .ok()?
                    .find_attribute_by_name("aria-hidden")
                    .ok()??;
                is_hidden_from_screen_reader(aria_hidden_attribute)?
            }
            UseAnchorContentNode::JsxSelfClosingElement(element) => {
                let aria_hidden_attribute =
                    element.find_attribute_by_name("aria-hidden").ok()??;
                is_hidden_from_screen_reader(aria_hidden_attribute)?
            }
        })
    }

    /// Check if the a element has content accessible to screen readers.
    /// Accessible means that the content is not hidden using the `aria-hidden` attribute.
    fn has_accessible_child(&self) -> Option<bool> {
        Some(match self {
            UseAnchorContentNode::JsxElement(element) => element
                .children()
                .into_iter()
                .any(|child| is_accessible_to_screen_reader(child).unwrap_or(true)),
            UseAnchorContentNode::JsxSelfClosingElement(element) => element
                .find_attribute_by_name("dangerouslySetInnerHTML")
                .ok()?
                .is_some(),
        })
    }
}

impl Rule for UseAnchorContent {
    type Query = Ast<UseAnchorContentNode>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        if !node.is_anchor()? {
            return None;
        }

        // If there's no `aria-hidden` attribute on the a element,
        // proceed to check the accessibility of its child elements
        if !node.is_hidden_from_screen_reader().unwrap_or(false) && node.has_accessible_child()? {
            return None;
        }

        Some(())
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        Some(RuleDiagnostic::new(
			rule_category!(),
            node.syntax().text_trimmed_range(),
            markup! {
				"Provide screen reader accessible content when using "<Emphasis>"a"</Emphasis>" elements."
			}
        ).footer_note(
			markup! {
				"All links on a page should have content that is accessible to screen readers."
			}
		))
    }
}

/// Check if the element is a text content for screen readers,
/// or it is not hidden using the `aria-hidden` attribute
fn is_accessible_to_screen_reader(element: JsxAnyChild) -> Option<bool> {
    Some(match element {
        JsxAnyChild::JsxText(text) => text.value_token().is_ok(),
        JsxAnyChild::JsxElement(element) => {
            let opening_element = element.opening_element().ok()?;

            // We don't check if a component (e.g. <Text aria-hidden />) is using the `aria-hidden` property,
            // since we don't have enough information about how the property is used.
            opening_element.name().ok()?.as_jsx_name()?;

            let aria_hidden_attribute = opening_element
                .find_attribute_by_name("aria-hidden")
                .ok()??;
            !is_hidden_from_screen_reader(aria_hidden_attribute)?
        }
        JsxAnyChild::JsxSelfClosingElement(element) => {
            // We don't check if a component (e.g. <Text aria-hidden />) is using the `aria-hidden` property,
            // since we don't have enough information about how the property is used.
            element.name().ok()?.as_jsx_name()?;

            let aria_hidden_attribute = element.find_attribute_by_name("aria-hidden").ok()??;
            !is_hidden_from_screen_reader(aria_hidden_attribute)?
        }
        JsxAnyChild::JsxExpressionChild(expression) => {
            let expression = expression.expression()?;
            match expression {
                JsAnyExpression::JsAnyLiteralExpression(
                    JsAnyLiteralExpression::JsNullLiteralExpression(_),
                ) => false,
                JsAnyExpression::JsIdentifierExpression(identifier) => {
                    let text = identifier.name().ok()?.value_token().ok()?;
                    return Some(text.text_trimmed() != "undefined");
                }
                _ => true,
            }
        }
        _ => true,
    })
}

/// Check if the `aria-hidden` attribute is present or the value is true.
fn is_hidden_from_screen_reader(aria_hidden_attribute: JsxAttribute) -> Option<bool> {
    let initializer = aria_hidden_attribute.initializer();
    if initializer.is_none() {
        return Some(true);
    }
    let attribute_value = initializer?.value().ok()?;
    Some(match attribute_value {
        JsxAnyAttributeValue::JsxExpressionAttributeValue(attribute_value) => {
            let expression = attribute_value.expression().ok()?;
            match expression {
                JsAnyExpression::JsAnyLiteralExpression(literal_expression) => {
                    if let JsAnyLiteralExpression::JsBooleanLiteralExpression(boolean_literal) =
                        literal_expression
                    {
                        let text = boolean_literal.value_token().ok()?;
                        text.text_trimmed() == "true"
                    } else if let JsAnyLiteralExpression::JsStringLiteralExpression(
                        string_literal,
                    ) = literal_expression
                    {
                        let text = string_literal.inner_string_text().ok()?;
                        text == "true"
                    } else {
                        false
                    }
                }
                _ => false,
            }
        }
        JsxAnyAttributeValue::JsxAnyTag(_) => false,
        JsxAnyAttributeValue::JsxString(aria_hidden_string) => {
            let aria_hidden_value = aria_hidden_string.inner_string_text().ok()?;
            aria_hidden_value == "true"
        }
    })
}
