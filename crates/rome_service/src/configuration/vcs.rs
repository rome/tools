use serde::{Deserialize, Serialize};

const GIT_IGNORE_FILE_NAME: &str = ".gitignore";

/// Set
#[derive(Debug, Default, Deserialize, Serialize)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct VcsConfiguration {
    /// The kind of client. Default value is `git`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_kind: Option<VcsClientKind>,

    /// Whether Rome should integrate itself with the VCS client
    pub enabled: bool,

    /// Whether Rome should use the VCS ignore file. When [true], Rome will ignore the files
    /// specified in the ignore file.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub use_ignore_file: Option<bool>,

    /// The folder where Rome should check for VCS files. By default, Rome will use the
    /// the working directory.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root: Option<String>,
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

impl VcsConfiguration {
    pub const KNOWN_KEYS: &'static [&'static str] =
        &["clientKind", "enabled", "useIgnoreFile", "root"];

    /// Configuration for `git`
    pub fn git() -> Self {
        Self {
            client_kind: Some(VcsClientKind::Git),
            enabled: true,
            use_ignore_file: Some(true),
            root: None,
        }
    }
}
