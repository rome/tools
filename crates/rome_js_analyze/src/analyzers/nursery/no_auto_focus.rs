use crate::JsRuleAction;
use rome_analyze::{context::RuleContext, declare_rule, ActionCategory, Ast, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_diagnostics::Applicability;
use rome_js_syntax::{JsAnyExpression, JsxAttribute, JsxSelfClosingElement};
use rome_rowan::{AstNode, BatchMutationExt};

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
    /// ### Valid
    ///
    /// ```jsx
    /// <input />
    ///```
    ///
    /// ```jsx
    /// <input autoFocus={undefined} />
    ///```
    ///
    /// ```jsx
    /// <Input autoFocus={"false"} />
    ///```
    pub(crate) NoAutoFocus {
        version: "10.0.0",
        name: "noAutofocus",
        recommended: false,
    }
}

impl Rule for NoAutoFocus {
    type Query = Ast<JsxSelfClosingElement>;
    type State = JsxAttribute;
    type Signals = Option<Self::State>;

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let name = node.name().ok()?.syntax().text_trimmed();

        if name == "input" {
            let attribute = node.find_attribute_by_name("autoFocus").ok()??;

            if let Some(value) = get_attribute_value(&attribute) {
                if value.syntax().text_trimmed() == "undefined" {
                    return None;
                }
            }

            return Some(attribute);
        }

        None
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

fn get_attribute_value(attribute: &JsxAttribute) -> Option<JsAnyExpression> {
    attribute
        .initializer()?
        .value()
        .ok()?
        .as_jsx_expression_attribute_value()?
        .expression()
        .ok()
}
