use crate::aria_services::Aria;
use rome_analyze::{context::RuleContext, declare_rule, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_js_syntax::jsx_ext::AnyJsxElement;
use rome_rowan::{AstNode, AstNodeList};

declare_rule! {
    /// Enforce that elements that do not support ARIA roles, states, and properties do not have those attributes.
    ///
    /// Source: https://github.com/jsx-eslint/eslint-plugin-jsx-a11y/blob/main/docs/rules/aria-unsupported-elements.md
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```jsx,expect_diagnostic
    /// <meta charset="UTF-8" role="meta" />
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <html aria-required="true" />
    /// ```
    ///
    /// ## Valid
    ///
    /// ```jsx
    /// <meta charset="UTF-8" />
    /// ```
    ///
    /// ```jsx
    /// <html></html>
    /// ```
    ///
    ///
    pub(crate) NoAriaUnsupportedElements {
        version: "next",
        name: "noAriaUnsupportedElements",
        recommended: true,
    }
}

impl Rule for NoAriaUnsupportedElements {
    type Query = Aria<AnyJsxElement>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let aria_properties = ctx.aria_properties();

        let element_name = node.name().ok()?.as_jsx_name()?.value_token().ok()?;
        let element_name = element_name.text_trimmed();
        let aria_unsuppurted_elements = ["meta", "html", "script", "style"];

        if aria_unsuppurted_elements.contains(&element_name) {
            // Check if the unsupported element has `role` or `aria-*` attribute
            let attributes: Vec<_> = node
                .attributes()
                .iter()
                .filter_map(|attribute| {
                    let attribute = attribute.as_jsx_attribute()?;
                    let attribute_name =
                        attribute.name().ok()?.as_jsx_name()?.value_token().ok()?;

                    if attribute_name.text_trimmed().starts_with("aria-")
                        && aria_properties
                            .get_property(attribute_name.text_trimmed())
                            .is_some()
                    {
                        return Some(());
                    }
                    if attribute_name.text_trimmed() == "role" {
                        return Some(());
                    }
                    None
                })
                .collect();

            if !attributes.is_empty() {
                return Some(());
            }
        }

        None
    }

    fn diagnostic(ctx: &RuleContext<Self>, _: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.range(),
                markup! {
                    "Avoid the "<Emphasis>"role"</Emphasis>" attribute and "<Emphasis>"aria-*"</Emphasis>" attributes when using "<Emphasis>"meta"</Emphasis>", "<Emphasis>"html"</Emphasis>", "<Emphasis>"script"</Emphasis>", and "<Emphasis>"style"</Emphasis>" elements."
                },
            )
            .note(markup! {
                "Using roles on elements that do not support them can cause issues with screen readers."
            }),
        )
    }
}
