use rome_analyze::{context::RuleContext, declare_rule, Ast, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_js_syntax::{
    JsAnyExpression, JsAnyLiteralExpression, JsObjectExpression, JsStringLiteralExpression,
    JsxAnyElementName, JsxSelfClosingElement, JsxString, TextRange,
};
use rome_rowan::{declare_node_union, AstNode};

declare_rule! {
    /// It asserts that alternative text to images or areas, help to rely on to screen readers to understand the purpose and the context of the image.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```jsx,expect_diagnostic
    /// <img src="image.png" />
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <input type="image" src="image.png" />
    /// ```
    ///
    /// ### Valid
    ///
    /// ```jsx
    /// <img src="image.png" alt="image alt" />
    /// ```
    ///
    /// ```jsx
    /// <input type="image" src="image.png" alt="alt text" />
    /// ```
    ///
    /// ```jsx
    /// <input type="image" src="image.png" aria-label="alt text" />
    /// ```
    ///
    /// ```jsx
    /// <input type="image" src="image.png" aria-labelledby="someId" />
    /// ```

    pub(crate) UseAltText {
        version: "10.0.0",
        name: "useAltText",
        recommended: true,
    }
}

declare_node_union! {
    pub(crate) UseAltTextNode = JsxString | JsxSelfClosingElement | JsStringLiteralExpression | JsObjectExpression
}

impl Rule for UseAltText {
    type Query = Ast<JsxSelfClosingElement>;
    type State = TextRange;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let element = ctx.query();
        let name = element.name().ok()?;
        if is_valid_tag(&name)? {
            if name.as_jsx_name()?.value_token().ok()?.text_trimmed() == "input"
                && !input_has_type_image(element)?
            {
                return None;
            }

            let alt_prop = element.find_attribute_by_name("alt").ok()?;
            if alt_prop.is_none() {
                let aria_label_prop = element.find_attribute_by_name("aria-label").ok()?;
                let aria_labelled_prop = element.find_attribute_by_name("aria-labelledby").ok()?;

                if aria_label_prop.is_some() || aria_labelled_prop.is_some() {
                    return None;
                }
                return Some(element.syntax().text_trimmed_range());
            }

            if let Some(prop) = alt_prop {
                if prop.initializer().is_none() {
                    return Some(element.syntax().text_trimmed_range());
                }
                let attribute_value = prop
                    .initializer()?
                    .value()
                    .ok()?
                    .as_jsx_expression_attribute_value()?
                    .expression()
                    .ok()?;

                match attribute_value {
                    JsAnyExpression::JsAnyLiteralExpression(
                        JsAnyLiteralExpression::JsNullLiteralExpression(null),
                    ) => return Some(null.syntax().text_trimmed_range()),
                    JsAnyExpression::JsAnyLiteralExpression(
                        JsAnyLiteralExpression::JsStringLiteralExpression(string),
                    ) => {
                        if string
                            .value_token()
                            .ok()?
                            .text_trimmed()
                            .contains("undefined")
                        {
                            return Some(string.syntax().text_trimmed_range());
                        }
                    }
                    _ => return None,
                }
            }
        }

        None
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        if state.is_empty() {
            return None;
        }
        let message = markup!(
            "Provide the attribute "<Emphasis>"alt"</Emphasis>" when using "<Emphasis>"img"</Emphasis>", "<Emphasis>"area"</Emphasis>" or "<Emphasis>"input type='image'"</Emphasis>""
        ).to_owned();

        Some(
            RuleDiagnostic::new(rule_category!(), state, message).note(markup! {
                "Meaningful alternative text on elements helps users relying on screen
            readers to understand content's purpose within a page."
            }),
        )
    }
}

/*
This function checks for the attribute `type` for input element where we checking for the input type which is image.
 */
fn input_has_type_image(element: &JsxSelfClosingElement) -> Option<bool> {
    let type_attribute = element.find_attribute_by_name("type").ok()?;

    if let Some(prop) = type_attribute {
        let initializer = prop.initializer()?.value().ok()?;
        let initializer = initializer.as_jsx_string()?;

        if initializer.inner_string_text().ok()? == "image" {
            return Some(true);
        }
        return None;
    }
    None
}

/*
Function to check that the HTML/JSX tag is valid for the lint rule, which check if the tag should be `img`, `input type='image' and `area`.

 */
fn is_valid_tag(name: &JsxAnyElementName) -> Option<bool> {
    Some(match name {
        JsxAnyElementName::JsxName(name) => {
            let name = name.value_token().ok()?;
            matches!(name.text_trimmed(), "input" | "area" | "img")
        }
        _ => false,
    })
}
