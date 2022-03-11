use serde::{Deserialize, Serialize};
use serde_json::{Error, Value};
use tracing::trace;

pub const CONFIGURATION_SECTION: &str = "rome";

#[derive(Debug, Default, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
/// Specific settings for Rome formatter
pub struct FormatterWorkspaceSettings {
    /// Allows to format code that might contain syntax errors
    pub format_with_syntax_errors: bool,
}

#[derive(Debug, Default, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
/// Settings for Rome Analysis
pub struct AnalysisWorkspaceSettings {
    /// Allows rome to compute and publish diagnostics
    pub enable_diagnostics: bool,
    /// Allows rome to compute and provide code actions
    pub enable_code_actions: bool,
}

#[derive(Debug, Default, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
/// The settings applied to the workspace by the LSP
pub struct WorkspaceSettings {
    /// Formatter settings
    #[serde(default)]
    pub formatter: FormatterWorkspaceSettings,

    /// Analysis settings
    #[serde(default)]
    pub analysis: AnalysisWorkspaceSettings,
}

#[derive(Debug)]
pub struct Config {
    settings: WorkspaceSettings,
}

impl Config {
    pub fn new() -> Self {
        Self {
            settings: WorkspaceSettings::default(),
        }
    }

    pub fn get_workspace_settings(&self) -> WorkspaceSettings {
        self.settings.clone()
    }

    pub fn set_workspace_settings(&mut self, value: Value) -> Result<(), Error> {
        let workspace_settings = serde_json::from_value(value)?;
        self.settings = workspace_settings;
        trace!(
            "Correctly stored the settings coming from the client: {:?}",
            self.settings
        );
        Ok(())
    }
}
