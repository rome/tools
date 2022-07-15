use rome_formatter::IndentStyle;
use rome_service::configuration::Configuration;
use rome_service::settings::FormatSettings;
use rome_service::{
    load_config, settings::WorkspaceSettings, workspace::UpdateSettingsParams, ConfigurationType,
};

use crate::{
    traversal::{traverse, TraversalMode},
    CliSession, Termination,
};

/// Handler for the "format" command of the Rome CLI
pub(crate) fn format(mut session: CliSession) -> Result<(), Termination> {
    let configuration = load_config(&session.app.fs, ConfigurationType::Root)?;
    let mut settings = WorkspaceSettings::default();

    dbg!(&configuration);
    if let Some(configuration) = &configuration {
        if configuration.is_formatter_disabled() {
            return Ok(());
        }
    }

    parse_format_options(&mut session, &mut settings, &configuration)?;

    let is_write = session.args.contains("--write");
    let ignore_errors = session.args.contains("--skip-errors");

    session
        .app
        .workspace
        .update_settings(UpdateSettingsParams { settings })?;

    traverse(
        TraversalMode::Format {
            ignore_errors,
            write: is_write,
        },
        session,
    )
}

/// Read the formatting options for the command line arguments and inject them
/// into the workspace settings
pub(crate) fn parse_format_options(
    session: &mut CliSession,
    workspace_settings: &mut WorkspaceSettings,
    configuration: &Option<Configuration>,
) -> Result<(), Termination> {
    if let Some(configuration) = configuration {
        if let Some(formatter) = configuration.formatter {
            settings.format = FormatSettings::from(&formatter);
        }
        let formatter = configuration.javascript.and_then(|j| j.formatter);
        if let Some(formatter) = formatter {
            settings.languages.javascript.format.quote_style = Some(formatter.quote_style);
        }
    }

    let size = session
        .args
        .opt_value_from_str("--indent-size")
        .map_err(|source| Termination::ParseError {
            argument: "--indent-size",
            source,
        })?;

    let indent_style = session
        .args
        .opt_value_from_str("--indent-style")
        .map_err(|source| Termination::ParseError {
            argument: "--indent-style",
            source,
        })?;

    match indent_style {
        Some(IndentStyle::Tab) => {
            workspace_settings.format.indent_style = Some(IndentStyle::Tab);
        }
        Some(IndentStyle::Space(default_size)) => {
            workspace_settings.format.indent_style =
                Some(IndentStyle::Space(size.unwrap_or(default_size)));
        }
        None => {}
    }

    let quote_style = session
        .args
        .opt_value_from_str("--quote-style")
        .map_err(|source| Termination::ParseError {
            argument: "--quote-style",
            source,
        })?;

    if let Some(quote_style) = quote_style {
        workspace_settings.languages.javascript.format.quote_style = Some(quote_style);
    }

    let line_width = session
        .args
        .opt_value_from_str("--line-width")
        .map_err(|source| Termination::ParseError {
            argument: "--line-width",
            source,
        })?;

    if let Some(line_width) = line_width {
        workspace_settings.format.line_width = Some(line_width);
    }

    Ok(())
}
