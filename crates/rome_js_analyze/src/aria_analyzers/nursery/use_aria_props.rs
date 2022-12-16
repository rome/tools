use crate::aria_services::Aria;
use rome_analyze::context::RuleContext;
use rome_analyze::{declare_rule, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_js_syntax::jsx_ext::AnyJsxElement;
use rome_rowan::{AstNode, TextRange};

declare_rule! {
    /// Ensures that ARIA properties `aria-*` are all valid.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```jsx, expect_diagnostic
    /// <input className="" aria-labell="" />
    /// ```
    ///
    /// ## Accessibility guidelines
    /// - [WCAG 4.1.2](https://www.w3.org/WAI/WCAG21/Understanding/name-role-value)
    pub(crate) UseAriaProps {
        version: "12.0.0",
        name: "useAriaProps",
        recommended: true,
    }
}

impl Rule for UseAriaProps {
    type Query = Aria<AnyJsxElement>;
    type State = (TextRange, String);
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let aria_properties = ctx.aria_properties();

        // check attributes that belong only to HTML elements
        if node.is_element() {
            for attribute in node.attributes() {
                let attribute = attribute.as_jsx_attribute()?;
                let attribute_name = attribute.name().ok()?.as_jsx_name()?.value_token().ok()?;
                if attribute_name.text_trimmed().starts_with("aria-")
                    && aria_properties
                        .get_property(attribute_name.text_trimmed())
                        .is_none()
                {
                    return Some((attribute.range(), attribute_name.to_string()));
                }
            }
        }

        None
    }

    fn diagnostic(_ctx: &RuleContext<Self>, (range, name): &Self::State) -> Option<RuleDiagnostic> {
        Some(RuleDiagnostic::new(
            rule_category!(),
            range,
            markup! {
                <Emphasis>{name}</Emphasis>" is not a valid ARIA attribute."
            },
        ))
    }
}
