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
    pub quote_style: PlainQuoteStyle,
}

#[derive(Deserialize, Debug, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum PlainQuoteStyle {
    Double,
    Single,
}

impl Default for PlainQuoteStyle {
    fn default() -> Self {
        Self::Double
    }
}

impl From<JavascriptConfiguration> for QuoteStyle {
    fn from(j: JavascriptConfiguration) -> Self {
        match j.formatter.quote_style {
            PlainQuoteStyle::Double => QuoteStyle::Double,
            PlainQuoteStyle::Single => QuoteStyle::Single,
        }
    }
}
