use crate::aria_services::Aria;
use rome_analyze::context::RuleContext;
use rome_analyze::{declare_rule, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_js_syntax::{JsSyntaxToken, JsxAttribute, TextRange};
use rome_rowan::AstNode;

declare_rule! {
    /// Enforce that ARIA state and property values are valid.
    ///
    pub(crate) UseAriaPropTypes {
        version: "11.0.0",
        name: "useAriaPropTypes",
        recommended: false,
    }
}

pub(crate) struct UseAriaProptypesState {
    attribute_value_range: TextRange,
    allowed_values: Vec<String>,
    attribute_name: JsSyntaxToken,
}

impl Rule for UseAriaPropTypes {
    type Query = Aria<JsxAttribute>;
    type State = UseAriaProptypesState;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let aria_properties = ctx.aria_properties();

        let attribute_name = node.name().ok()?.as_jsx_name()?.value_token().ok()?;

        if let Some(aria_property) = aria_properties.get_property(attribute_name.text_trimmed()) {
            let attribute_value = node.initializer()?.value().ok()?;
            let attribute_value = attribute_value.as_jsx_string()?;

            let attribute_text = attribute_value.inner_string_text().ok()?;
            if !aria_property.contains_correct_value(attribute_text.text()) {
                return Some(UseAriaProptypesState {
                    attribute_value_range: attribute_value.range(),
                    allowed_values: aria_property
                        .values()
                        .map(|value| value.to_string())
                        .collect::<Vec<_>>(),
                    attribute_name,
                });
            }
        }

        None
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let attribute_name = state.attribute_name.text_trimmed();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                state.attribute_value_range,
                markup! {
                "The value of the ARIA attribute "<Emphasis>{attribute_name}</Emphasis>" is not correct."
            },
            ).footer_list(
        markup!{
                    "The supported values for the "<Emphasis>{attribute_name}</Emphasis>" attribute are:"
                },
            &state.allowed_values
            )
        )
    }
}
