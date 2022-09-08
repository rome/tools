use crate::{execute_mode, CliSession, Execution, Termination, TraversalMode};
use rome_service::load_config;
use rome_service::workspace::UpdateSettingsParams;

use super::format::apply_format_settings_from_cli;

/// Handler for the "ci" command of the Rome CLI
pub(crate) fn ci(mut session: CliSession) -> Result<(), Termination> {
    let configuration = load_config(&session.app.fs, None)?;
    let configuration = apply_format_settings_from_cli(&mut session, configuration)?;

    session
        .app
        .workspace
        .update_settings(UpdateSettingsParams { configuration })?;

    execute_mode(Execution::new(TraversalMode::CI), session)
}
