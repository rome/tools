use rome_analyze::{context::RuleContext, declare_rule, Ast, Rule, RuleDiagnostic};
use rome_console::{fmt::Display, fmt::Formatter, markup};
use rome_js_syntax::{jsx_ext::AnyJsxElement, TextRange};
use rome_rowan::AstNode;

declare_rule! {
    /// Enforce that all elements that require alternative text have meaningful information to relay back to the end user.
    ///
    /// This is a critical component of accessibility for screen reader users in order for them to understand the content's purpose on the page.
    /// By default, this rule checks for alternative text on the following elements: `<img>`, `<area>`, `<input type="image">`, and `<object>`.
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
    ///
    /// ## Accessibility guidelines
    ///
    /// - [WCAG 1.1.1](https://www.w3.org/WAI/WCAG21/Understanding/non-text-content.html)
    ///
    pub(crate) UseAltText {
        version: "10.0.0",
        name: "useAltText",
        recommended: true,
    }
}

pub enum ValidatedElement {
    Object,
    Img,
    Area,
    Input,
}

impl Display for ValidatedElement {
    fn fmt(&self, fmt: &mut Formatter) -> std::io::Result<()> {
        match self {
            ValidatedElement::Object => fmt.write_markup(markup!(<Emphasis>"title"</Emphasis>)),
            _ => fmt.write_markup(markup!(<Emphasis>"alt"</Emphasis>)),
        }
    }
}

impl Rule for UseAltText {
    type Query = Ast<AnyJsxElement>;
    type State = (ValidatedElement, TextRange);
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let element = ctx.query();

        if element.is_custom_component() {
            return None;
        }

        let has_alt = has_valid_alt_text(element);
        let has_aria_label = has_valid_label(element, "aria-label");
        let has_aria_labelledby = has_valid_label(element, "aria-labelledby");
        match element.name_value_token()?.text_trimmed() {
            "object" => {
                let has_title = has_valid_label(element, "title");

                if !has_title && !has_aria_label && !has_aria_labelledby {
                    match element {
                        AnyJsxElement::JsxOpeningElement(opening_element) => {
                            if !opening_element.has_accessible_child() {
                                return Some((
                                    ValidatedElement::Object,
                                    element.syntax().text_range(),
                                ));
                            }
                        }
                        AnyJsxElement::JsxSelfClosingElement(_) => {
                            return Some((ValidatedElement::Object, element.syntax().text_range()));
                        }
                    }
                }
            }
            "img" => {
                if !has_alt && !has_aria_label && !has_aria_labelledby {
                    return Some((ValidatedElement::Img, element.syntax().text_range()));
                }
            }
            "area" => {
                if !has_alt && !has_aria_label && !has_aria_labelledby {
                    return Some((ValidatedElement::Area, element.syntax().text_range()));
                }
            }
            "input" => {
                if has_type_image_attribute(element)
                    && !has_alt
                    && !has_aria_label
                    && !has_aria_labelledby
                {
                    return Some((ValidatedElement::Input, element.syntax().text_range()));
                }
            }
            _ => {}
        }

        None
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let (validate_element, range) = state;
        let message = markup!(
            "Provide a text alternative through the "{{validate_element}}", "<Emphasis>"aria-label"</Emphasis>" or "<Emphasis>"aria-labelledby"</Emphasis>" attribute"
        ).to_owned();
        Some(
            RuleDiagnostic::new(rule_category!(), range, message).note(markup! {
                "Meaningful alternative text on elements helps users relying on screen readers to understand content's purpose within a page."
            }),
        )
    }
}

fn has_type_image_attribute(element: &AnyJsxElement) -> bool {
    element
        .find_attribute_by_name("type")
        .map_or(false, |attribute| {
            attribute
                .as_static_value()
                .map_or(false, |value| value.as_string_constant() == Some("image"))
        })
}

fn has_valid_alt_text(element: &AnyJsxElement) -> bool {
    element
        .find_attribute_by_name("alt")
        .map_or(false, |attribute| {
            if attribute.initializer().is_none() {
                return false;
            }

            attribute
                .as_static_value()
                .map_or(true, |value| !value.is_null_or_undefined())
                && !element.has_trailing_spread_prop(attribute)
        })
}

fn has_valid_label(element: &AnyJsxElement, name_to_lookup: &str) -> bool {
    element
        .find_attribute_by_name(name_to_lookup)
        .map_or(false, |attribute| {
            if attribute.initializer().is_none() {
                return false;
            }
            attribute.as_static_value().map_or(true, |value| {
                !value.is_null_or_undefined()
                    && !value.as_string_constant().unwrap_or_default().is_empty()
            }) && !element.has_trailing_spread_prop(attribute)
        })
}
