use rome_analyze::context::RuleContext;
use rome_analyze::{declare_rule, Ast, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_js_syntax::{
    AnyJsExpression, AnyJsLiteralExpression, AnyJsTemplateElement, AnyJsxAttributeValue,
    AnyJsxChild, JsxAttribute, JsxElement, JsxReferenceIdentifier, JsxSelfClosingElement,
};
use rome_rowan::{declare_node_union, AstNode, AstNodeList};

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
    /// ```jsx,expect_diagnostic
    /// <a></a>
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <a>    </a>
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <a aria-hidden>content</a>
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <a><span aria-hidden="true">content</span></a>
    /// ```
    ///
    /// ## Valid
    ///
    /// ```jsx
    /// <a>content</a>
    /// ```
    ///
    /// ```jsx
    /// function html() {
    ///     return { __html: "foo" }
    /// }
    /// <a dangerouslySetInnerHTML={html()} />
    /// ```
    ///
    /// ```jsx
    /// <a><TextWrapper aria-hidden={true} />content</a>
    /// ```
    ///
    /// ```jsx
    /// <a><div aria-hidden="true"></div>content</a>
    /// ```
    pub(crate) UseAnchorContent {
        version: "10.0.0",
        name: "useAnchorContent",
        recommended: true,
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

    /// Check if the `a` element has the `aria-hidden` attribute set to true.
    fn is_hidden_from_screen_reader(&self) -> bool {
        match self {
            UseAnchorContentNode::JsxElement(element) => {
                if let Ok(opening_element) = element.opening_element() {
                    match opening_element.find_attribute_by_name("aria-hidden") {
                        Ok(Some(aria_hidden_attribute)) => {
                            is_aria_hidden_truthy(aria_hidden_attribute).unwrap_or(false)
                        }
                        _ => false,
                    }
                } else {
                    false
                }
            }
            UseAnchorContentNode::JsxSelfClosingElement(element) => {
                match element.find_attribute_by_name("aria-hidden") {
                    Ok(Some(aria_hidden_attribute)) => {
                        is_aria_hidden_truthy(aria_hidden_attribute).unwrap_or(false)
                    }
                    _ => false,
                }
            }
        }
    }

    /// Check if the `a` element has content accessible to screen readers.
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

        // If there's no `aria-hidden` attribute on the `a` element,
        // proceed to check the accessibility of its child elements
        if !node.is_hidden_from_screen_reader() && node.has_accessible_child()? {
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
				"Provide screen reader accessible content when using "<Emphasis>"`a`"</Emphasis>" elements."
			}
        ).note(
			markup! {
				"All links on a page should have content that is accessible to screen readers."
			}
		))
    }
}

/// Check if the element is a text content for screen readers,
/// or it is not hidden using the `aria-hidden` attribute
fn is_accessible_to_screen_reader(element: AnyJsxChild) -> Option<bool> {
    Some(match element {
        AnyJsxChild::JsxText(text) => {
            let value_token = text.value_token().ok()?;
            value_token.text_trimmed().trim() != ""
        }
        AnyJsxChild::JsxElement(element) => {
            let opening_element = element.opening_element().ok()?;

            // We don't check if a component (e.g. <Text aria-hidden />) is using the `aria-hidden` property,
            // since we don't have enough information about how the property is used.
            let element_name = opening_element.name().ok()?;
            if JsxReferenceIdentifier::can_cast(element_name.syntax().kind()) {
                return None;
            }

            let aria_hidden_attribute = opening_element
                .find_attribute_by_name("aria-hidden")
                .ok()??;
            !is_aria_hidden_truthy(aria_hidden_attribute)?
        }
        AnyJsxChild::JsxSelfClosingElement(element) => {
            // We don't check if a component (e.g. <Text aria-hidden />) is using the `aria-hidden` property,
            // since we don't have enough information about how the property is used.
            let element_name = element.name().ok()?;
            if JsxReferenceIdentifier::can_cast(element_name.syntax().kind()) {
                return None;
            }

            let aria_hidden_attribute = element.find_attribute_by_name("aria-hidden").ok()??;
            !is_aria_hidden_truthy(aria_hidden_attribute)?
        }
        AnyJsxChild::JsxExpressionChild(expression) => {
            let expression = expression.expression()?;
            match expression {
                AnyJsExpression::AnyJsLiteralExpression(
                    AnyJsLiteralExpression::JsNullLiteralExpression(_),
                ) => false,
                AnyJsExpression::JsIdentifierExpression(identifier) => {
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
fn is_aria_hidden_truthy(aria_hidden_attribute: JsxAttribute) -> Option<bool> {
    let initializer = aria_hidden_attribute.initializer();
    if initializer.is_none() {
        return Some(true);
    }
    let attribute_value = initializer?.value().ok()?;
    Some(match attribute_value {
        AnyJsxAttributeValue::JsxExpressionAttributeValue(attribute_value) => {
            let expression = attribute_value.expression().ok()?;
            is_expression_truthy(expression)?
        }
        AnyJsxAttributeValue::AnyJsxTag(_) => false,
        AnyJsxAttributeValue::JsxString(aria_hidden_string) => {
            let quoted_string = aria_hidden_string.inner_string_text().ok()?;
            quoted_string.text() == "true"
        }
    })
}

/// Check if the expression contains only one boolean literal `true`
/// or one string literal `"true"`
fn is_expression_truthy(expression: AnyJsExpression) -> Option<bool> {
    Some(match expression {
        AnyJsExpression::AnyJsLiteralExpression(literal_expression) => {
            if let AnyJsLiteralExpression::JsBooleanLiteralExpression(boolean_literal) =
                literal_expression
            {
                let text = boolean_literal.value_token().ok()?;
                text.text_trimmed() == "true"
            } else if let AnyJsLiteralExpression::JsStringLiteralExpression(string_literal) =
                literal_expression
            {
                let quoted_string = string_literal.inner_string_text().ok()?;
                quoted_string.text() == "true"
            } else {
                false
            }
        }
        AnyJsExpression::JsTemplateExpression(template) => {
            let mut iter = template.elements().iter();
            if iter.len() != 1 {
                return None;
            }
            match iter.next() {
                Some(AnyJsTemplateElement::JsTemplateChunkElement(element)) => {
                    let template_token = element.template_chunk_token().ok()?;
                    template_token.text_trimmed() == "true"
                }
                Some(AnyJsTemplateElement::JsTemplateElement(element)) => {
                    let expression = element.expression().ok()?;
                    is_expression_truthy(expression)?
                }
                _ => false,
            }
        }
        _ => false,
    })
}
