use crate::commands::format::parse_format_options;
use crate::{
    traversal::{traverse, TraversalMode},
    CliSession, Termination,
};
use rome_diagnostics::MAXIMUM_DISPLAYABLE_DIAGNOSTICS;
use rome_service::configuration::Configuration;
use rome_service::load_config;
use rome_service::load_config::ConfigurationType;
use rome_service::settings::{LinterSettings, WorkspaceSettings};
use rome_service::workspace::UpdateSettingsParams;

/// Handler for the "check" command of the Rome CLI
pub(crate) fn check(mut session: CliSession) -> Result<(), Termination> {
    let configuration = load_config(&session.app.fs, ConfigurationType::Root)?;
    let mut workspace_settings = WorkspaceSettings::default();

    let max_diagnostics: Option<u8> = session
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

    parse_format_options(&mut session, &mut workspace_settings, &configuration)?;
    parse_linter_options(&mut session, &mut workspace_settings, &configuration)?;

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

pub(crate) fn parse_linter_options(
    _session: &mut CliSession,
    workspace_settings: &mut WorkspaceSettings,
    configuration: &Option<Configuration>,
) -> Result<(), Termination> {
    if let Some(configuration) = configuration {
        workspace_settings.linter = LinterSettings::from(&configuration.linter);
    }

    Ok(())
}
