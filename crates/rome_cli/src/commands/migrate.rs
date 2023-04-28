use crate::cli_options::CliOptions;
use crate::configuration::load_configuration;
use crate::diagnostics::MigrationDiagnostic;
use crate::execute::{execute_mode, Execution, TraversalMode};
use crate::{CliDiagnostic, CliSession};

/// Handler for the "check" command of the Rome CLI
pub(crate) fn migrate(
    mut session: CliSession,
    cli_options: CliOptions,
    write: bool,
) -> Result<(), CliDiagnostic> {
    let (_, _, path) = load_configuration(&mut session, &cli_options)?.consume();
    let config_name = session.app.fs.config_name();
    if let Some(path) = path {
        execute_mode(
            Execution::new(TraversalMode::Migrate {
                write,
                configuration_path: path.join(config_name),
            }),
            session,
            &cli_options,
            vec![],
        )
    } else {
        Err(CliDiagnostic::MigrateError(MigrationDiagnostic {
            reason: "Rome couldn't find the configuration file".to_string(),
        }))
    }
}
