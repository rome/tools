use crate::configuration::merge::MergeWith;
use crate::configuration::string_set::StringSet;
use bpaf::Bpaf;
use rome_js_formatter::context::{
    trailing_comma::TrailingComma, QuoteProperties, QuoteStyle, Semicolons,
};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Deserialize, Serialize, Eq, PartialEq, Clone, Bpaf)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(default, deny_unknown_fields)]
pub struct JavascriptConfiguration {
    #[serde(skip_serializing_if = "Option::is_none")]
    #[bpaf(external(javascript_formatter), optional)]
    pub formatter: Option<JavascriptFormatter>,

    /// A list of global bindings that should be ignored by the analyzers
    ///
    /// If defined here, they should not emit diagnostics.
    #[serde(skip_serializing_if = "Option::is_none")]
    #[bpaf(hide)]
    pub globals: Option<StringSet>,
    //
    #[serde(skip_serializing_if = "Option::is_none")]
    #[bpaf(external(javascript_organize_imports), optional)]
    pub organize_imports: Option<JavascriptOrganizeImports>,
}

impl MergeWith<JavascriptConfiguration> for JavascriptConfiguration {
    fn merge_with(&mut self, other: JavascriptConfiguration) {
        if let Some(other_formatter) = other.formatter {
            let formatter = self
                .formatter
                .get_or_insert_with(JavascriptFormatter::default);
            formatter.merge_with(other_formatter);
        }
    }
}

impl MergeWith<Option<JavascriptFormatter>> for JavascriptConfiguration {
    fn merge_with(&mut self, other: Option<JavascriptFormatter>) {
        if let Some(other_formatter) = other {
            let formatter = self
                .formatter
                .get_or_insert_with(JavascriptFormatter::default);
            formatter.merge_with(other_formatter);
        }
    }
}

impl JavascriptConfiguration {
    pub(crate) const KNOWN_KEYS: &'static [&'static str] =
        &["formatter", "globals", "organizeImports"];

    pub fn with_formatter() -> Self {
        Self {
            formatter: Some(JavascriptFormatter::default()),
            ..JavascriptConfiguration::default()
        }
    }
}

#[derive(Default, Debug, Deserialize, Serialize, Eq, PartialEq, Clone, Bpaf)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct JavascriptFormatter {
    /// The style for quotes. Defaults to double.
    #[bpaf(long("quote-style"), argument("double|single"), optional)]
    pub quote_style: Option<QuoteStyle>,
    /// The style for JSX quotes. Defaults to double.
    #[bpaf(long("jsx-quote-style"), argument("double|single"), optional)]
    pub jsx_quote_style: Option<QuoteStyle>,
    /// When properties in objects are quoted. Defaults to asNeeded.
    #[bpaf(long("quote-properties"), argument("preserve|as-needed"), optional)]
    pub quote_properties: Option<QuoteProperties>,
    /// Print trailing commas wherever possible in multi-line comma-separated syntactic structures. Defaults to "all".
    #[bpaf(long("trailing-comma"), argument("all|es5|none"), optional)]
    pub trailing_comma: Option<TrailingComma>,
    /// Whether the formatter prints semicolons for all statements or only in for statements where it is necessary because of ASI.
    #[bpaf(long("semicolons"), argument("always|as-needed"), optional)]
    pub semicolons: Option<Semicolons>,
}

impl JavascriptFormatter {
    pub(crate) const KNOWN_KEYS: &'static [&'static str] = &[
        "quoteStyle",
        "jsxQuoteStyle",
        "quoteProperties",
        "trailingComma",
        "semicolons",
    ];
}

impl MergeWith<JavascriptFormatter> for JavascriptFormatter {
    fn merge_with(&mut self, other: JavascriptFormatter) {
        if let Some(quote_properties) = other.quote_properties {
            self.quote_properties = Some(quote_properties);
        }
        if let Some(quote_style) = other.quote_style {
            self.quote_style = Some(quote_style);
        }
        if let Some(jsx_quote_style) = other.jsx_quote_style {
            self.jsx_quote_style = Some(jsx_quote_style);
        }
        if let Some(semicolons) = other.semicolons {
            self.semicolons = Some(semicolons);
        }
        if let Some(trailing_comma) = other.trailing_comma {
            self.trailing_comma = Some(trailing_comma);
        }
    }
}

#[derive(Debug, Default, Deserialize, Serialize, Eq, PartialEq, Clone, Bpaf)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(default, deny_unknown_fields)]
pub struct JavascriptOrganizeImports {}
