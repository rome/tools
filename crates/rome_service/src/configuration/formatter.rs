use crate::settings::FormatSettings;
use rome_formatter::{IndentStyle, LineWidth};
use serde::Deserialize;

#[derive(Deserialize, Debug, Eq, PartialEq)]
#[serde(rename_all = "camelCase", default)]
pub struct FormatterConfiguration {
    // if `false`, it disables the feature. `true` by default
    pub enabled: bool,

    /// Stores whether formatting should be allowed to proceed if a given file
    /// has syntax errors
    pub format_with_errors: bool,

    /// The indent style.
    pub indent_style: PlainIndentStyle,

    /// The size of the indentation, 2 by default
    indent_size: u8,

    /// What's the max width of a line. Defaults to 80.
    #[serde(deserialize_with = "deserialize_line_width")]
    pub line_width: LineWidth,
}

impl Default for FormatterConfiguration {
    fn default() -> Self {
        Self {
            enabled: true,
            format_with_errors: false,
            indent_size: 2,
            indent_style: PlainIndentStyle::default(),
            line_width: LineWidth::default(),
        }
    }
}

impl From<&FormatterConfiguration> for FormatSettings {
    fn from(conf: &FormatterConfiguration) -> Self {
        let indent_style = match conf.indent_style {
            PlainIndentStyle::Tab => IndentStyle::Tab,
            PlainIndentStyle::Space => IndentStyle::Space(conf.indent_size),
        };
        Self {
            indent_style: Some(indent_style),
            line_width: Some(conf.line_width),
            format_with_errors: false,
        }
    }
}

fn deserialize_line_width<'de, D>(deserializer: D) -> Result<LineWidth, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    let value: u16 = Deserialize::deserialize(deserializer)?;
    LineWidth::try_from(value).map_err(serde::de::Error::custom)
}

#[derive(Deserialize, Debug, Eq, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum PlainIndentStyle {
    /// Tab
    Tab,
    /// Space
    Space,
}

impl Default for PlainIndentStyle {
    fn default() -> Self {
        Self::Tab
    }
}
