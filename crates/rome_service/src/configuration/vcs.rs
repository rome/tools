use serde::{Deserialize, Serialize};
use std::str::FromStr;

const GIT_IGNORE_FILE_NAME: &str = ".gitignore";

/// Set
#[derive(Debug, Default, Deserialize, Serialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct VcsConfiguration {
    /// The kind of client.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_kind: Option<VcsClientKind>,

    /// Whether Rome should integrate itself with the VCS client
    pub enabled: bool,

    /// Whether Rome should use the VCS ignore file. When [true], Rome will ignore the files
    /// specified in the ignore file.
    pub use_ignore_file: Option<bool>,

    /// The folder where Rome should check for VCS files. By default, Rome will use the same
    /// folder where `rome.json` was found.
    ///
    /// If Rome can't fine the configuration, it will attempt to use the current working directory.
    /// If no current working directory can't be found, Rome won't use the VCS integration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root: Option<String>,
}

impl VcsConfiguration {
    pub const fn ignore_file_disabled(&self) -> bool {
        matches!(self.use_ignore_file, Some(false))
    }
}

#[derive(Debug, Default, Deserialize, Serialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase")]
pub enum VcsClientKind {
    #[default]
    /// Integration with the git client as VCS
    Git,
}

impl VcsClientKind {
    pub const KNOWN_VALUES: &'static [&'static str] = &["git"];

    pub const fn ignore_file(&self) -> &'static str {
        match self {
            VcsClientKind::Git => GIT_IGNORE_FILE_NAME,
        }
    }
}

impl FromStr for VcsClientKind {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "git" => Ok(Self::Git),
            _ => Err("Value not supported for VcsClientKind"),
        }
    }
}

impl VcsConfiguration {
    pub const KNOWN_KEYS: &'static [&'static str] =
        &["clientKind", "enabled", "useIgnoreFile", "root"];
}
