use crate::react::{ReactApiCall, ReactCreateElementCall};
use crate::semantic_services::Semantic;
use rome_analyze::context::RuleContext;
use rome_analyze::{declare_rule, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_diagnostics::Severity;
use rome_js_syntax::{
    JsAnyExpression, JsAnyLiteralExpression, JsCallExpression, JsPropertyObjectMember,
    JsxAnyAttributeValue, JsxAttribute, JsxOpeningElement, JsxSelfClosingElement,
};
use rome_rowan::{declare_node_union, AstNode, SyntaxTokenText};

declare_rule! {
    /// TODO
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    pub(crate) NoPositiveTabindex {
        version: "0.10.0",
        name: "noPositiveTabindex",
        recommended: false,
    }
}

declare_node_union! {
    pub(crate) JsAnyCreateElement = JsxAttribute | JsCallExpression
}

declare_node_union! {
    pub(crate) NoPositiveTabindexQuery = JsxOpeningElement | JsxSelfClosingElement | JsCallExpression
}

pub(crate) enum NoPositiveTabindexState {
    Attribute(JsxAttribute),
    MemberProp(JsPropertyObjectMember),
}

impl Rule for NoPositiveTabindex {
    type Query = Semantic<NoPositiveTabindexQuery>;
    type State = NoPositiveTabindexState;
    type Signals = Option<Self::State>;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let model = ctx.model();

        match node {
            NoPositiveTabindexQuery::JsxOpeningElement(opening_element) => {
                let attribute = opening_element.find_attribute_by_name("tabIndex").ok()?;

                if let Some(is_valid) = is_jsx_attribute_valid(&attribute) {
                    if !is_valid {
                        return Some(NoPositiveTabindexState::Attribute(
                            attribute.unwrap().clone(),
                        ));
                    }
                }
            }
            NoPositiveTabindexQuery::JsxSelfClosingElement(self_closing_element) => {
                let attribute = self_closing_element
                    .find_attribute_by_name("tabIndex")
                    .ok()?;

                if let Some(is_valid) = is_jsx_attribute_valid(&attribute) {
                    if !is_valid {
                        return Some(NoPositiveTabindexState::Attribute(
                            attribute.unwrap().clone(),
                        ));
                    }
                }
            }
            NoPositiveTabindexQuery::JsCallExpression(expression) => {
                let react_create_element =
                    ReactCreateElementCall::from_call_expression(expression, model)?;

                let tabindex_prop = react_create_element.find_prop_by_name("tabIndex");

                if let Some(prop) = tabindex_prop {
                    let value = prop.value().ok()?;

                    if let Some(expression_value) = get_expression_value(&value) {
                        let is_valid = is_valid_tabindex(&expression_value);

                        if !is_valid {
                            return Some(NoPositiveTabindexState::MemberProp(prop.clone()));
                        }
                    }
                }
            }
        }

        None
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let text_range = match state {
            NoPositiveTabindexState::Attribute(jsx_attribute) => {
                let name = jsx_attribute.name().ok()?;
                name.syntax().text_trimmed_range()
            }
            NoPositiveTabindexState::MemberProp(object_member) => {
                let name = object_member.name().ok()?;
                name.syntax().text_trimmed_range()
            }
        };

        let diagnostic = RuleDiagnostic::new(
            rule_category!(),
            text_range,
            markup! { "Avoid positive values for the "<Emphasis>"tabIndex"</Emphasis>" prop." }
                .to_owned(),
        )
        .footer(
            Severity::Note,
            "Elements with a positive tab index override natural page content order. This causes elements without a positive tab index to come last when navigating using a keyboard.",
        );

        Some(diagnostic)
    }
}

fn get_expression_value(expression: &JsAnyExpression) -> Option<SyntaxTokenText> {
    match expression {
        JsAnyExpression::JsAnyLiteralExpression(
            JsAnyLiteralExpression::JsStringLiteralExpression(literal),
        ) => literal.inner_string_text().ok(),
        JsAnyExpression::JsAnyLiteralExpression(
            JsAnyLiteralExpression::JsNumberLiteralExpression(literal),
        ) => Some(literal.value_token().ok()?.token_text()),
        _ => None,
    }
}

fn is_jsx_attribute_valid(jsx_attribute: &Option<JsxAttribute>) -> Option<bool> {
    if let Some(attribute) = jsx_attribute {
        let initializer = attribute.initializer()?.value().ok()?;

        match initializer {
            JsxAnyAttributeValue::JsxString(value) => {
                let literal_string = value.inner_string_text().ok()?;
                return Some(is_valid_tabindex(&literal_string));
            }
            JsxAnyAttributeValue::JsxExpressionAttributeValue(value) => {
                let expression = value.expression().ok()?;

                if let Some(expression_value) = get_expression_value(&expression) {
                    return Some(is_valid_tabindex(&expression_value));
                }
            }
            _ => return Some(true),
        }
    }

    return Some(true);
}

fn is_valid_tabindex(token_text: &SyntaxTokenText) -> bool {
    let number_string_result = token_text.trim().parse::<i32>();

    println!("INNER STRING TEXT {:#?}", token_text.to_string());

    match number_string_result {
        Ok(number) => {
            println!("NUMBER {}", number);
            return number <= 0;
        }
        Err(e) => {
            println!("ERROR {}", e);
            return true;
        }
    }
}
