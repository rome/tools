use std::path::PathBuf;

use crate::{CliDiagnostic, CliSession};
use rome_deserialize::Deserialized;
use rome_service::{load_config, Configuration, ConfigurationBasePath};

/// Load the configuration for this session of the CLI, merging the content of
/// the `rome.json` file if it exists on disk with common command line options
pub(crate) fn load_configuration(
    session: &mut CliSession,
) -> Result<Deserialized<Configuration>, CliDiagnostic> {
    let config_path: Option<PathBuf> = session
        .args
        .opt_value_from_str("--config-path")
        .map_err(|source| CliDiagnostic::parse_error("--config-path", source))?;

    let base_path = match config_path {
        None => ConfigurationBasePath::default(),
        Some(path) => ConfigurationBasePath::FromUser(path),
    };

    let config = load_config(&session.app.fs, base_path)?;

    Ok(config.unwrap_or_default())
}
