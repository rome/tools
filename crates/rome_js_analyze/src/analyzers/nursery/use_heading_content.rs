use rome_analyze::{context::RuleContext, declare_rule, Ast, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_js_syntax::TextRange;
use rome_js_syntax::{AnyJsxChild, JsxElement, JsxSelfClosingElement};
use rome_rowan::AstNodeList;
use rome_rowan::{declare_node_union, AstNode};

declare_rule! {
    /// Enforce that heading element has some content.
    /// Provide screen reader accessible content when using heading elements.
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
    /// <h1><TextWrapper aria-hidden /></h1>
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
    pub(crate) UseHeadingContent {
        version: "next",
        name: "useHeadingContent",
        recommended: false,
    }
}

declare_node_union! {
    pub(crate) UseHeadingContentNode = JsxElement | JsxSelfClosingElement
}

const HEADING_ELEMENTS: [&str; 6] = ["h1", "h2", "h3", "h4", "h5", "h6"];

impl Rule for UseHeadingContent {
    type Query = Ast<UseHeadingContentNode>;
    type State = TextRange;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();

        if node.is_heading_element()? {
            match node {
                UseHeadingContentNode::JsxElement(element) => {
                    let child_nodes = element.children();
                    if child_nodes.len() == 0 && !node.has_dangerously_set_inner_html_attribute()? {
                        return Some(element.syntax().text_range());
                    }

                    if !child_nodes.into_iter().any(|element| {
                        if let Ok(node) = UseHeadingContentNode::try_from(element) {
                            if let Some(value) = node.has_aria_hidden_attribute() {
                                if value {
                                    return false;
                                }
                            }
                        }
                        true
                    }) {
                        return Some(element.syntax().text_range());
                    }
                }
                UseHeadingContentNode::JsxSelfClosingElement(element) => {
                    if !node.has_dangerously_set_inner_html_attribute()? {
                        return Some(element.syntax().text_range());
                    }
                }
            }
        }

        None
    }

    fn diagnostic(_: &RuleContext<Self>, reference: &Self::State) -> Option<RuleDiagnostic> {
        Some(RuleDiagnostic::new(
            rule_category!(),
            reference,
            markup! {
                "Provide screen reader accessible content when using "<Emphasis>"heading"</Emphasis>"  elements."
            },
        ).note(
            "All headings on a page should have content that is accessible to screen readers."
        ))
    }
}

impl UseHeadingContentNode {
    fn is_heading_element(&self) -> Option<bool> {
        match self {
            UseHeadingContentNode::JsxElement(element) => {
                let name_node = element.opening_element().ok()?.name().ok()?;
                let name = name_node.as_jsx_name()?.value_token().ok()?;
                Some(HEADING_ELEMENTS.contains(&name.text_trimmed()))
            }
            UseHeadingContentNode::JsxSelfClosingElement(element) => {
                let name_node = element.name().ok()?;
                let name = name_node.as_jsx_name()?.value_token().ok()?;
                Some(HEADING_ELEMENTS.contains(&name.text_trimmed()))
            }
        }
    }

    fn has_dangerously_set_inner_html_attribute(&self) -> Option<bool> {
        match self {
            UseHeadingContentNode::JsxElement(element) => {
                let opening_element = element.opening_element().ok()?;
                Some(
                    opening_element
                        .find_attribute_by_name("dangerouslySetInnerHTML")
                        .ok()?
                        .is_some(),
                )
            }
            UseHeadingContentNode::JsxSelfClosingElement(element) => Some(
                element
                    .find_attribute_by_name("dangerouslySetInnerHTML")
                    .ok()?
                    .is_some(),
            ),
        }
    }

    fn has_aria_hidden_attribute(&self) -> Option<bool> {
        match self {
            UseHeadingContentNode::JsxElement(element) => {
                let opening_element = element.opening_element().ok()?;
                Some(
                    opening_element
                        .find_attribute_by_name("aria-hidden")
                        .ok()?
                        .is_some(),
                )
            }
            UseHeadingContentNode::JsxSelfClosingElement(element) => Some(
                element
                    .find_attribute_by_name("aria-hidden")
                    .ok()?
                    .is_some(),
            ),
        }
    }
}

impl TryFrom<AnyJsxChild> for UseHeadingContentNode {
    type Error = ();

    fn try_from(member: AnyJsxChild) -> Result<Self, Self::Error> {
        match member {
            AnyJsxChild::JsxElement(member) => Ok(UseHeadingContentNode::JsxElement(member)),
            AnyJsxChild::JsxSelfClosingElement(member) => {
                Ok(UseHeadingContentNode::JsxSelfClosingElement(member))
            }
            _ => Err(()),
        }
    }
}
