use rome_analyze::{context::RuleContext, declare_rule, Ast, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_js_syntax::{
    AnyJsxAttribute, AnyJsxAttributeValue, JsxAttribute, JsxOpeningElement, TextRange,
};
use rome_rowan::AstNode;

declare_rule! {
    /// Enforce that `html` element has `lang` attribute.
    /// `html` element must have a valid `lang` attribute or correspond to a valid language code
    /// in order to provide a language preference for multilingual screen reader users.
    /// This allows users to choose a language other than the default.
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```jsx,expect_diagnostic
    /// <html></html>
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <html lang={null}></html>
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <html lang={undefined}></html>
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <html lang={true}></html>
    /// ```
    ///
    /// ### Valid
    ///
    /// ```jsx
    /// <html lang="en"></html>
    /// ```
    ///
    /// ```jsx
    /// <html lang={language}></html>
    /// ```
    ///
    /// ```jsx
    /// <html {...props}></html>
    /// ```
    ///
    /// ## Accessibility guidelines
    ///
    /// [WCAG 3.1.1](https://www.w3.org/WAI/WCAG21/Understanding/language-of-page)
    pub(crate) UseHtmlLang {
        version: "12.0.0",
        name: "useHtmlLang",
        recommended: true,
    }
}

impl Rule for UseHtmlLang {
    type Query = Ast<JsxOpeningElement>;
    type State = TextRange;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let element = ctx.query();
        let name = element.name().ok()?;
        let name = name.as_jsx_name()?.value_token().ok()?;
        let name_trimmed = name.text_trimmed();
        if name_trimmed == "html" {
            if let Some(lang_attribute) = element.find_attribute_by_name("lang").ok()? {
                if element.has_trailing_spread_prop(lang_attribute.clone())
                    || is_valid_lang_attribute(lang_attribute).is_some()
                {
                    return None;
                }
                return Some(element.syntax().text_trimmed_range());
            }
            if !has_spread_prop(element) {
                return Some(element.syntax().text_trimmed_range());
            }
        }
        None
    }

    fn diagnostic(_ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        Some(RuleDiagnostic::new(
			rule_category!(),
            state,
            markup! {
				"Provide a "<Emphasis>"lang"</Emphasis>" attribute when using the "<Emphasis>"html"</Emphasis>" element."
			}
        ).note(
			markup! {
				"Setting a "<Emphasis>"lang"</Emphasis>" attribute on HTML document elements configures the language
used by screen readers when no user default is specified."
			}
		))
    }
}

fn is_valid_lang_attribute(attr: JsxAttribute) -> Option<bool> {
    if attr.is_value_undefined_or_null() {
        return None;
    }

    let attribute_value = attr.initializer()?.value().ok()?;

    if let AnyJsxAttributeValue::JsxExpressionAttributeValue(expression) = attribute_value {
        let expression = expression.expression().ok()?;

        if let Some(identifier_expression) = expression.as_js_identifier_expression() {
            if !identifier_expression.text().is_empty() {
                return Some(true);
            }
            return None;
        }

        let bool_expression = expression
            .as_any_js_literal_expression()?
            .as_js_boolean_literal_expression();
        if bool_expression.is_some() {
            return None;
        }

        let string_expression = expression
            .as_any_js_literal_expression()?
            .as_js_string_literal_expression()?;
        let string_expression_text = string_expression.inner_string_text().ok()?;
        if string_expression_text.is_empty() {
            return None;
        }

        return Some(true);
    }
    let string_text = attribute_value.as_jsx_string()?.inner_string_text().ok()?;
    if string_text.is_empty() {
        return None;
    }

    Some(true)
}

fn has_spread_prop(element: &JsxOpeningElement) -> bool {
    element
        .attributes()
        .into_iter()
        .any(|attribute| matches!(attribute, AnyJsxAttribute::JsxSpreadAttribute(_)))
}
