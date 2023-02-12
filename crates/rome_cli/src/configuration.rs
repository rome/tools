use std::path::PathBuf;

use crate::{CliDiagnostic, CliSession};
use rome_deserialize::Deserialized;
use rome_service::{load_config, Configuration};

/// Load the configuration for this session of the CLI, merging the content of
/// the `rome.json` file if it exists on disk with common command line options
pub(crate) fn load_configuration(
    session: &mut CliSession,
) -> Result<Deserialized<Configuration>, CliDiagnostic> {
    let config_path: Option<PathBuf> = session
        .args
        .opt_value_from_str("--config-path")
        .map_err(|source| CliDiagnostic::parse_error("--config-path", source))?;

    let is_config_path = config_path.is_some();

    Ok(load_config(&session.app.fs, config_path, is_config_path)?.unwrap_or_default())
}
