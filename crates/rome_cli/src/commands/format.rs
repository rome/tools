use rome_formatter::IndentStyle;
use rome_service::configuration::{
    FormatterConfiguration, JavascriptConfiguration, JavascriptFormatter, PlainIndentStyle,
};
use rome_service::{workspace::UpdateSettingsParams, Configuration};
use std::path::PathBuf;

use crate::configuration::load_configuration;
use crate::execute::ReportMode;
use crate::{execute_mode, CliSession, Execution, Termination, TraversalMode};

/// Handler for the "format" command of the Rome CLI
pub(crate) fn format(mut session: CliSession) -> Result<(), Termination> {
    let mut configuration = load_configuration(&mut session)?;
    apply_format_settings_from_cli(&mut session, &mut configuration)?;

    session
        .app
        .workspace
        .update_settings(UpdateSettingsParams { configuration })?;

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
        let input_code = console.read();
        if let Some(input_code) = input_code {
            let path = PathBuf::from(stdin_file_path);
            Some((path, input_code))
        } else {
            // we provided the argument without a piped stdin, we bail
            return Err(Termination::MissingArgument {
                subcommand: "format",
                argument: "stdin",
            });
        }
    } else {
        None
    };

    let execution = if session.args.contains("--json") {
        Execution::with_report(
            TraversalMode::Format {
                ignore_errors,
                write: is_write,
                stdin,
            },
            ReportMode::Json,
        )
    } else {
        Execution::new(TraversalMode::Format {
            ignore_errors,
            write: is_write,
            stdin,
        })
    };

    execute_mode(execution, session)
}

/// Read the formatting options for the command line arguments and inject them
/// into the workspace settings
pub(crate) fn apply_format_settings_from_cli(
    session: &mut CliSession,
    configuration: &mut Configuration,
) -> Result<(), Termination> {
    let formatter = configuration
        .formatter
        .get_or_insert_with(FormatterConfiguration::default);

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

    let line_width = session
        .args
        .opt_value_from_str("--line-width")
        .map_err(|source| Termination::ParseError {
            argument: "--line-width",
            source,
        })?;

    match indent_style {
        Some(IndentStyle::Tab) => {
            formatter.indent_style = PlainIndentStyle::Tab;
        }
        Some(IndentStyle::Space(default_size)) => {
            formatter.indent_style = PlainIndentStyle::Space;
            formatter.indent_size = size.unwrap_or(default_size);
        }
        None => {}
    }

    if let Some(line_width) = line_width {
        formatter.line_width = line_width;
    }

    let quote_properties = session
        .args
        .opt_value_from_str("--quote-properties")
        .map_err(|source| Termination::ParseError {
            argument: "--quote-properties",
            source,
        })?;

    let quote_style = session
        .args
        .opt_value_from_str("--quote-style")
        .map_err(|source| Termination::ParseError {
            argument: "--quote-style",
            source,
        })?;

    let trailing_comma = session
        .args
        .opt_value_from_str("--trailing-comma")
        .map_err(|source| Termination::ParseError {
            argument: "--trailing-comma",
            source,
        })?;

    let semicolons = session
        .args
        .opt_value_from_str("--semicolons")
        .map_err(|source| Termination::ParseError {
            argument: "--semicolons",
            source,
        })?;

    let javascript = configuration
        .javascript
        .get_or_insert_with(JavascriptConfiguration::default);
    let javascript_formatter = javascript
        .formatter
        .get_or_insert_with(JavascriptFormatter::default);

    if let Some(quote_properties) = quote_properties {
        javascript_formatter.quote_properties = quote_properties;
    }

    if let Some(quote_style) = quote_style {
        javascript_formatter.quote_style = quote_style;
    }

    if let Some(trailing_comma) = trailing_comma {
        javascript_formatter.trailing_comma = trailing_comma;
    }

    if let Some(semicolons) = semicolons {
        javascript_formatter.semicolons = semicolons;
    }

    Ok(())
}
