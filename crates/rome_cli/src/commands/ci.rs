use crate::{
    configuration::load_configuration, execute_mode, CliSession, Execution, Termination,
    TraversalMode,
};
use rome_service::workspace::UpdateSettingsParams;

use super::format::apply_format_settings_from_cli;

/// Handler for the "ci" command of the Rome CLI
pub(crate) fn ci(mut session: CliSession) -> Result<(), Termination> {
    let mut configuration = load_configuration(&mut session)?;
    apply_format_settings_from_cli(&mut session, &mut configuration)?;

    session
        .app
        .workspace
        .update_settings(UpdateSettingsParams { configuration })?;

    execute_mode(Execution::new(TraversalMode::CI), session)
}
