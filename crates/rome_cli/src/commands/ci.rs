use crate::{
    traversal::{traverse, TraversalMode},
    CliSession, Termination,
};
use rome_service::load_config;
use rome_service::settings::WorkspaceSettings;
use rome_service::workspace::UpdateSettingsParams;

use super::format::apply_format_settings_from_cli;

/// Handler for the "ci" command of the Rome CLI
pub(crate) fn ci(mut session: CliSession) -> Result<(), Termination> {
    let configuration = load_config(&session.app.fs)?;
    let mut workspace_settings = WorkspaceSettings::default();

    if let Some(configuration) = configuration {
        workspace_settings.merge_with_configuration(configuration);
    }

    apply_format_settings_from_cli(&mut session, &mut workspace_settings)?;

    session
        .app
        .workspace
        .update_settings(UpdateSettingsParams {
            settings: workspace_settings,
        })?;

    traverse(TraversalMode::CI, session)
}
