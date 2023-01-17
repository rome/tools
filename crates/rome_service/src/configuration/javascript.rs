use indexmap::IndexSet;
use rome_js_formatter::context::{
    trailing_comma::TrailingComma, QuoteProperties, QuoteStyle, Semicolons,
};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(default, deny_unknown_fields)]
pub struct JavascriptConfiguration {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub formatter: Option<JavascriptFormatter>,

    /// A list of global bindings that should be ignored by the analyzers
    ///
    /// If defined here, they should not emit diagnostics.
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::deserialize_set_of_strings",
        serialize_with = "crate::serialize_set_of_strings"
    )]
    pub globals: Option<IndexSet<String>>,
}

impl JavascriptConfiguration {
    pub(crate) const KNOWN_KEYS: &'static [&'static str] = &["formatter", "globals"];
}

impl JavascriptConfiguration {
    pub fn with_formatter() -> Self {
        Self {
            formatter: Some(JavascriptFormatter::default()),
            ..JavascriptConfiguration::default()
        }
    }
}

#[derive(Default, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct JavascriptFormatter {
    /// The style for quotes. Defaults to double.
    #[serde(with = "PlainQuoteStyle")]
    pub quote_style: QuoteStyle,
    /// When properties in objects are quoted. Defaults to asNeeded.
    #[serde(with = "PlainQuoteProperties")]
    pub quote_properties: QuoteProperties,
    /// Print trailing commas wherever possible in multi-line comma-separated syntactic structures. Defaults to "all".
    #[serde(with = "PlainTrailingComma")]
    pub trailing_comma: TrailingComma,
    /// Whether the formatter prints semicolons for all statements or only in for statements where it is necessary because of ASI.
    #[serde(with = "PlainSemicolons")]
    pub semicolons: Semicolons,
}

impl JavascriptFormatter {
    pub(crate) const KNOWN_KEYS: &'static [&'static str] = &[
        "quoteStyle",
        "quoteProperties",
        "trailingComma",
        "semicolons",
    ];
}

#[derive(Deserialize, Serialize, Debug, Eq, PartialEq, Default)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", remote = "QuoteStyle")]
pub enum PlainQuoteStyle {
    #[default]
    Double,
    Single,
}

impl From<PlainQuoteStyle> for QuoteStyle {
    fn from(variant: PlainQuoteStyle) -> Self {
        match variant {
            PlainQuoteStyle::Double => QuoteStyle::Double,
            PlainQuoteStyle::Single => QuoteStyle::Single,
        }
    }
}

impl PlainQuoteStyle {
    pub(crate) const KNOWN_VALUES: &'static [&'static str] = &["double", "single"];
}

#[derive(Deserialize, Default, Serialize, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", remote = "QuoteProperties")]
pub enum PlainQuoteProperties {
    #[default]
    AsNeeded,
    Preserve,
}

impl From<PlainQuoteProperties> for QuoteProperties {
    fn from(variant: PlainQuoteProperties) -> Self {
        match variant {
            PlainQuoteProperties::AsNeeded => QuoteProperties::AsNeeded,
            PlainQuoteProperties::Preserve => QuoteProperties::Preserve,
        }
    }
}

impl PlainQuoteProperties {
    pub(crate) const KNOWN_VALUES: &'static [&'static str] = &["preserve", "asNeeded"];
}

#[derive(Deserialize, Default, Serialize, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "lowercase", remote = "TrailingComma")]
pub enum PlainTrailingComma {
    #[default]
    All,
    ES5,
    None,
}

impl From<PlainTrailingComma> for TrailingComma {
    fn from(variant: PlainTrailingComma) -> Self {
        match variant {
            PlainTrailingComma::All => TrailingComma::All,
            PlainTrailingComma::ES5 => TrailingComma::ES5,
            PlainTrailingComma::None => TrailingComma::None,
        }
    }
}

impl PlainTrailingComma {
    pub(crate) const KNOWN_VALUES: &'static [&'static str] = &["all", "es5", "none"];
}

#[derive(Deserialize, Default, Serialize, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", remote = "Semicolons")]
pub enum PlainSemicolons {
    #[default]
    Always,
    AsNeeded,
}

impl From<PlainSemicolons> for Semicolons {
    fn from(variant: PlainSemicolons) -> Self {
        match variant {
            PlainSemicolons::Always => Semicolons::Always,
            PlainSemicolons::AsNeeded => Semicolons::AsNeeded,
        }
    }
}

impl PlainSemicolons {
    pub(crate) const KNOWN_VALUES: &'static [&'static str] = &["always", "asNeeded"];
}
