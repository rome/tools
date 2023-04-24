use crate::configuration::string_set::StringSet;
use crate::settings::FormatSettings;
use crate::{ConfigurationDiagnostic, MatchOptions, Matcher, WorkspaceError};
use bpaf::Bpaf;
use indexmap::IndexSet;
use rome_formatter::{IndentStyle, LineWidth};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Deserialize, Serialize, Debug, Eq, PartialEq, Bpaf)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
#[bpaf(options)]
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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ignore: Option<StringSet>,
}

impl FormatterConfiguration {
    pub(crate) const KNOWN_KEYS: &'static [&'static str] = &[
        "enabled",
        "formatWithErrors",
        "indentStyle",
        "indentSize",
        "lineWidth",
        "ignore",
    ];
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

impl TryFrom<FormatterConfiguration> for FormatSettings {
    type Error = WorkspaceError;

    fn try_from(conf: FormatterConfiguration) -> Result<Self, Self::Error> {
        let indent_style = match conf.indent_style {
            PlainIndentStyle::Tab => IndentStyle::Tab,
            PlainIndentStyle::Space => IndentStyle::Space(conf.indent_size),
        };
        let mut matcher = Matcher::new(MatchOptions {
            case_sensitive: true,
            require_literal_leading_dot: false,
            require_literal_separator: false,
        });
        if let Some(ignore) = conf.ignore {
            for pattern in ignore.index_set() {
                matcher.add_pattern(pattern).map_err(|err| {
                    WorkspaceError::Configuration(
                        ConfigurationDiagnostic::new_invalid_ignore_pattern(
                            pattern.to_string(),
                            err.msg.to_string(),
                        ),
                    )
                })?;
            }
        }
        Ok(Self {
            enabled: conf.enabled,
            indent_style: Some(indent_style),
            line_width: Some(conf.line_width),
            format_with_errors: conf.format_with_errors,
            ignored_files: matcher,
        })
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

impl PlainIndentStyle {
    pub(crate) const KNOWN_VALUES: &'static [&'static str] = &["tab", "space"];
}

impl FromStr for PlainIndentStyle {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}
