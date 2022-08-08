use rome_service::configuration::Configuration;
use rome_service::settings;
use serde::{Deserialize, Serialize};
use serde_json::{Error, Value};
use tracing::trace;

pub(crate) const CONFIGURATION_SECTION: &str = "rome";

#[derive(Debug, Default, Clone, Deserialize, Serialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
/// The settings applied to the workspace by the LSP
pub struct WorkspaceSettings {
    /// Unstable features enabled
    #[serde(default)]
    pub unstable: bool,
}

#[derive(Debug)]
pub(crate) struct Config {
    settings: WorkspaceSettings,
}

impl Config {
    pub(crate) fn new() -> Self {
        Self {
            settings: WorkspaceSettings::default(),
        }
    }

    pub(crate) fn set_workspace_settings(&mut self, value: Value) -> Result<(), Error> {
        let workspace_settings = serde_json::from_value(value)?;
        self.settings = workspace_settings;
        trace!(
            "Correctly stored the settings coming from the client: {:?}",
            self.settings
        );
        Ok(())
    }

    /// Convert the current configuration to an instance of [settings::WorkspaceSettings]
    ///
    /// If the configuration file is found we use it with its defaults, otherwise
    /// we use the settings coming from the client
    pub(crate) fn as_workspace_settings(
        &self,
        configuration: Option<Configuration>,
    ) -> settings::WorkspaceSettings {
        let mut settings = settings::WorkspaceSettings::default();

        if let Some(configuration) = configuration {
            trace!("Applying configuration coming from the configuration file");
            settings.merge_with_configuration(configuration);
        }

        settings
    }
}
