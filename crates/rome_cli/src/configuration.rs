use std::path::PathBuf;

use rome_service::{load_config, Configuration};

use crate::{CliDiagnostic, CliSession};

/// Load the configuration for this session of the CLI, merging the content of
/// the `rome.json` file if it exists on disk with common command line options
pub(crate) fn load_configuration(session: &mut CliSession) -> Result<Configuration, CliDiagnostic> {
    let config_path: Option<PathBuf> = session
        .args
        .opt_value_from_str("--config-path")
        .map_err(|source| CliDiagnostic::parse_error("--config-path", source))?;

    let is_config_path = config_path.is_some();

    let mut config = load_config(&session.app.fs, config_path, is_config_path)?.unwrap_or_default();

    let files_max_size = session
        .args
        .opt_value_from_str("--files-max-size")
        .map_err(|source| CliDiagnostic::parse_error("--files-max-size", source))?;

    if let Some(files_max_size) = files_max_size {
        let files = config.files.get_or_insert_with(Default::default);
        files.max_size = Some(files_max_size);
    }

    Ok(config)
}
