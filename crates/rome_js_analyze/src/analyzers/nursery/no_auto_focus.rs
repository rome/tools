use crate::JsRuleAction;
use rome_analyze::{context::RuleContext, declare_rule, ActionCategory, Ast, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_diagnostics::Applicability;
use rome_js_syntax::{JsxAttribute, JsxOpeningElement, JsxSelfClosingElement};
use rome_rowan::{declare_node_union, AstNode, BatchMutationExt};

declare_rule! {
    /// Avoid the `autoFocus` attribute
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```jsx,expect_diagnostic
    /// <input autoFocus />
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <input autoFocus="true" />
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <input autoFocus={"false"} />
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <input autoFocus={undefined} />
    /// ```
    ///
    /// ### Valid
    ///
    /// ```jsx
    /// <input />
    ///```
    ///
    /// ```jsx
    /// <div />
    ///```
    ///
    /// ```jsx
    /// <button />
    ///```
    ///
    /// ```jsx
    /// // `autoFocus` prop in user created component is valid
    /// <MyComponent autoFocus={true} />
    ///```
    pub(crate) NoAutoFocus {
        version: "10.0.0",
        name: "noAutofocus",
        recommended: false,
    }
}

declare_node_union! {
    pub(crate) JsxAnyElement = JsxOpeningElement | JsxSelfClosingElement
}

impl Rule for NoAutoFocus {
    type Query = Ast<JsxAnyElement>;
    type State = JsxAttribute;
    type Signals = Option<Self::State>;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        match node {
            JsxAnyElement::JsxOpeningElement(element) => {
                element.name().ok()?.as_jsx_name()?;
                element.find_attribute_by_name("autoFocus").ok()?
            }
            JsxAnyElement::JsxSelfClosingElement(element) => {
                element.name().ok()?.as_jsx_name()?;
                element.find_attribute_by_name("autoFocus").ok()?
            }
        }
    }

    fn diagnostic(_ctx: &RuleContext<Self>, attr: &Self::State) -> Option<RuleDiagnostic> {
        Some(RuleDiagnostic::new(
            rule_category!(),
            attr.syntax().text_trimmed_range(),
            markup! {
                "Avoid the "<Emphasis>"autoFocus"</Emphasis>" attribute."
            },
        ))
    }

    fn action(ctx: &RuleContext<Self>, attr: &Self::State) -> Option<JsRuleAction> {
        let mut mutation = ctx.root().begin();

        mutation.remove_node(attr.clone());

        Some(JsRuleAction {
            category: ActionCategory::QuickFix,
            applicability: Applicability::MaybeIncorrect,
            message: markup! { "Remove the "<Emphasis>"autoFocus"</Emphasis>" attribute." }
                .to_owned(),
            mutation,
        })
    }
}
