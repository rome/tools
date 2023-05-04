use rome_analyze::{context::RuleContext, declare_rule, Ast, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_js_syntax::{jsx_ext::AnyJsxElement, AnyJsExpression, AnyJsxAttributeValue};
use rome_rowan::{AstNode, AstNodeList};

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
        let node = ctx.query();

        if node.name_value_token()?.text_trimmed() == "iframe" && has_valid_title(element) {
            return Some(());
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

fn has_valid_title(element: &AnyJsxElement) -> bool {
    element
        .find_attribute_by_name("title")
        .map_or(false, |attribute| {
            if attribute.initializer().is_none() {
                return false;
            }

            attribute.as_static_value().map_or(true, |value| {
                !value.is_null_or_undefined() && value.is_not_string_constant("")
            }) && !element.has_trailing_spread_prop(attribute)
        })
}
