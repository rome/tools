use crate::commands::format::apply_format_settings_from_cli;
use crate::{
    traversal::{traverse, TraversalMode},
    CliSession, Termination,
};
use rome_diagnostics::MAXIMUM_DISPLAYABLE_DIAGNOSTICS;
use rome_service::load_config;
use rome_service::settings::WorkspaceSettings;
use rome_service::workspace::UpdateSettingsParams;

/// Handler for the "check" command of the Rome CLI
pub(crate) fn check(mut session: CliSession) -> Result<(), Termination> {
    let configuration = load_config(&session.app.fs)?;
    let mut workspace_settings = WorkspaceSettings::default();

    let max_diagnostics: Option<u16> = session
        .args
        .opt_value_from_str("--max-diagnostics")
        .map_err(|source| Termination::ParseError {
            argument: "--max-diagnostics",
            source,
        })?;
    let max_diagnostics = if let Some(max_diagnostics) = max_diagnostics {
        if max_diagnostics > MAXIMUM_DISPLAYABLE_DIAGNOSTICS {
            return Err(Termination::OverflowNumberArgument(
                "--max-diagnostics",
                "50",
            ));
        }

        max_diagnostics
    } else {
        // default value
        20
    };

    if let Some(configuration) = configuration {
        workspace_settings.merge_with_configuration(configuration)
    }

    apply_format_settings_from_cli(&mut session, &mut workspace_settings)?;

    session
        .app
        .workspace
        .update_settings(UpdateSettingsParams {
            settings: workspace_settings,
        })?;

    traverse(
        TraversalMode::Check {
            max_diagnostics,
            should_fix: session.args.contains("--apply"),
        },
        session,
    )
}
