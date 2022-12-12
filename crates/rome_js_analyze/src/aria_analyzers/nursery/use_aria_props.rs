use crate::aria_services::Aria;
use rome_analyze::context::RuleContext;
use rome_analyze::{declare_rule, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_js_syntax::jsx_ext::AnyJsxElement;
use rome_js_syntax::JsxAttribute;
use rome_rowan::AstNode;
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
    type Query = Aria<JsxAttribute>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let aria_properties = ctx.aria_properties();
        let parent_element = node.syntax().ancestors().find_map(AnyJsxElement::cast)?;

        // check attributes that belong only to HTML elements
        if parent_element.is_element() {
            let attribute_name = node.name().ok()?.as_jsx_name()?.value_token().ok()?;
            if attribute_name.text_trimmed().starts_with("aria-")
                && aria_properties
                    .get_property(attribute_name.text_trimmed())
                    .is_none()
            {
                return Some(());
            }
        }

        None
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        let attribute_name = node.name().ok()?.as_jsx_name()?.value_token().ok()?;
        Some(RuleDiagnostic::new(
            rule_category!(),
            node.range(),
            markup! {
                <Emphasis>{{attribute_name.text_trimmed()}}</Emphasis>" is not a valid ARIA attribute."
            },
        ))
    }
}
