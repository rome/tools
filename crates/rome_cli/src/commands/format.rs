use rome_formatter::IndentStyle;
use rome_service::configuration::{
    FormatterConfiguration, JavascriptConfiguration, JavascriptFormatter, PlainIndentStyle,
};
use rome_service::{load_config, workspace::UpdateSettingsParams, Configuration};
use std::path::PathBuf;

use crate::execute::ReportMode;
use crate::{execute_mode, CliSession, Execution, Termination, TraversalMode};

/// Handler for the "format" command of the Rome CLI
pub(crate) fn format(mut session: CliSession) -> Result<(), Termination> {
    let configuration = load_config(&session.app.fs, None)?;
    let configuration = apply_format_settings_from_cli(&mut session, configuration)?;

    dbg!(&configuration);
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
            return Err(Termination::MissingArgument { argument: "stdin" });
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
    configuration: Option<Configuration>,
) -> Result<Configuration, Termination> {
    let mut configuration = if let Some(configuration) = configuration {
        configuration
    } else {
        Configuration {
            formatter: Some(FormatterConfiguration::default()),
            javascript: Some(JavascriptConfiguration {
                formatter: Some(JavascriptFormatter::default()),
                globals: None,
            }),
            ..Configuration::default()
        }
    };

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

    // if at least one argument is passed via CLI and no "formatter" configuration was passed
    // via `rome.json`, we need to create it
    if (line_width.is_some() | indent_style.is_some() | size.is_some())
        && configuration.formatter.is_none()
    {
        configuration.formatter = Some(FormatterConfiguration::default());
    }

    if let Some(formatter) = configuration.formatter.as_mut() {
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

    // if at least one argument is passed via CLI and no "javascript.formatter" configuration was passed
    // via `rome.json`, we need to create it
    if quote_style.is_some() | quote_properties.is_some() {
        if configuration.javascript.is_none() {
            configuration.javascript = Some(JavascriptConfiguration::with_formatter())
        } else if let Some(javascript) = configuration.javascript.as_mut() {
            if javascript.formatter.is_none() {
                javascript.formatter = Some(JavascriptFormatter::default());
            }
        }
    }
    if let Some(javascript) = configuration
        .javascript
        .as_mut()
        .and_then(|j| j.formatter.as_mut())
    {
        if let Some(quote_properties) = quote_properties {
            javascript.quote_properties = quote_properties;
        }

        if let Some(quote_style) = quote_style {
            javascript.quote_style = quote_style;
        }
    }

    Ok(configuration)
}
