use rome_analyze::{context::RuleContext, declare_rule, Ast, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_js_syntax::{jsx_ext::JsxAnyElement, JsxAnyAttribute, JsxAnyElementName};
use rome_rowan::AstNode;

declare_rule! {
    /// Enforce to have the `onClick` mouse event with the `onKeyUp`, the `onKeyDown`, or the `onKeyPress` keyboard event.
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
    ///
    /// ```jsx,
    /// <div onClick={() => {}} {...spread}></div>
    /// ```
    ///
    /// ```jsx
    /// <div {...spread} onClick={() => {}} ></div>
    /// ```
    ///
    /// ```jsx
    /// <button onClick={() => console.log("test")}>Submit</button>
    /// ```
    pub(crate) UseKeyWithClickEvents {
        version: "10.0.0",
        name: "useKeyWithClickEvents",
        recommended: true,
    }
}

impl Rule for UseKeyWithClickEvents {
    type Query = Ast<JsxAnyElement>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let element = ctx.query();

        match element.name() {
            Ok(JsxAnyElementName::JsxName(name)) => {
                let element_name = name.value_token().ok()?.text_trimmed().to_lowercase();

                // Don't handle interactive roles
                // TODO Support aria roles https://github.com/rome/tools/issues/3640
                if matches!(
                    element_name.as_str(),
                    "button" | "checkbox" | "combobox" | "a" | "input"
                ) {
                    return None;
                }
            }
            _ => {
                return None;
            }
        }

        let attributes = element.attributes();
        let on_click_attribute = attributes.find_by_name("onClick").ok()?;

        #[allow(clippy::question_mark)]
        if on_click_attribute.is_none() {
            return None;
        }

        for attribute in attributes {
            match attribute {
                JsxAnyAttribute::JsxAttribute(attribute) => {
                    let attribute_name = attribute.name().ok()?;
                    let name = attribute_name.as_jsx_name()?;
                    let name_token = name.value_token().ok()?;

                    if matches!(
                        name_token.text_trimmed(),
                        "onKeyDown" | "onKeyUp" | "onKeyPress"
                    ) {
                        return None;
                    }
                }
                JsxAnyAttribute::JsxSpreadAttribute(_) => {
                    return None;
                }
            }
        }

        Some(())
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
