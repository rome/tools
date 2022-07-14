use rome_js_formatter::context::QuoteStyle;
use serde::Deserialize;

#[derive(Default, Debug, Deserialize, Eq, PartialEq)]
pub struct JavascriptConfiguration {
    pub formatter: JavascriptFormatter,
}

#[derive(Default, Debug, Deserialize, Eq, PartialEq)]
#[serde(rename_all = "camelCase", default)]
pub struct JavascriptFormatter {
    /// The style for quotes. Defaults to double.
    #[serde(with = "PlainQuoteStyle")]
    pub quote_style: QuoteStyle,
}

#[derive(Deserialize, Debug, Eq, PartialEq)]
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
