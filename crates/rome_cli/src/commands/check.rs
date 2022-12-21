use crate::commands::format::apply_format_settings_from_cli;
use crate::configuration::load_configuration;
use crate::{execute_mode, CliDiagnostic, CliSession, Execution, TraversalMode};
use rome_service::workspace::{FixFileMode, UpdateSettingsParams};

/// Handler for the "check" command of the Rome CLI
pub(crate) fn check(mut session: CliSession) -> Result<(), CliDiagnostic> {
    let mut configuration = load_configuration(&mut session)?;

    apply_format_settings_from_cli(&mut session, &mut configuration)?;

    session
        .app
        .workspace
        .update_settings(UpdateSettingsParams { configuration })?;

    let apply = session.args.contains("--apply");
    let apply_suggested = session.args.contains("--apply-suggested");

    let fix_file_mode = if apply && apply_suggested {
        return Err(CliDiagnostic::incompatible_arguments(
            "--apply",
            "--apply-suggested",
        ));
    } else if !apply && !apply_suggested {
        None
    } else if apply && !apply_suggested {
        Some(FixFileMode::SafeFixes)
    } else {
        Some(FixFileMode::SafeAndSuggestedFixes)
    };

    execute_mode(
        Execution::new(TraversalMode::Check { fix_file_mode }),
        session,
    )
}
