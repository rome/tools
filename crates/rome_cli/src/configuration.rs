use std::path::PathBuf;

use crate::cli_options::CliOptions;
use crate::{CliDiagnostic, CliSession};
use rome_deserialize::Deserialized;
use rome_service::{load_config, ConfigurationBasePath, RomeConfiguration};

#[derive(Default)]
pub struct LoadedConfiguration {
    deserialized: Deserialized<RomeConfiguration>,
    path: Option<PathBuf>,
}

impl LoadedConfiguration {
    pub fn consume(
        self,
    ) -> (
        RomeConfiguration,
        Vec<rome_diagnostics::Error>,
        Option<PathBuf>,
    ) {
        let path = self.path;
        let (configuration, diagnostics) = self.deserialized.consume();
        (configuration, diagnostics, path)
    }
}

impl From<Option<(Deserialized<RomeConfiguration>, PathBuf)>> for LoadedConfiguration {
    fn from(value: Option<(Deserialized<RomeConfiguration>, PathBuf)>) -> Self {
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
    cli_options: &CliOptions,
) -> Result<LoadedConfiguration, CliDiagnostic> {
    let base_path = match &cli_options.config_path {
        None => ConfigurationBasePath::default(),
        Some(path) => ConfigurationBasePath::FromUser(PathBuf::from(path)),
    };

    let config = load_config(&session.app.fs, base_path)?;

    Ok(config.into())
}
