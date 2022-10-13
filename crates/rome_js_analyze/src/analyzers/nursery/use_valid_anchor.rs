use rome_analyze::context::RuleContext;
use rome_analyze::{declare_rule, Ast, Rule, RuleDiagnostic};
use rome_console::{markup, MarkupBuf};
use rome_js_syntax::{
    JsAnyExpression, JsAnyLiteralExpression, JsAnyTemplateElement, JsxAnyAttributeValue,
    JsxAttribute, JsxElement, JsxSelfClosingElement,
};
use rome_rowan::{declare_node_union, AstNode, AstNodeList, TextRange};

declare_rule! {
    /// Enforce that all anchors are valid, and they are navigable elements.
    ///
    /// The anchor element (`<a></a>`) - also called **hyperlink** - is an important element
    /// that allows users to navigate pages, in the same page, same website or on another website.
    ///
    /// While before it was possible to attach logic to an anchor element, with the advent of JSX libraries,
    /// it's now  easier to attach logic to any HTML element, anchors included.
    ///
    /// This rule is designed to prevent users to attach logic at the click of anchors, and also makes
    /// sure that the `href` provided to the anchor element is valid. If the anchor has logic attached to it,
    /// the rules suggests to turn it to a `button`, because that's likely what the user wants.
    ///
    /// Anchor `<a></a>` elements should be used for navigation, while `<button></button>` should be
    /// used for user interaction.
    ///
    /// There are **many reasons** why an anchor should not have a logic and have a correct `href` attribute:
    /// - it can disrupt the correct flow of the user navigation e.g. a user that wants to open the link
    /// in another tab, but the default "click" behaviour is prevented;
    /// - it can source of invalid links, and [crawlers] can't navigate the website, risking to penalise
    /// [SEO] ranking
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```jsx,expect_diagnostic
    /// <a href={null}>navigate here</a>
    /// ```
    /// ```jsx,expect_diagnostic
    /// <a href={undefined}>navigate here</a>
    /// ```
    /// ```jsx,expect_diagnostic
    /// <a href>navigate here</a>
    /// ```
    /// ```jsx,expect_diagnostic
    /// <a href="javascript:void(0)">navigate here</a>
    /// ```
    /// ```jsx,expect_diagnostic
    /// <a href="https://example.com" onClick={something}>navigate here</a>
    /// ```
    /// ### Valid
    ///
    /// ```jsx
    /// <>
    ///     <a href={`https://www.javascript.com`}>navigate here</a>
    ///     <a href={somewhere}>navigate here</a>
    /// </>
    /// ```
    ///
    /// ## Accessibility guidelines
    ///
    /// [WCAG 2.1.1]
    ///
    /// ## Resources
    ///
    /// - [WebAIM - Introduction to Links and Hypertext]
    /// - [Links vs. Buttons in Modern Web Applications]
    /// - [Using ARIA - Notes on ARIA use in HTML]
    ///
    /// [SEO]: https://en.wikipedia.org/wiki/Search_engine_optimization
    /// [crawlers]: https://en.wikipedia.org/wiki/Web_crawler
    /// [WCAG 2.1.1]: https://www.w3.org/WAI/WCAG21/Understanding/keyboard
    /// [WebAIM - Introduction to Links and Hypertext]: https://webaim.org/techniques/hypertext/
    /// [Links vs. Buttons in Modern Web Applications]: https://marcysutton.com/links-vs-buttons-in-modern-web-applications/
    /// [Using ARIA - Notes on ARIA use in HTML]: https://www.w3.org/TR/using-aria/#NOTES
    pub(crate) UseValidAnchor {
        version: "10.0.0",
        name: "useValidAnchor",
        recommended: false,
    }
}

declare_node_union! {
    pub(crate) UseValidAnchorQuery = JsxElement | JsxSelfClosingElement
}

/// Representation of the various states
///
/// The `TextRange` of each variant represents the range of where the issue
/// is found.
pub(crate) enum UseValidAnchorState {
    /// The anchor element has not `href` attribute
    MissingHrefAttribute(TextRange),
    /// The `href` attribute has not value
    HrefNotInitialized(TextRange),
    /// The value assigned to attribute `href` is not valid
    IncorrectHref(TextRange),
    /// The element has `href` and `onClick`
    CantBeAnchor(TextRange),
}

impl UseValidAnchorState {
    fn message(&self) -> MarkupBuf {
        match self {
            UseValidAnchorState::MissingHrefAttribute(_) => {
                (markup! {
                    "Provide a "<Emphasis>"href"</Emphasis>" attribute for the "<Emphasis>"a"</Emphasis>" element."
                }).to_owned()
            },
            UseValidAnchorState::IncorrectHref(_) => {
                (markup! {
                    "Provide a valid value for the attribute "<Emphasis>"href"</Emphasis>"."
                }).to_owned()
            }
            UseValidAnchorState::HrefNotInitialized(_) => {
                (markup! {
                    "The attribute "<Emphasis>"href"</Emphasis>" has to be assigned to a valid value."
                }).to_owned()
            }
            UseValidAnchorState::CantBeAnchor(_) => {
                (markup! {
                    "Use a "<Emphasis>"button"</Emphasis>" element instead of an "<Emphasis>"a"</Emphasis>" element."
                }).to_owned()
            }
        }
    }

    fn note(&self) -> MarkupBuf {
        match self {
            UseValidAnchorState::MissingHrefAttribute(_) => (markup! {
                "An anchor element should always have a "<Emphasis>"href"</Emphasis>""
            })
            .to_owned(),
            UseValidAnchorState::IncorrectHref(_) | UseValidAnchorState::HrefNotInitialized(_) => {
                (markup! {
                    "The href attribute should be a valid a URL"
                })
                .to_owned()
            }
            UseValidAnchorState::CantBeAnchor(_) => (markup! {
                "Anchor elements should only be used for default sections or page navigation"
            })
            .to_owned(),
        }
    }

