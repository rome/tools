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

    #[serde(skip_serializing_if = "Option::is_none")]
    pub organize_imports: Option<JavascriptOrganizeImports>,
}

impl JavascriptConfiguration {
    pub(crate) const KNOWN_KEYS: &'static [&'static str] =
        &["formatter", "globals", "organizeImports"];
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
    pub quote_style: QuoteStyle,
    /// When properties in objects are quoted. Defaults to asNeeded.
    pub quote_properties: QuoteProperties,
    /// Print trailing commas wherever possible in multi-line comma-separated syntactic structures. Defaults to "all".
    pub trailing_comma: TrailingComma,
    /// Whether the formatter prints semicolons for all statements or only in for statements where it is necessary because of ASI.
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

#[derive(Debug, Default, Deserialize, Serialize, Eq, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(default, deny_unknown_fields)]
pub struct JavascriptOrganizeImports {}
