use rome_js_formatter::context::QuoteStyle;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Deserialize, Serialize, Eq, PartialEq)]
pub struct JavascriptConfiguration {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub formatter: Option<JavascriptFormatter>,
}

#[derive(Default, Debug, Deserialize, Serialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase", default)]
pub struct JavascriptFormatter {
    /// The style for quotes. Defaults to double.
    #[serde(with = "PlainQuoteStyle")]
    pub quote_style: QuoteStyle,
}

#[derive(Deserialize, Serialize, Debug, Eq, PartialEq)]
#[serde(rename_all = "camelCase", remote = "QuoteStyle")]
pub enum PlainQuoteStyle {
    Double,
    Single,
}

impl Default for PlainQuoteStyle {
    fn default() -> Self {
        Self::Double
    }
}
