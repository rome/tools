use rome_analyze::context::RuleContext;
use rome_analyze::{declare_rule, Ast, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_js_syntax::{JsxElement, JsxSelfClosingElement};
use rome_rowan::{declare_node_union, AstNode};

use crate::aria::{is_accessible_to_screen_reader, is_aria_hidden_truthy};

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
                            is_aria_hidden_truthy(&aria_hidden_attribute).unwrap_or(false)
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
                        is_aria_hidden_truthy(&aria_hidden_attribute).unwrap_or(false)
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
                .any(|child| is_accessible_to_screen_reader(&child).unwrap_or(true)),
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
