use crate::aria_services::Aria;
use rome_analyze::context::RuleContext;
use rome_analyze::{declare_rule, Rule, RuleDiagnostic};
use rome_console::markup;
use rome_js_syntax::jsx_ext::AnyJsxElement;
use rome_rowan::{AstNode, TextRange};
declare_rule! {
    /// Ensure that the attribute passed to the `lang` attribute is a correct ISO language and/or country.
    ///
    /// ## Examples
    ///
    /// ### Invalid
    ///
    /// ```jsx,expect_diagnostic
    /// <html lang="lorem" />
    /// ```
    ///
    /// ```jsx,expect_diagnostic
    /// <html lang="en-babab" />
    /// ```
    ///
    /// ### Valid
    ///
    /// ```jsx
    /// <Html lang="en-babab" />
    /// ```
    pub(crate) UseValidLang {
        version: "12.0.0",
        name: "useValidLang",
        recommended: true,
    }
}

enum ErrorKind {
    InvalidLanguage,
    InvalidCountry,
    InvalidValue,
}

pub(crate) struct UseValidLangState {
    error_kind: ErrorKind,
    attribute_range: TextRange,
}

impl Rule for UseValidLang {
    type Query = Aria<AnyJsxElement>;
    type State = UseValidLangState;
    type Signals = Option<Self::State>;
    type Options = ();

    fn run(ctx: &RuleContext<Self>) -> Self::Signals {
        let node = ctx.query();
        let element_text = node.name().ok()?.as_jsx_name()?.value_token().ok()?;
        if element_text.text_trimmed() == "html" {
            let attribute = node.find_attribute_by_name("lang")?;
            let attribute_value = attribute.initializer()?.value().ok()?;
            let attribute_text = attribute_value.inner_text_value().ok()??;
            let mut split_value = attribute_text.text().split('-');
            match (split_value.next(), split_value.next()) {
                (Some(language), Some(country)) => {
                    if !ctx.is_valid_language(language) {
                        return Some(UseValidLangState {
                            attribute_range: attribute_value.range(),
                            error_kind: ErrorKind::InvalidLanguage,
                        });
                    } else if !ctx.is_valid_country(country) {
                        return Some(UseValidLangState {
                            attribute_range: attribute_value.range(),
                            error_kind: ErrorKind::InvalidCountry,
                        });
                    }
                }

                (Some(language), None) => {
                    if !ctx.is_valid_language(language) {
                        return Some(UseValidLangState {
                            attribute_range: attribute_value.range(),
                            error_kind: ErrorKind::InvalidLanguage,
                        });
                    }
                }
                _ => {
                    if split_value.next().is_some() {
                        return Some(UseValidLangState {
                            attribute_range: attribute_value.range(),
                            error_kind: ErrorKind::InvalidValue,
                        });
                    }
                }
            }
        }

        None
    }

    fn diagnostic(ctx: &RuleContext<Self>, state: &Self::State) -> Option<RuleDiagnostic> {
        let mut diagnostic = RuleDiagnostic::new(
            rule_category!(),
            state.attribute_range,
            markup! {
                "Provide a valid value for the "<Emphasis>"lang"</Emphasis>" attribute."
            },
        );
        diagnostic = match state.error_kind {
            ErrorKind::InvalidLanguage => {
                let languages = ctx.iso_language_list();
                let languages = if languages.len() > 15 {
                    &languages[..15]
                } else {
                    languages
                };

                diagnostic.footer_list("Some of valid languages:", languages)
            }
            ErrorKind::InvalidCountry => {
                let countries = ctx.iso_country_list();
                let countries = if countries.len() > 15 {
                    &countries[..15]
                } else {
                    countries
                };

                diagnostic.footer_list("Some of valid countries:", countries)
            }
            ErrorKind::InvalidValue => diagnostic,
        };y
        Some(diagnostic)
    }
}
