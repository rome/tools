use crate::aria_services::Aria;
use rome_analyze::context::RuleContext;
use rome_analyze::{declare_rule, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_js_syntax::jsx_ext::AnyJsxElement;
use rome_js_syntax::{
    AnyJsExpression, AnyJsLiteralExpression, AnyJsTemplateElement, AnyJsxAttributeValue,
    JsSyntaxToken,
};
use rome_rowan::{AstNode};

declare_rule! {
    /// Enforce `img` alt prop does not contain the word "image", "picture", or "photo"
    /// 
    /// Examples
    /// 
    /// ### Invalid
    /// 
    /// ```jsx,expect_diagnostic
    /// <img src="src" alt="photo content" />;
    /// ```
    /// ```jsx,expect_diagnostic
    /// <img src="src" alt="picture content" />;
    /// ```
    /// 
    /// ### Valid
    /// 
    /// ```jsx
    /// <img src="src" alt="alt" />;
    /// <img src="src" alt={photo} />;
    /// <img src="src" alt="content" />;
    /// ```
    ///
    pub(crate) NoRedundantAlt {
        version: "12.0.0",
        name: "noRedundantAlt",
        recommended: true,
    }
}

impl Rule for NoRedundantAlt {
    type Query = Aria<AnyJsxElement>;
    type State = AnyJsxAttributeValue;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        if node
            .name()
            .ok()?
            .as_jsx_name()?
            .value_token()
            .ok()?
            .text_trimmed()
            != "img"
        {
            return None;
        }
        let hidden = node.find_attribute_by_name("aria-hidden");

        if hidden.is_some() {
            let is_false = hidden
                .unwrap()
                .initializer()?
                .value()
                .ok()?
                .as_jsx_expression_attribute_value()?
                .expression()
                .ok()?
                .as_any_js_literal_expression()?
                .as_js_boolean_literal_expression()?
                .value_token()
                .ok()?
                .text_trimmed()
                == "false";
            if !is_false {
                return None;
            }
        }

        let alt = node
            .find_attribute_by_name("alt")?
            .initializer()?
            .value()
            .ok()?;

        match alt {
            AnyJsxAttributeValue::AnyJsxTag(_) => None,
            AnyJsxAttributeValue::JsxExpressionAttributeValue(ref value) => {
                match value.expression().ok()? {
                    AnyJsExpression::AnyJsLiteralExpression(expr) => match expr {
                        AnyJsLiteralExpression::JsStringLiteralExpression(string_literal_expr) => {
                            let token = string_literal_expr.value_token().ok()?;

                            is_redundant_alt(trim_quote(&token)).map(|_| alt)
                        }
                        _ => None,
                    },
                    AnyJsExpression::JsTemplateExpression(expr) => {
                        let contain_redundant_alt = expr.elements().into_iter().any(|x| match x {
                            AnyJsTemplateElement::JsTemplateChunkElement(node) => {
                                node.template_chunk_token().ok().map_or(false, |token| {
                                    is_redundant_alt(token.text_trimmed().to_string())
                                        .map(|_| true)
                                        .unwrap_or(false)
                                })
                            }
                            AnyJsTemplateElement::JsTemplateElement(_) => false,
                        });

                        if contain_redundant_alt {
                            Some(alt)
                        } else {
                            None
                        }
                    }

                    _ => None,
                }
            }
            AnyJsxAttributeValue::JsxString(ref value) => {
                let text = value.inner_string_text().ok()?.to_string();
                is_redundant_alt(text).map(|_| alt)
            }
        }
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                state.range(),
                markup! {
                    "Avoid the words \"image\", \"picture\", or \"photo\" in" <Emphasis>"img"</Emphasis>" element alt text."
                },
            )
            .note(markup! {
                "Screen readers announce img elements as \"images\", so it is not necessary to redeclare this in alternative text."
            }),
        )
    }
}

const REDUNDANT_WORDS: [&str; 3] = ["image", "photo", "picture"];

fn is_redundant_alt(alt: String) -> Option<()> {
    let is_redundant = REDUNDANT_WORDS
        .into_iter()
        .any(|word| alt.split_whitespace().any(|x| x.to_lowercase() == word));

    if is_redundant {
        Some(())
    } else {
        None
    }
}

fn trim_quote(token: &JsSyntaxToken) -> String {
    let trimmed_string = token.text_trimmed().to_string();

    trimmed_string[1..trimmed_string.len() - 1].to_string()
}
