use std::path::PathBuf;

use crate::{CliDiagnostic, CliSession};
use rome_deserialize::Deserialized;
use rome_service::{load_config, Configuration, ConfigurationBasePath};

#[derive(Default)]
pub struct LoadedConfiguration {
    deserialized: Deserialized<Configuration>,
    path: Option<PathBuf>,
}

impl LoadedConfiguration {
    pub fn consume(self) -> (Configuration, Vec<rome_diagnostics::Error>, Option<PathBuf>) {
        let path = self.path;
        let (configuration, diagnostics) = self.deserialized.consume();
        (configuration, diagnostics, path)
    }
}

impl From<Option<(Deserialized<Configuration>, PathBuf)>> for LoadedConfiguration {
    fn from(value: Option<(Deserialized<Configuration>, PathBuf)>) -> Self {
        if let Some((deserialized, path)) = value {
            LoadedConfiguration {
                deserialized,
                path: Some(path),
            }
        } else {
            LoadedConfiguration::default()
        }
    }
}

/// Load the configuration for this session of the CLI, merging the content of
/// the `rome.json` file if it exists on disk with common command line options
pub(crate) fn load_configuration(
    session: &mut CliSession,
) -> Result<LoadedConfiguration, CliDiagnostic> {
    let config_path: Option<PathBuf> = session
        .args
        .opt_value_from_str("--config-path")
        .map_err(|source| CliDiagnostic::parse_error("--config-path", source))?;

    let base_path = match config_path {
        None => ConfigurationBasePath::default(),
        Some(path) => ConfigurationBasePath::FromUser(path),
    };

    let config = load_config(&session.app.fs, base_path)?;

    Ok(config.into())
}
