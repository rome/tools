use crate::JsRuleAction;
use rome_analyze::{context::RuleContext, declare_rule, Ast, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_js_syntax::{JsxOpeningElement, JsxSelfClosingElement};
use rome_rowan::{declare_node_union, AstNode};

declare_rule! {
    /// Pair the `onClick` mouse event with the `onKeyUp`, the `onKeyDown`, or the `noKeyPress` keyboard event.
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
    pub(crate) UseKeyWithClickEvents {
        version: "10.0.0",
        name: "useKeyWithClickEvents",
        recommended: false,
    }
}

declare_node_union! {
    pub(crate) JsxAnyElement = JsxOpeningElement | JsxSelfClosingElement
}

impl Rule for UseKeyWithClickEvents {
    type Query = Ast<JsxAnyElement>;
    type State = ();
    type Signals = Option<Self::State>;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();

        let required_props = ["onKeyDown", "onKeyUp", "onKeyPress"];

        match node {
            JsxAnyElement::JsxOpeningElement(element) => {
                element.name().ok()?.as_jsx_name()?;
                element.find_attribute_by_name("onClick").ok()??;

                match required_props
                    .iter()
                    .find_map(|key_event| element.find_attribute_by_name(key_event).ok().flatten())
                {
                    Some(_) => None,
                    None => Some(()),
                }
            }
            JsxAnyElement::JsxSelfClosingElement(element) => {
                element.name().ok()?.as_jsx_name()?;
                element.find_attribute_by_name("onClick").ok()??;

                match required_props
                    .iter()
                    .find_map(|key_event| element.find_attribute_by_name(key_event).ok().flatten())
                {
                    Some(_) => None,
                    None => Some(()),
                }
            }
        }
    }

    fn diagnostic(ctx: &RuleContext<Self>, _attr: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();

        Some(RuleDiagnostic::new(
            rule_category!(),
            node.range(),
            markup! {
                "Pair the "<Emphasis>"onClick"</Emphasis>" mouse event with the "<Emphasis>"onKeyUp"</Emphasis>", the "<Emphasis>"onKeyDown"</Emphasis>", or the "<Emphasis>"onKeyPress"</Emphasis>" keyboard event."
            },
        ))
    }

    fn action(_ctx: &RuleContext<Self>, _attr: &Self::State) -> Option<JsRuleAction> {
        None
    }
}
