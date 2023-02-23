use rome_analyze::{context::RuleContext, declare_rule, Ast, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_js_syntax::{
    AnyJsxAttribute, AnyJsxChild, JsxAttribute, JsxElement, JsxSelfClosingElement,
};
use rome_rowan::{declare_node_union, AstNode};

use crate::aria::{is_accessible_to_screen_reader, is_aria_hidden_truthy};

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
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();

        if node.is_heading_element()? {
            if node.has_truthy_aria_hidden_attribute()? {
                return Some(());
            }

            if node.has_valid_children_attribute()? || node.has_spread_prop()? {
                return None;
            }

            if !node.has_dangerously_set_inner_html_attribute() {
                match node {
                    UseHeadingContentNode::JsxElement(element) => {
                        if !element.children().into_iter().any(|child_node| {
                            is_accessible_to_screen_reader(&child_node) != Some(false)
                        }) {
                            return Some(());
                        }
                    }
                    UseHeadingContentNode::JsxSelfClosingElement(_) => {
                        return Some(());
                    }
                }
            }
        }

        None
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        let range = ctx.query().syntax().text_trimmed_range();
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

impl UseHeadingContentNode {
    fn is_heading_element(&self) -> Option<bool> {
        let name_node = match self {
            UseHeadingContentNode::JsxElement(element) => {
                element.opening_element().ok()?.name().ok()?
            }
            UseHeadingContentNode::JsxSelfClosingElement(element) => element.name().ok()?,
        };
        Some(
            HEADING_ELEMENTS.contains(&name_node.as_jsx_name()?.value_token().ok()?.text_trimmed()),
        )
    }

    fn find_attribute_by_name(&self, name: &str) -> Option<JsxAttribute> {
        match self {
            UseHeadingContentNode::JsxElement(element) => {
                let opening_element = element.opening_element().ok()?;
                opening_element.find_attribute_by_name(name).ok()?
            }
            UseHeadingContentNode::JsxSelfClosingElement(element) => {
                element.find_attribute_by_name(name).ok()?
            }
        }
    }

    fn has_dangerously_set_inner_html_attribute(&self) -> bool {
        self.find_attribute_by_name("dangerouslySetInnerHTML")
            .is_some()
    }

    fn has_truthy_aria_hidden_attribute(&self) -> Option<bool> {
        if let Some(attribute) = self.find_attribute_by_name("aria-hidden") {
            Some(!self.has_trailing_spread_prop(&attribute)? && is_aria_hidden_truthy(&attribute)?)
        } else {
            Some(false)
        }
    }

    fn has_valid_children_attribute(&self) -> Option<bool> {
        if let Some(attribute) = self.find_attribute_by_name("children") {
            if attribute.initializer().is_some()
                && !(attribute.is_value_undefined_or_null() || attribute.is_value_empty_string())
            {
                return Some(true);
            }
        }

        Some(false)
    }

    fn has_trailing_spread_prop(&self, current_attribute: &JsxAttribute) -> Option<bool> {
        match self {
            UseHeadingContentNode::JsxElement(element) => {
                let opening_element = element.opening_element().ok()?;
                Some(opening_element.has_trailing_spread_prop(current_attribute.clone()))
            }
            UseHeadingContentNode::JsxSelfClosingElement(element) => {
                Some(element.has_trailing_spread_prop(current_attribute.clone()))
            }
        }
    }

    fn has_spread_prop(&self) -> Option<bool> {
        let attrs = match self {
            UseHeadingContentNode::JsxElement(element) => {
                element.opening_element().ok()?.attributes()
            }
            UseHeadingContentNode::JsxSelfClosingElement(element) => element.attributes(),
        };

        Some(
            attrs
                .into_iter()
                .any(|attribute| matches!(attribute, AnyJsxAttribute::JsxSpreadAttribute(_))),
        )
    }
}
