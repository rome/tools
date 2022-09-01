use crate::settings::FormatSettings;
use indexmap::IndexSet;
use rome_formatter::{IndentStyle, LineWidth};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct FormatterConfiguration {
    // if `false`, it disables the feature. `true` by default
    pub enabled: bool,

    /// Stores whether formatting should be allowed to proceed if a given file
    /// has syntax errors
    pub format_with_errors: bool,

    /// The indent style.
    pub indent_style: PlainIndentStyle,

    /// The size of the indentation, 2 by default
    pub indent_size: u8,

    /// What's the max width of a line. Defaults to 80.
    #[serde(
        deserialize_with = "deserialize_line_width",
        serialize_with = "serialize_line_width"
    )]
    pub line_width: LineWidth,

    /// A list of Unix shell style patterns. The formatter will ignore files/folders that will
    /// match these patterns.
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::deserialize_set_of_strings",
        serialize_with = "crate::serialize_set_of_strings"
    )]
    pub ignore: Option<IndexSet<String>>,
}

impl Default for FormatterConfiguration {
    fn default() -> Self {
        Self {
            enabled: true,
            format_with_errors: false,
            indent_size: 2,
            indent_style: PlainIndentStyle::default(),
            line_width: LineWidth::default(),
            ignore: None,
        }
    }
}

impl From<FormatterConfiguration> for FormatSettings {
    fn from(conf: FormatterConfiguration) -> Self {
        let indent_style = match conf.indent_style {
            PlainIndentStyle::Tab => IndentStyle::Tab,
            PlainIndentStyle::Space => IndentStyle::Space(conf.indent_size),
        };
        Self {
            enabled: conf.enabled,
            indent_style: Some(indent_style),
            line_width: Some(conf.line_width),
            format_with_errors: conf.format_with_errors,
            ignored_files: conf.ignore.unwrap_or_default(),
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

pub fn serialize_line_width<S>(line_width: &LineWidth, s: S) -> Result<S::Ok, S::Error>
where
    S: serde::ser::Serializer,
{
    s.serialize_u16(line_width.value())
}

#[derive(Deserialize, Serialize, Debug, Eq, PartialEq, Default)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub enum PlainIndentStyle {
    /// Tab
    #[default]
    Tab,
    /// Space
    Space,
}
