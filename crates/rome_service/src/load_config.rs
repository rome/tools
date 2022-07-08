use crate::configuration::{Configuration, ConfigurationError};
use crate::RomeError;
use rome_fs::RomePath;
use std::path::PathBuf;

pub const CONFIG_FILENAME: &str = "rome.json";

/// This function is responsible to load the rome configuration.
///
/// The `config_path` is the path to the configuration file.
pub fn load_config(
    config_path: &PathBuf,
    configuration_type: ConfigurationType,
) -> Result<Configuration, RomeError> {
    // path of the configuration file
    let config_path = RomePath::new(config_path, 0);

    let buffer = config_path
        .read_to_string()
        .map_err(|_| RomeError::CantReadFile(config_path))?;

    let configuration: Configuration = serde_json::from_str(&buffer)
        .map_err(|err| RomeError::MalformedConfigurationFile(err.to_string()))?;

    compute_configuration(configuration, configuration_type)
}

/// The type of configuration we want to load
pub enum ConfigurationType {
    /// The main configuration, usually `rome.json`
    Root,
    /// The extended configuration, usually to be loaded via `extends` field
    #[allow(unused_imports)]
    Extended,
}

impl ConfigurationType {
    fn is_root(&self) -> bool {
        matches!(self, ConfigurationType::Root)
    }
}

/// This function computes the configuration that is being loaded and makes sure that is correct.
///
/// Operations are:
/// - making sure that the master configuration is set to `root: true`
fn compute_configuration(
    configuration: Configuration,
    configuration_type: ConfigurationType,
) -> Result<Configuration, RomeError> {
    if configuration_type.is_root() {
        if configuration.root == false {
            return Err(RomeError::InvalidConfiguration(ConfigurationError::NotRoot));
        }
    }

    Ok(configuration)
}
