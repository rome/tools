use crate::settings::FormatSettings;
use crate::{ConfigurationDiagnostic, MatchOptions, Matcher, WorkspaceError};
use indexmap::IndexSet;
use rome_formatter::{IndentStyle, LineWidth};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct FormatterConfiguration {
    // if `false`, it disables the feature. `true` by default
	#[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,

    /// Stores whether formatting should be allowed to proceed if a given file
    /// has syntax errors
	#[serde(skip_serializing_if = "Option::is_none")]
    pub format_with_errors: Option<bool>,

    /// The indent style.
	#[serde(skip_serializing_if = "Option::is_none")]
    pub indent_style: Option<PlainIndentStyle>,

    /// The size of the indentation, 2 by default
	#[serde(skip_serializing_if = "Option::is_none")]
    pub indent_size: Option<u8>,

    /// What's the max width of a line. Defaults to 80.
    #[serde(
        deserialize_with = "deserialize_line_width",
        serialize_with = "serialize_line_width"
    )]
	#[serde(skip_serializing_if = "Option::is_none")]
    pub line_width: Option<LineWidth>,

    /// A list of Unix shell style patterns. The formatter will ignore files/folders that will
    /// match these patterns.
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::deserialize_set_of_strings",
        serialize_with = "crate::serialize_set_of_strings"
    )]
    pub ignore: Option<IndexSet<String>>,
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
            enabled: Some(true),
            format_with_errors: Some(false),
            indent_size: Some(2),
            indent_style: Some(PlainIndentStyle::default()),
            line_width: Some(LineWidth::default()),
            ignore: None,
        }
    }
}

impl TryFrom<FormatterConfiguration> for FormatSettings {
    type Error = WorkspaceError;

    fn try_from(conf: FormatterConfiguration) -> Result<Self, Self::Error> {
        let indent_style = match conf.indent_style {
            Some(PlainIndentStyle::Tab) => IndentStyle::Tab,
            Some(PlainIndentStyle::Space) => IndentStyle::Space(conf.indent_size),
        };
        let mut matcher = Matcher::new(MatchOptions {
            case_sensitive: true,
            require_literal_leading_dot: false,
            require_literal_separator: false,
        });
        if let Some(ignore) = conf.ignore {
            for pattern in ignore {
                matcher.add_pattern(&pattern).map_err(|err| {
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

fn deserialize_line_width<'de, D>(deserializer: D) -> Result<Option<LineWidth>, D::Error>
where
    D: serde::de::Deserializer<'de>,
{
    let value: u16 = Deserialize::deserialize(deserializer)?;
    Option::<LineWidth>::try_from(value).map_err(serde::de::Error::custom)
}

pub fn serialize_line_width<S>(line_width: &Option<LineWidth>, s: S) -> Result<S::Ok, S::Error>
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
