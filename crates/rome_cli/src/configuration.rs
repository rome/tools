use rome_service::{load_config, Configuration};

use crate::{CliSession, Termination};

/// Load the configuration for this session of the CLI, merging the content of
/// the `rome.json` file if it exists on disk with common command line options
pub(crate) fn load_configuration(session: &mut CliSession) -> Result<Configuration, Termination> {
    let mut configuration = load_config(&session.app.fs, None)?.unwrap_or_default();

    let files_max_size = session
        .args
        .opt_value_from_str("--files-max-size")
        .map_err(|source| Termination::ParseError {
            argument: "--files-max-size",
            source,
        })?;

    if let Some(files_max_size) = files_max_size {
        let files = configuration.files.get_or_insert_with(Default::default);
        files.max_size = Some(files_max_size);
    }

    Ok(configuration)
}
