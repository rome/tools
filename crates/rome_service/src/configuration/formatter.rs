use rome_formatter::{IndentStyle, LineWidth, LineWidthFromIntError};
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
    pub line_width: u16,
}

impl From<&FormatterConfiguration> for IndentStyle {
    fn from(c: &FormatterConfiguration) -> Self {
        match c.indent_style {
            PlainIndentStyle::Tab => IndentStyle::Tab,
            PlainIndentStyle::Space => IndentStyle::Space(c.indent_size),
        }
    }
}

impl TryFrom<&FormatterConfiguration> for LineWidth {
    type Error = LineWidthFromIntError;
    fn try_from(value: &FormatterConfiguration) -> Result<Self, Self::Error> {
        LineWidth::try_from(value.line_width)
    }
}

impl Default for FormatterConfiguration {
    fn default() -> Self {
        Self {
            enabled: true,
            format_with_errors: false,
            indent_size: 2,
            indent_style: PlainIndentStyle::default(),
            line_width: LineWidth::default().value(),
        }
    }
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
