use indexmap::IndexSet;
use rome_js_formatter::context::{trailing_comma::TrailingComma, QuoteProperties, QuoteStyle};
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
}

#[derive(Deserialize, Serialize, Debug, Eq, PartialEq, Default)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", remote = "QuoteStyle")]
pub enum PlainQuoteStyle {
    #[default]
    Double,
    Single,
}

#[derive(Deserialize, Default, Serialize, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", remote = "QuoteProperties")]
pub enum PlainQuoteProperties {
    #[default]
    AsNeeded,
    Preserve,
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
