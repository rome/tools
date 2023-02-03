use crate::{CliDiagnostic, CliSession};
use rome_deserialize::Deserialized;
use rome_service::{load_config, Configuration};

/// Load the configuration for this session of the CLI, merging the content of
/// the `rome.json` file if it exists on disk with common command line options
pub(crate) fn load_configuration(
    session: &mut CliSession,
) -> Result<Deserialized<Configuration>, CliDiagnostic> {
    Ok(load_config(&session.app.fs, None)?.unwrap_or_default())
}
