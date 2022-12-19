use rome_analyze::{context::RuleContext, declare_rule, Ast, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_js_syntax::{jsx_ext::AnyJsxElement, AnyJsExpression, AnyJsxAttributeValue};
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

        let Some(title_attribute) = node.find_attribute_by_name("title") else {
            return Some(UseIframeTitleState { node: node.clone() })
        };

        let attribute_value = title_attribute.initializer()?.value().ok()?;

        match attribute_value {
            AnyJsxAttributeValue::JsxString(str) => {
                let text = str.inner_string_text().ok()?;
                let is_valid_string = !text.is_empty() && text != r#"``"#;
                if is_valid_string {
                    return None;
                }
                Some(UseIframeTitleState { node: node.clone() })
            }
            AnyJsxAttributeValue::JsxExpressionAttributeValue(expr_attribute_value) => {
                let expr = expr_attribute_value.expression().ok()?;
                if let AnyJsExpression::JsIdentifierExpression(identifier) = expr {
                    let text = identifier.name().ok()?.value_token().ok()?;
                    let is_undefined_or_null =
                        text.text_trimmed() == "undefined" || text.text_trimmed() == "null";
                    if is_undefined_or_null {
                        return Some(UseIframeTitleState { node: node.clone() });
                    } else {
                        // we assueme the identifier is a string type
                        return None;
                    }
                }
                Some(UseIframeTitleState { node: node.clone() })
            }
            AnyJsxAttributeValue::AnyJsxTag(_) => Some(UseIframeTitleState { node: node.clone() }),
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
