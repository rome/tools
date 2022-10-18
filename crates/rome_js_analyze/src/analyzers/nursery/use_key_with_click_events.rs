use rome_analyze::{context::RuleContext, declare_rule, Ast, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_js_syntax::{JsxAnyAttribute, JsxOpeningElement, JsxSelfClosingElement};
use rome_rowan::{declare_node_union, AstNode, AstNodeList};

declare_rule! {
    /// Enforce to have the `onClick` mouse event with the `onKeyUp`, the `onKeyDown`, or the `noKeyPress` keyboard event.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```jsx,expect_diagnostic
    /// <div onClick={() => {}} />
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <div onClick={() => {}} ></div>
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <div {...spread} onClick={() => {}} ></div>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```jsx
    /// <div onClick={() => {}} onKeyDown={handleKeyDown} />
    ///```
    ///
    /// ```jsx
    /// <div onClick={() => {}} onKeyUp={handleKeyUp} />
    ///```
    ///
    /// ```jsx
    /// <div onClick={() => {}} onKeyPress={handleKeyPress} />
    ///```
    ///
    /// ```jsx
    /// // this rule doesn't apply to user created component
    /// <MyComponent onClick={() => {}} />
    ///```
    ///
    /// ```jsx,
    /// <div onClick={() => {}} {...spread}></div>
    /// ```
    ///
    /// ```jsx
    /// <div {...spread} onClick={() => {}} ></div>
    /// ```
    pub(crate) UseKeyWithClickEvents {
        version: "10.0.0",
        name: "useKeyWithClickEvents",
        recommended: false,
    }
}

declare_node_union! {
    pub(crate) JsxAnyElement = JsxOpeningElement | JsxSelfClosingElement
}

impl JsxAnyElement {
    fn has_spread_attribute(&self) -> bool {
        match self {
            JsxAnyElement::JsxOpeningElement(element) => element
                .attributes()
                .iter()
                .any(|attribute| matches!(attribute, JsxAnyAttribute::JsxSpreadAttribute(_))),
            JsxAnyElement::JsxSelfClosingElement(element) => element
                .attributes()
                .iter()
                .any(|attribute| matches!(attribute, JsxAnyAttribute::JsxSpreadAttribute(_))),
        }
    }
}

impl UseKeyWithClickEvents {
    const REQUIRED_PROPS: [&'static str; 3] = ["onKeyDown", "onKeyUp", "onKeyPress"];
}

impl Rule for UseKeyWithClickEvents {
    type Query = Ast<JsxAnyElement>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();

        match node {
            JsxAnyElement::JsxOpeningElement(element) => {
                let on_click_attribute = element.find_attribute_by_name("onClick").ok()?;
                if element.name().ok()?.as_jsx_name().is_none()
                    || on_click_attribute.is_none()
                    || node.has_spread_attribute()
                {
                    return None;
                }

                for attribute in element.attributes().into_iter() {
                    if let JsxAnyAttribute::JsxAttribute(attribute) = attribute {
                        let name = attribute
                            .name()
                            .ok()?
                            .as_jsx_name()?
                            .syntax()
                            .text_trimmed()
                            .to_string();

                        if Self::REQUIRED_PROPS.contains(&name.as_str()) {
                            return None;
                        }
                    }
                }

                Some(())
            }
            JsxAnyElement::JsxSelfClosingElement(element) => {
                let on_click_attribute = element.find_attribute_by_name("onClick").ok()?;
                if element.name().ok()?.as_jsx_name().is_none()
                    || on_click_attribute.is_none()
                    || node.has_spread_attribute()
                {
                    return None;
                }

                for attribute in element.attributes().into_iter() {
                    if let JsxAnyAttribute::JsxAttribute(attribute) = attribute {
                        let name = attribute
                            .name()
                            .ok()?
                            .as_jsx_name()?
                            .syntax()
                            .text_trimmed()
                            .to_string();

                        if Self::REQUIRED_PROPS.contains(&name.as_str()) {
                            return None;
                        }
                    }
                }

                Some(())
            }
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();

        Some(RuleDiagnostic::new(
            rule_category!(),
            node.range(),
            markup! {
                "Enforce to have the "<Emphasis>"onClick"</Emphasis>" mouse event with the "<Emphasis>"onKeyUp"</Emphasis>", the "<Emphasis>"onKeyDown"</Emphasis>", or the "<Emphasis>"onKeyPress"</Emphasis>" keyboard event."
            },
        ).note(markup! {
            "Actions triggered using mouse events should have corresponding keyboard events to account for keyboard-only navigation."
        }))
    }
}
