use rome_analyze::context::RuleContext;
use rome_analyze::{declare_rule, Ast, Rule, RuleDiagnostic};
use rome_console::{markup, MarkupBuf};
use rome_js_syntax::jsx_ext::AnyJsxElement;
use rome_rowan::{AstNode, TextRange};

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
    /// in another tab, but the default "click" behaviour is prevented
    /// - it can source of invalid links, and crawlers can't navigate the website, risking to penalise
    /// SEO ranking
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
    /// <a href={`https://www.javascript.com`}>navigate here</a>
    /// ```
    ///
    /// ```jsx
    /// <a href={somewhere}>navigate here</a>
    /// ```
    ///
    /// ```jsx
    /// <a {...spread}>navigate here</a>
    /// ```
    ///
    /// ## Accessibility guidelines
    ///
    /// - [WCAG 2.1.1](https://www.w3.org/WAI/WCAG21/Understanding/keyboard)
    ///
    pub(crate) UseValidAnchor {
        version: "10.0.0",
        name: "useValidAnchor",
        recommended: true,
    }
}

/// Representation of the various states
///
/// The `TextRange` of each variant represents the range of where the issue is found.
pub(crate) enum UseValidAnchorState {
    /// The anchor element has not `href` attribute
    MissingHrefAttribute(TextRange),
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
            UseValidAnchorState::IncorrectHref(_) => (markup! {
                "The href attribute should be a valid a URL"
            })
            .to_owned(),
            UseValidAnchorState::CantBeAnchor(_) => (markup! {
                "Anchor elements should only be used for default sections or page navigation"
            })
            .to_owned(),
        }
    }

    fn range(&self) -> &TextRange {
        match self {
            UseValidAnchorState::MissingHrefAttribute(range)
            | UseValidAnchorState::CantBeAnchor(range)
            | UseValidAnchorState::IncorrectHref(range) => range,
        }
    }
}

impl Rule for UseValidAnchor {
    type Query = Ast<AnyJsxElement>;
    type State = UseValidAnchorState;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let name = node.name().ok()?.name_value_token()?;

        if name.text_trimmed() == "a" {
            let anchor_attribute = node.find_attribute_by_name("href");
            let on_click_attribute = node.find_attribute_by_name("onClick");

            match (anchor_attribute, on_click_attribute) {
                (Some(_), Some(_)) => {
                    return Some(UseValidAnchorState::CantBeAnchor(
                        node.syntax().text_trimmed_range(),
                    ))
                }
                (Some(anchor_attribute), _) => {
                    if anchor_attribute.initializer().is_none() {
                        return Some(UseValidAnchorState::IncorrectHref(
                            anchor_attribute.syntax().text_trimmed_range(),
                        ));
                    }

                    let static_value = anchor_attribute.as_static_value()?;
                    if static_value.as_string_constant().map_or(true, |const_str| {
                        const_str.is_empty()
                            || const_str == "#"
                            || const_str.contains("javascript:")
                    }) {
                        return Some(UseValidAnchorState::IncorrectHref(
                            anchor_attribute.syntax().text_trimmed_range(),
                        ));
                    }
                }
                (None, Some(on_click_attribute)) => {
                    return Some(UseValidAnchorState::CantBeAnchor(
                        on_click_attribute.syntax().text_trimmed_range(),
                    ))
                }
                (None, None) => {
                    if !node.has_spread_prop() {
                        return Some(UseValidAnchorState::MissingHrefAttribute(
                            node.syntax().text_trimmed_range(),
                        ));
                    }
                }
            };
        }

        None
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
