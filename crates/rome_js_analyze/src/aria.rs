use rome_js_syntax::{
    AnyJsExpression, AnyJsLiteralExpression, AnyJsTemplateElement, AnyJsxAttributeValue,
    AnyJsxChild, JsxAttribute, JsxReferenceIdentifier,
};
use rome_rowan::{AstNode, AstNodeList};

/// Check if the element is a text content for screen readers,
/// or it is not hidden using the `aria-hidden` attribute
pub fn is_accessible_to_screen_reader(element: &AnyJsxChild) -> Option<bool> {
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
            opening_element.has_trailing_spread_prop(aria_hidden_attribute.clone())
                || !is_aria_hidden_truthy(&aria_hidden_attribute)?
        }
        AnyJsxChild::JsxSelfClosingElement(element) => {
            // We don't check if a component (e.g. <Text aria-hidden />) is using the `aria-hidden` property,
            // since we don't have enough information about how the property is used.
            let element_name = element.name().ok()?;
            if JsxReferenceIdentifier::can_cast(element_name.syntax().kind()) {
                return None;
            }

            let aria_hidden_attribute = element.find_attribute_by_name("aria-hidden").ok()??;
            element.has_trailing_spread_prop(aria_hidden_attribute.clone())
                || !is_aria_hidden_truthy(&aria_hidden_attribute)?
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
        AnyJsxChild::JsxFragment(fragment) => fragment
            .children()
            .iter()
            .any(|child| is_accessible_to_screen_reader(&child).unwrap_or(false)),
        _ => true,
    })
}

/// Check if the `aria-hidden` attribute is present or the value is true.
pub fn is_aria_hidden_truthy(aria_hidden_attribute: &JsxAttribute) -> Option<bool> {
    let initializer = aria_hidden_attribute.initializer();
    if initializer.is_none() {
        return Some(true);
    }
    let attribute_value = initializer?.value().ok()?;
    Some(match attribute_value {
        AnyJsxAttributeValue::JsxExpressionAttributeValue(attribute_value) => {
            let expression = attribute_value.expression().ok()?;
            is_expression_truthy(&expression)?
        }
        AnyJsxAttributeValue::AnyJsxTag(_) => false,
        AnyJsxAttributeValue::JsxString(aria_hidden_string) => {
            let aria_hidden_value = aria_hidden_string.inner_string_text().ok()?;
            aria_hidden_value == "true"
        }
    })
}

/// Check if the expression contains only one boolean literal `true`
/// or one string literal `"true"`
fn is_expression_truthy(expression: &AnyJsExpression) -> Option<bool> {
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
                let text = string_literal.inner_string_text().ok()?;
                text == "true"
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
                    is_expression_truthy(&expression)?
                }
                _ => false,
            }
        }
        _ => false,
    })
}
