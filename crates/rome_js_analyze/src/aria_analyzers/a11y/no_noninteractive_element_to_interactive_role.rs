use crate::aria_services::Aria;
use rome_analyze::context::RuleContext;
use rome_analyze::{declare_rule, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_js_syntax::jsx_ext::AnyJsxElement;
use rome_rowan::{AstNode, TextRange};

declare_rule! {
    /// Enforce that interactive ARIA roles are not assigned to non-interactive HTML elements.
    ///
    /// Non-interactive HTML elements indicate _content_ and _containers_ in the user interface.
    /// Non-interactive elements include `<main>`, `<area>`, `<h1>` (,`<h2>`, etc), `<img>`, `<li>`, `<ul>` and `<ol>`.
    ///
    /// Interactive HTML elements indicate _controls_ in the user interface.
    /// Interactive elements include `<a href>`, `<button>`, `<input>`, `<select>`, `<textarea>`.
    ///
    /// [WAI-ARIA roles](https://www.w3.org/TR/wai-aria-1.1/#usage_intro) should not be used to convert a non-interactive element to an interactive element.
    /// Interactive ARIA roles include `button`, `link`, `checkbox`, `menuitem`, `menuitemcheckbox`, `menuitemradio`, `option`, `radio`, `searchbox`, `switch` and `textbox`.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```jsx,expect_diagnostic
    /// <h1 role="button">Some text</h1>
    /// ```
    ///
    /// ### Valid
    ///
    ///
    /// ```jsx
    /// <span role="button">Some text</span>
    /// ```
    ///
    /// ## Accessibility guidelines
    ///
    /// - [WCAG 4.1.2](https://www.w3.org/WAI/WCAG21/Understanding/name-role-value)
    ///
    /// ### Resources
    ///
    /// - [WAI-ARIA roles](https://www.w3.org/TR/wai-aria-1.1/#usage_intro)
    /// - [WAI-ARIA Authoring Practices Guide - Design Patterns and Widgets](https://www.w3.org/TR/wai-aria-practices-1.1/#aria_ex)
    /// - [Fundamental Keyboard Navigation Conventions](https://www.w3.org/TR/wai-aria-practices-1.1/#kbd_generalnav)
    /// - [Mozilla Developer Network - ARIA Techniques](https://developer.mozilla.org/en-US/docs/Web/Accessibility/ARIA/ARIA_Techniques/Using_the_button_role#Keyboard_and_focus)
    ///
    pub(crate) NoNoninteractiveElementToInteractiveRole {
        version: "12.0.0",
        name: "noNoninteractiveElementToInteractiveRole",
        recommended: true,
    }
}

pub(crate) struct RuleState {
    attribute_range: TextRange,
    element_name: String,
}

impl Rule for NoNoninteractiveElementToInteractiveRole {
    type Query = Aria<AnyJsxElement>;
    type State = RuleState;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let aria_roles = ctx.aria_roles();

        if node.is_element() {
            let role_attribute = node.find_attribute_by_name("role")?;
            let role_attribute_static_value = role_attribute.as_static_value()?;
            let role_attribute_value = role_attribute_static_value.text();
            let element_name = node.name().ok()?.as_jsx_name()?.value_token().ok()?;

            let attributes = ctx.extract_attributes(&node.attributes());
            if aria_roles.is_not_interactive_element(element_name.text_trimmed(), attributes)
                && aria_roles.is_role_interactive(role_attribute_value)
            {
                // <div> and <span> are considered neither interactive nor non-interactive, depending on the presence or absence of the role attribute.
                // We don't report <div> and <span> here, because we cannot determine whether they are interactive or non-interactive.
                let role_sensitive_elements = ["div", "span"];
                if role_sensitive_elements.contains(&element_name.text_trimmed()) {
                    return None;
                }

                return Some(RuleState {
                    attribute_range: role_attribute.range(),
                    element_name: element_name.text_trimmed().to_string(),
                });
            }
        }
        None
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(RuleDiagnostic::new(
            rule_category!(),
            state.attribute_range,
            markup! {
                "The HTML element "<Emphasis>{{&state.element_name}}</Emphasis>" is non-interactive and should not have an interactive role."
            },
        ).note(
            markup!{
                "Replace "<Emphasis>{{&state.element_name}}</Emphasis>" with a div or a span."
            }
        ))
    }
}
