use crate::commands::check::parse_linter_options;
use crate::{
    traversal::{traverse, TraversalMode},
    CliSession, Termination,
};
use rome_service::settings::WorkspaceSettings;
use rome_service::workspace::UpdateSettingsParams;
use rome_service::{load_config, ConfigurationType};

use super::format::parse_format_options;

/// Handler for the "ci" command of the Rome CLI
pub(crate) fn ci(mut session: CliSession) -> Result<(), Termination> {
    let configuration = load_config(&session.app.fs, ConfigurationType::Root)?;
    let mut workspace_settings = WorkspaceSettings::default();

    parse_format_options(&mut session, &mut workspace_settings, &configuration)?;
    parse_linter_options(&mut session, &mut workspace_settings, &configuration)?;

    session
        .app
        .workspace
        .update_settings(UpdateSettingsParams {
            settings: workspace_settings,
        })?;

    traverse(TraversalMode::CI, session)
}
