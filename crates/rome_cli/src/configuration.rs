use crate::{CliDiagnostic, CliSession};
use rome_console::{markup, ConsoleExt};
use rome_diagnostics::PrintDiagnostic;
use rome_service::{load_config, Configuration};

/// Load the configuration for this session of the CLI, merging the content of
/// the `rome.json` file if it exists on disk with common command line options
pub(crate) fn load_configuration(session: &mut CliSession) -> Result<Configuration, CliDiagnostic> {
    let console = &mut session.app.console;
    let (mut configuration, diagnostics) = load_config(&session.app.fs, None)?.unwrap_or_default();
    if !diagnostics.is_empty() {
        console.log(markup! {
            "The configuration has errors, Rome will use its defaults for the sections that were invalid."
        });
        for diagnostic in diagnostics {
            console.error(markup! {
                {PrintDiagnostic::verbose(&diagnostic)}
            })
        }
    }

    let files_max_size = session
        .args
        .opt_value_from_str("--files-max-size")
        .map_err(|source| CliDiagnostic::parse_error("--files-max-size", source))?;

    if let Some(files_max_size) = files_max_size {
        let files = configuration.files.get_or_insert_with(Default::default);
        files.max_size = Some(files_max_size);
    }

    Ok(configuration)
}
