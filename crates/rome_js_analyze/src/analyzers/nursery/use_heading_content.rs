use rome_analyze::{context::RuleContext, declare_rule, Ast, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_js_syntax::{jsx_ext::AnyJsxElement, JsxElement};
use rome_rowan::AstNode;

declare_rule! {
    /// Enforce that heading elements (h1, h2, etc.) have content and that the content is accessible to screen readers.
    /// Accessible means that it is not hidden using the aria-hidden prop.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```jsx,expect_diagnostic
    /// <h1 />
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <h1><div aria-hidden /></h1>
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <h1></h1>
    /// ```
    ///
    /// ## Valid
    ///
    /// ```jsx
    /// <h1>heading</h1>
    /// ```
    ///
    /// ```jsx
    /// <h1><div aria-hidden="true"></div>visible content</h1>
    /// ```
    ///
    /// ```jsx
    /// <h1 dangerouslySetInnerHTML={{ __html: "heading" }} />
    /// ```
    ///
    /// ```jsx
    /// <h1><div aria-hidden />visible content</h1>
    /// ```
    ///
    /// ## Accessibility guidelines
    ///
    /// - [WCAG 2.4.6](https://www.w3.org/TR/UNDERSTANDING-WCAG20/navigation-mechanisms-descriptive.html)
    ///
    pub(crate) UseHeadingContent {
        version: "next",
        name: "useHeadingContent",
        recommended: false,
    }
}

const HEADING_ELEMENTS: [&str; 6] = ["h1", "h2", "h3", "h4", "h5", "h6"];

impl Rule for UseHeadingContent {
    type Query = Ast<AnyJsxElement>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let name = node.name().ok()?.name_value_token()?;

        if HEADING_ELEMENTS.contains(&name.text_trimmed()) {
            if node.has_truthy_attribute("aria-hidden") {
                return Some(());
            }

            if has_valid_heading_content(node) {
                return None;
            }

            match node {
                AnyJsxElement::JsxOpeningElement(_) => {
                    let children = node.parent::<JsxElement>()?.children();
                    if !children
                        .into_iter()
                        .any(|child| child.is_accessible_node().unwrap_or(true))
                    {
                        return Some(());
                    }
                }
                AnyJsxElement::JsxSelfClosingElement(_) => return Some(()),
            }
        }

        None
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        let range = match ctx.query() {
            AnyJsxElement::JsxOpeningElement(node) => {
                node.parent::<JsxElement>()?.syntax().text_trimmed_range()
            }
            AnyJsxElement::JsxSelfClosingElement(node) => node.syntax().text_trimmed_range(),
        };
        Some(RuleDiagnostic::new(
            rule_category!(),
            range,
            markup! {
                "Provide screen reader accessible content when using "<Emphasis>"heading"</Emphasis>"  elements."
            },
        ).note(
            "All headings on a page should have content that is accessible to screen readers."
        ))
    }
}

/// check if the node has a valid heading attribute
fn has_valid_heading_content(node: &AnyJsxElement) -> bool {
    node.find_attribute_by_name("dangerouslySetInnerHTML")
        .is_some()
        || node
            .find_attribute_by_name("children")
            .map_or(false, |attribute| {
                if attribute.initializer().is_none() {
                    return false;
                }
                attribute
                    .as_static_value()
                    .map_or(true, |attribute| !attribute.is_falsy())
            })
        || node.has_spread_prop()
}
