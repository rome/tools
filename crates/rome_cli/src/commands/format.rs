use rome_formatter::IndentStyle;
use rome_service::{load_config, settings::WorkspaceSettings, workspace::UpdateSettingsParams};
use std::path::PathBuf;

use crate::{
    traversal::{traverse, TraversalMode},
    CliSession, Termination,
};

/// Handler for the "format" command of the Rome CLI
pub(crate) fn format(mut session: CliSession) -> Result<(), Termination> {
    let configuration = load_config(&session.app.fs)?;
    let mut workspace_settings = WorkspaceSettings::default();

    if let Some(configuration) = &configuration {
        if configuration.is_formatter_disabled() {
            return Ok(());
        }
    }
    if let Some(configuration) = configuration {
        workspace_settings.merge_with_configuration(configuration);
    }

    apply_format_settings_from_cli(&mut session, &mut workspace_settings)?;

    let is_write = session.args.contains("--write");
    let ignore_errors = session.args.contains("--skip-errors");
    let stdin_file_path: Option<String> = session
        .args
        .opt_value_from_str("--stdin-file-path")
        .map_err(|source| Termination::ParseError {
            argument: "--stdin-file-path",
            source,
        })?;

    let stdin = if let Some(stdin_file_path) = stdin_file_path {
        let console = &mut session.app.console;
        let input_code = console.read(None);
        if let Some(input_code) = input_code {
            let path = PathBuf::from(stdin_file_path);
            Some((path, input_code))
        } else {
            // we provided the argument without a piped stdin, we bail
            return Err(Termination::MissingArgument { argument: "stdin" });
        }
    } else {
        None
    };

    session
        .app
        .workspace
        .update_settings(UpdateSettingsParams {
            settings: workspace_settings,
        })?;

    traverse(
        TraversalMode::Format {
            ignore_errors,
            write: is_write,
            stdin,
        },
        session,
    )
}

/// Read the formatting options for the command line arguments and inject them
/// into the workspace settings
pub(crate) fn apply_format_settings_from_cli(
    session: &mut CliSession,
    workspace_settings: &mut WorkspaceSettings,
) -> Result<(), Termination> {
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
