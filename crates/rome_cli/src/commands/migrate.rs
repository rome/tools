use crate::configuration::load_configuration;
use crate::diagnostics::MigrationDiagnostic;
use crate::execute::{execute_mode, Execution, TraversalMode};
use crate::{CliDiagnostic, CliSession};

/// Handler for the "check" command of the Rome CLI
pub(crate) fn migrate(mut session: CliSession) -> Result<(), CliDiagnostic> {
    let (_, _, path) = load_configuration(&mut session)?.consume();
    if let Some(path) = path {
        execute_mode(
            Execution::new(TraversalMode::Migrate {
                write: session.args.contains("--dry-run"),
                configuration_path: path,
            }),
            session,
        )
    } else {
        Err(CliDiagnostic::MigrateError(MigrationDiagnostic {
            reason: "Rome couldn't find the configuration file".to_string(),
        }))
    }
}