    fn range(&self) -> &TextRange {
        match self {
            UseValidAnchorState::MissingHrefAttribute(range)
            | UseValidAnchorState::HrefNotInitialized(range)
            | UseValidAnchorState::CantBeAnchor(range)
            | UseValidAnchorState::IncorrectHref(range) => range,
        }
    }
}

impl UseValidAnchorQuery {
    /// Checks if the current element is anchor
    fn is_anchor(&self) -> Option<bool> {
        Some(match self {
            UseValidAnchorQuery::JsxElement(element) => {
                element.opening_element().ok()?.name().ok()?.text() == "a"
            }
            UseValidAnchorQuery::JsxSelfClosingElement(element) => {
                element.name().ok()?.text() == "a"
            }
        })
    }

    /// Finds the `href` attribute
    fn find_href_attribute(&self) -> Option<JsxAttribute> {
        match self {
            UseValidAnchorQuery::JsxElement(element) => element
                .opening_element()
                .ok()?
                .find_attribute_by_name("href")
                .ok()?,
            UseValidAnchorQuery::JsxSelfClosingElement(element) => {
                element.find_attribute_by_name("href").ok()?
            }
        }
    }

    /// Finds the `onClick` attribute
    fn find_on_click_attribute(&self) -> Option<JsxAttribute> {
        match self {
            UseValidAnchorQuery::JsxElement(element) => element
                .opening_element()
                .ok()?
                .find_attribute_by_name("onClick")
                .ok()?,
            UseValidAnchorQuery::JsxSelfClosingElement(element) => {
                element.find_attribute_by_name("onClick").ok()?
            }
        }
    }
}

impl Rule for UseValidAnchor {
    type Query = Ast<UseValidAnchorQuery>;
    type State = UseValidAnchorState;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        if !node.is_anchor()? {
            return None;
        }

        let anchor_attribute = node.find_href_attribute();
        let on_click_attribute = node.find_on_click_attribute();

        match (anchor_attribute, on_click_attribute) {
            (Some(_), Some(_)) => Some(UseValidAnchorState::CantBeAnchor(
                node.syntax().text_trimmed_range(),
            )),
            (Some(anchor_attribute), _) => is_invalid_anchor(&anchor_attribute),
            (None, Some(on_click_attribute)) => Some(UseValidAnchorState::CantBeAnchor(
                on_click_attribute.syntax().text_trimmed_range(),
            )),
            (None, _) => Some(UseValidAnchorState::MissingHrefAttribute(
                node.syntax().text_trimmed_range(),
            )),
        }
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let diagnostic = RuleDiagnostic::new(rule_category!(), state.range(), state.message())
            .note(state.note())
            .note(
            markup! {
                "Check "<Hyperlink href="https://marcysutton.com/links-vs-buttons-in-modern-web-applications">"this thorough explanation"</Hyperlink>" to better understand the context."
            }
        );

        Some(diagnostic)
    }
}

fn is_invalid_anchor(anchor_attribute: &JsxAttribute) -> Option<UseValidAnchorState> {
    let initializer = anchor_attribute.initializer();
    if initializer.is_none() {
        return Some(UseValidAnchorState::HrefNotInitialized(
            anchor_attribute.syntax().text_range(),
        ));
    }

    let attribute_value = initializer?.value().ok()?;

    match attribute_value {
        JsxAnyAttributeValue::JsxExpressionAttributeValue(attribute_value) => {
            let expression = attribute_value.expression().ok()?;
            // href={null}
            if let JsAnyExpression::JsAnyLiteralExpression(
                JsAnyLiteralExpression::JsNullLiteralExpression(null),
            ) = expression
            {
                return Some(UseValidAnchorState::IncorrectHref(
                    null.syntax().text_trimmed_range(),
                ));
            } else if let JsAnyExpression::JsIdentifierExpression(identifier) = expression {
                let text = identifier.name().ok()?.value_token().ok()?;
                // href={undefined}
                if text.text_trimmed() == "undefined" {
                    return Some(UseValidAnchorState::IncorrectHref(
                        text.text_trimmed_range(),
                    ));
                }
            } else if let JsAnyExpression::JsAnyLiteralExpression(
                JsAnyLiteralExpression::JsStringLiteralExpression(string_literal),
            ) = expression
            {
                let text = string_literal.inner_string_text().ok()?;
                if text == "#" {
                    return Some(UseValidAnchorState::IncorrectHref(
                        string_literal.syntax().text_trimmed_range(),
                    ));
                }
            } else if let JsAnyExpression::JsTemplate(template) = expression {
                let mut iter = template.elements().iter();
                if let Some(JsAnyTemplateElement::JsTemplateChunkElement(element)) = iter.next() {
                    let template_token = element.template_chunk_token().ok()?;
                    let text = template_token.text_trimmed();
                    if text == "#" || text.contains("javascript:") {
                        return Some(UseValidAnchorState::IncorrectHref(
                            template_token.text_trimmed_range(),
                        ));
                    }
                }
            } else {
                return Some(UseValidAnchorState::IncorrectHref(
                    expression.syntax().text_trimmed_range(),
                ));
            }
        }
        JsxAnyAttributeValue::JsxAnyTag(_) => {}
        JsxAnyAttributeValue::JsxString(href_string) => {
            let href_value = href_string.inner_string_text().ok()?;

            // href="#" or href="javascript:void(0)"
            if href_value == "#" || href_value.contains("javascript:") {
                return Some(UseValidAnchorState::IncorrectHref(
                    href_string.syntax().text_trimmed_range(),
                ));
            }
        }
    }

    None
}
