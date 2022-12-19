use rome_analyze::{context::RuleContext, declare_rule, Ast, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_js_syntax::{jsx_ext::AnyJsxElement, AnyJsExpression};
use rome_rowan::{AstNode, AstNodeList};

declare_rule! {
    /// Enforces the usage of the attribute `title` for the element `iframe`
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```jsx,expect_diagnostic
    ///     <iframe />
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    ///     <iframe></iframe>
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    ///     <iframe {...props} />
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    ///     <iframe title="" />
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    ///     <iframe title={""} />
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    ///     <iframe title={<span className={"token string"}></span>}></iframe>
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    ///     <iframe title={undefined} />
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    ///     <iframe title={false} />
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    ///     <iframe title={true} />
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    ///     <iframe title={42} />
    /// ```
    ///
    ///
    /// ### Valid
    ///
    /// ```jsx
    ///     <>
    ///         <iframe title="This is a unique title" />
    ///         <iframe title={uniqueTitle} />
    ///     </>
    /// ```
    ///
    /// ## Accessibility guidelines
    ///
    /// [WCAG 2.4.1](https://www.w3.org/WAI/WCAG21/Understanding/bypass-blocks)
    /// [WCAG 4.1.2](https://www.w3.org/WAI/WCAG21/Understanding/name-role-value)
    ///
    pub(crate) UseIframeTitle {
        version: "12.0.0",
        name: "useIframeTitle",
        recommended: true,
    }
}

pub(crate) struct UseIframeTitleState {
    node: AnyJsxElement,
}

impl Rule for UseIframeTitle {
    type Query = Ast<AnyJsxElement>;
    type State = UseIframeTitleState;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();

        if node.name_value_token()?.text_trimmed() != "iframe" {
            return None;
        }

        if node.attributes().is_empty() {
            return Some(UseIframeTitleState { node: node.clone() });
        }

        let title_attribute = node.find_attribute_by_name("title");

        if let Some(title_attribute) = title_attribute {
            let attribute_value = title_attribute.initializer()?.value().ok()?;
            match attribute_value.as_jsx_string() {
                Some(text_value) => {
                    // the title attribute is a string
                    let text = text_value.inner_string_text().ok()?;
                    if text.is_empty() || text == r#"``"# {
                        return Some(UseIframeTitleState { node: node.clone() });
                    }
                    None
                }
                None => {
                    // the title attribute is not a string
                    let expression = attribute_value
                        .as_jsx_expression_attribute_value()?
                        .expression()
                        .ok()?;

                    if let AnyJsExpression::JsIdentifierExpression(identifier) = expression {
                        let text = identifier.name().ok()?.value_token().ok()?;
                        if text.text_trimmed() == "undefined" || text.text_trimmed() == "null" {
                            return Some(UseIframeTitleState { node: node.clone() });
                        } else {
                            // we assueme the identifier is a string type
                            return None;
                        }
                    }

                    Some(UseIframeTitleState { node: node.clone() })
                }
            }
        } else {
            // the iframe has some attributes but no `title` attribute. (e.g. <iframe {...props} />)
            Some(UseIframeTitleState { node: node.clone() })
        }
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(
            RuleDiagnostic::new(
                rule_category!(),
                state.node.syntax().text_trimmed_range(),
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
