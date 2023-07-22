use rome_analyze::{context::RuleContext, declare_rule, Ast, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_js_syntax::jsx_ext::AnyJsxElement;
use rome_rowan::AstNode;

declare_rule! {
    /// Enforces the usage of the attribute `title` for the element `iframe`.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```jsx,expect_diagnostic
    ///  <iframe />
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <iframe></iframe>
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <iframe title="" />
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <iframe title={""} />
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <iframe title={undefined} />
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <iframe title={false} />
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <iframe title={true} />
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <iframe title={42} />
    /// ```
    ///
    ///
    /// ### Valid
    ///
    /// ```jsx
    /// <>
    ///   <iframe title="This is a unique title" />
    ///   <iframe title={uniqueTitle} />
    ///   <iframe {...props} />
    /// </>
    /// ```
    ///
    /// ## Accessibility guidelines
    ///
    /// - [WCAG 2.4.1](https://www.w3.org/WAI/WCAG21/Understanding/bypass-blocks)
    /// - [WCAG 4.1.2](https://www.w3.org/WAI/WCAG21/Understanding/name-role-value)
    ///
    pub(crate) UseIframeTitle {
        version: "12.0.0",
        name: "useIframeTitle",
        recommended: true,
    }
}

impl Rule for UseIframeTitle {
    type Query = Ast<AnyJsxElement>;
    type State = ();
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let element = ctx.query();
        let name = element.name().ok()?.name_value_token()?;

        if name.text_trimmed() == "iframe" {
            if let Some(lang_attribute) = element.find_attribute_by_name("title") {
                if !lang_attribute.as_static_value().map_or(true, |attribute| {
                    !attribute
                        .as_string_constant()
                        .unwrap_or_default()
                        .is_empty()
                }) && !element.has_trailing_spread_prop(lang_attribute)
                {
                    return Some(());
                }
            } else if !element.has_spread_prop() {
                return Some(());
            }
        }

        None
    }

    fn diagnostic(ctx: &RuleContext<Self>, _state: &Self::State) -> Option<RuleDiagnostic> {
        let node = ctx.query();
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                node.syntax().text_trimmed_range(),
                markup! {
                "Provide a "<Emphasis>"title"</Emphasis>" attribute when using "<Emphasis>"iframe"</Emphasis>" elements."
            }
            )
            .note(markup! {
                "Screen readers rely on the title set on an iframe to describe the content being displayed."
            }),
        )
    }
}
