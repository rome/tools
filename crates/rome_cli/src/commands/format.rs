use crate::configuration::load_configuration;
use crate::execute::ReportMode;
use crate::parse_arguments::{apply_files_settings_from_cli, apply_format_settings_from_cli};
use crate::{execute_mode, CliDiagnostic, CliSession, Execution, TraversalMode};
use rome_console::{markup, ConsoleExt};
use rome_diagnostics::{DiagnosticExt, PrintDiagnostic, Severity};
use rome_service::workspace::UpdateSettingsParams;
use std::path::PathBuf;

/// Handler for the "format" command of the Rome CLI
pub(crate) fn format(mut session: CliSession) -> Result<(), CliDiagnostic> {
    let (mut configuration, diagnostics, _) = load_configuration(&mut session)?.consume();
    if !diagnostics.is_empty() {
        let console = &mut session.app.console;
        console.log(markup!{
           <Warn>"Found errors in the configuration file, Rome will use its defaults for the sections that are incorrect."</Warn>
        });
        for diagnostic in diagnostics {
            let diagnostic = diagnostic.with_severity(Severity::Warning);
            console.log(markup! {
                {PrintDiagnostic::verbose(&diagnostic)}
            })
        }
    }

    apply_files_settings_from_cli(&mut session, &mut configuration)?;
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
        .map_err(|source| CliDiagnostic::parse_error("--stdin-file-path", source))?;

    let stdin = if let Some(stdin_file_path) = stdin_file_path {
        let console = &mut session.app.console;
        let input_code = console.read();
        if let Some(input_code) = input_code {
            let path = PathBuf::from(stdin_file_path);
            Some((path, input_code))
        } else {
            // we provided the argument without a piped stdin, we bail
            return Err(CliDiagnostic::missing_argument("stdin", "format"));
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
