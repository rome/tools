use rome_analyze::{context::RuleContext, declare_rule, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_diagnostics::Severity;
use rome_js_syntax::{
    JsCallExpression, JsObjectExpression, JsStringLiteralExpression, JsxAnyElementName,
    JsxSelfClosingElement, JsxString,
};
use rome_rowan::{declare_node_union, AstNode, AstNodeList};

use crate::semantic_services::Semantic;

declare_rule! {
    /// It asserts that alternative text to images or areas, help to rely on to screen readers to understand the purpose and the context of the image.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```js,expect_diagnostic
    /// <img src="image.png" />
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// <input type="image" src="image.png" />
    /// ```
    ///
    /// ### Valid
    ///
    /// ```js,expect_diagnostic
    /// <img src="image.png" alt="image alt" />
    /// ```
    ///
    /// ```js,expect_diagnostic
    /// <input type="image" src="image.png" alt="alt text" />
    /// ```
    ///

    pub(crate) UseAltText{
        version:"0.10.0",
        name:"useAltText",
        recommended: false,
    }
}

declare_node_union! {
    pub (crate) UseAltTextQuery = JsxSelfClosingElement  | JsCallExpression
}

declare_node_union! {
    pub(crate) UseAltTextNode = JsxString | JsxSelfClosingElement | JsStringLiteralExpression | JsObjectExpression
}

pub(crate) struct UseAltTextState {
    node: UseAltTextNode,
    missing_alt_prop: bool,
}

impl Rule for UseAltText {
    type Query = Semantic<UseAltTextQuery>;
    type State = UseAltTextState;
    type Signals = Option<Self::State>;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        match node {
            UseAltTextQuery::JsxSelfClosingElement(element) => {
                let name = element.name().ok()?;
                if is_valid_tag(&name)? {
                    let _attributes = element.attributes();

                    if name.as_jsx_name()?.value_token().ok()?.text_trimmed() == "input" {
                        if !is_valid_input(element)? {
                            return None;
                        }
                    }

                    let is_spread_available = element
                        .attributes()
                        .iter()
                        .any(|attribute| attribute.as_jsx_spread_attribute().is_some());

                    let alt_prop = element.find_attribute_by_name("alt").ok()?;
                    if alt_prop.is_none() {
                        let aria_label_prop = element.find_attribute_by_name("aria-label").ok()?;
                        let aria_labelled_prop =
                            element.find_attribute_by_name("aria-labelledby").ok()?;

                        if aria_label_prop.is_some() || aria_labelled_prop.is_some() {
                            return None;
                        }
                        return Some(UseAltTextState {
                            node: UseAltTextNode::from(element.clone()),
                            missing_alt_prop: true,
                        });
                    }

                    if let Some(prop) = alt_prop {
                        if prop.initializer().is_none() && !is_spread_available {
                            return Some(UseAltTextState {
                                node: UseAltTextNode::from(element.clone()),
                                missing_alt_prop: true,
                            });
                        }
                        if prop
                            .initializer()?
                            .value()
                            .ok()?
                            .text()
                            .contains("undefined")
                        {
                            return Some(UseAltTextState {
                                node: UseAltTextNode::from(element.clone()),
                                missing_alt_prop: true,
                            });
                        }
                    }
                }

                None
            }
            UseAltTextQuery::JsCallExpression(_expression) => {
                println!("todo...");
                None
            }
        }
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let message = if state.missing_alt_prop {
            (markup!(
                "Provide "<Emphasis>"alt"</Emphasis>" when using "<Emphasis>"img"</Emphasis>", "<Emphasis>"area"</Emphasis>", "<Emphasis>"input type='image'"</Emphasis>""
            )).to_owned()
        } else {
            (markup!(
                "Provide "<Emphasis>"alt"</Emphasis>" when using "<Emphasis>"img"</Emphasis>", "<Emphasis>"area"</Emphasis>", "<Emphasis>"input type='image'"</Emphasis>""
            )).to_owned()
        };

        Some(
            RuleDiagnostic::new(
                rule_category!(),
                state.node.syntax().text_trimmed_range(),
                message,
            )
            .footer(
                Severity::Note,
                markup! {
                    "Meaningful alternative text on elements helps users relying on screen
                readers to understand content's purpose within a page."
                },
            ),
        )
    }
}

fn is_valid_input(element: &JsxSelfClosingElement) -> Option<bool> {
    let type_attribute = element.find_attribute_by_name("type").ok()?;

    if let Some(prop) = type_attribute {
        let initalizer = prop.initializer()?.value().ok()?;
        let initalizer = initalizer.as_jsx_string()?;

        if initalizer.inner_string_text().ok()? == "image" {
            return Some(true);
        }
        return None;
    }
    None
}

fn is_valid_tag(name: &JsxAnyElementName) -> Option<bool> {
    Some(match name {
        JsxAnyElementName::JsxName(name) => {
            let name = name.value_token().ok()?;
            name.text_trimmed() == "input"
                || name.text_trimmed() == "img"
                || name.text_trimmed() == "area"
        }
        _ => false,
    })
}
