use crate::configuration::{Configuration, ConfigurationError};
use crate::{DynRef, RomeError};
use rome_fs::FileSystem;
use std::env::current_dir;

/// This function is responsible to load the rome configuration.
///
/// The `file_system` will read the configuration file
pub fn load_config(
    file_system: &DynRef<dyn FileSystem>,
    configuration_type: ConfigurationType,
) -> Result<Option<Configuration>, RomeError> {
    let working_directory = current_dir().ok();
    let config_name = file_system.config_name();
    if let Some(working_directory) = working_directory {
        let configuration_path = working_directory.join(config_name);
        let file = file_system.open(&configuration_path);

        return match file {
            Ok(mut file) => {
                let mut buffer = String::new();
                file.read_to_string(&mut buffer).ok();

                let configuration: Configuration = serde_json::from_str(&buffer)
                    .map_err(|err| RomeError::MalformedConfigurationFile(err.to_string()))?;

                compute_configuration(configuration, configuration_type)
            }
            Err(_) => {
                // TODO: log possible error
                Ok(None)
            }
        };
    }

    Ok(None)
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
) -> Result<Option<Configuration>, RomeError> {
    if configuration_type.is_root() && !configuration.root {
        return Err(RomeError::InvalidConfiguration(ConfigurationError::NotRoot));
    }

    Ok(Some(configuration))
}
