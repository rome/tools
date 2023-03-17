use crate::settings::OrganizeImportsSettings;
use crate::{ConfigurationDiagnostic, MatchOptions, Matcher, WorkspaceError};
use indexmap::IndexSet;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize, Eq, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(rename_all = "camelCase", default, deny_unknown_fields)]
pub struct OrganizeImports {
    /// Enables the organization of imports
    pub enabled: bool,

    /// A list of Unix shell style patterns. The formatter will ignore files/folders that will
    /// match these patterns.
    #[serde(
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::deserialize_set_of_strings",
        serialize_with = "crate::serialize_set_of_strings"
    )]
    pub ignore: Option<IndexSet<String>>,
}

impl TryFrom<OrganizeImports> for OrganizeImportsSettings {
    type Error = WorkspaceError;

    fn try_from(organize_imports: OrganizeImports) -> Result<Self, Self::Error> {
        let mut matcher = Matcher::new(MatchOptions {
            case_sensitive: true,
            require_literal_leading_dot: false,
            require_literal_separator: false,
        });
        if let Some(ignore) = organize_imports.ignore {
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
            enabled: organize_imports.enabled,
            ignored_files: matcher,
        })
    }
}
