use crate::JsRuleAction;
use rome_analyze::{context::RuleContext, declare_rule, ActionCategory, Ast, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_diagnostics::Applicability;
use rome_js_syntax::{JsxAttribute, JsxSelfClosingElement, JsxSelfClosingElementFields};
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
        let JsxSelfClosingElementFields {
            l_angle_token: _,
            name,
            type_arguments: _,
            attributes: _,
            slash_token: _,
            r_angle_token: _,
        } = node.as_fields();

        if name.ok()?.text().trim() == "input" {
            let attribute = node.find_attribute_by_name("autoFocus").ok()??;
            match attribute.initializer() {
                Some(initializer) => match initializer.value().ok() {
                    Some(value) => {
                        let value = match value.as_jsx_expression_attribute_value() {
                            Some(value) => value,
                            None => return Some(attribute),
                        };
                        let value = match value.expression().ok() {
                            Some(value) => value.text(),
                            None => return Some(attribute),
                        };
                        if value.trim() == "undefined" {
                            return None;
                        } else {
                            return Some(attribute);
                        }
                    }
                    None => {
                        return Some(attribute);
                    }
                },
                None => {
                    return Some(attribute);
                }
            }
        }

        None
    }

    fn diagnostic(_ctx: &RuleContext<Self>, attr: &Self::State) -> Option<RuleDiagnostic> {
        Some(RuleDiagnostic::new(
            rule_category!(),
            attr.range(),
            markup! {
                "Avoid the "<Emphasis>"autoFocus"</Emphasis>" attribute."
            }
            .to_owned(),
        ))
    }

    fn action(ctx: &RuleContext<Self>, attr: &Self::State) -> Option<JsRuleAction> {
        let mut mutation = ctx.root().begin();

        mutation.remove_node(attr.to_owned());

        Some(JsRuleAction {
            category: ActionCategory::QuickFix,
            applicability: Applicability::MaybeIncorrect,
            message: markup! { "Remove the "<Emphasis>"autoFocus"</Emphasis>" attribute." }
                .to_owned(),
            mutation,
        })
    }
}
