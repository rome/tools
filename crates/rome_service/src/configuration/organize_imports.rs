use crate::settings::AnalyzerSettings;
use crate::WorkspaceError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Serialize, Deserialize, Eq, PartialEq)]
#[cfg_attr(feature = "schemars", derive(schemars::JsonSchema))]
#[serde(deny_unknown_fields)]
pub struct OrganizeImports {
    /// Enables the organization of imports
    pub enabled: bool,
}

impl TryFrom<OrganizeImports> for AnalyzerSettings {
    type Error = WorkspaceError;

    fn try_from(organize_imports: OrganizeImports) -> Result<Self, Self::Error> {
        Ok(Self {
            organize_imports_enabled: organize_imports.enabled,
        })
    }
}
