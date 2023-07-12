use crate::cli_options::CliOptions;
use crate::configuration::{load_configuration, LoadedConfiguration};
use crate::vcs::store_path_to_ignore_from_vcs;
use crate::{execute_mode, CliDiagnostic, CliSession, Execution, TraversalMode};
use rome_service::workspace::{FixFileMode, UpdateSettingsParams};
use rome_service::{Configuration, MergeWith};
use std::ffi::OsString;
use std::path::PathBuf;

pub(crate) struct LintCommandPayload {
    pub(crate) apply: bool,
    pub(crate) apply_unsafe: bool,
    pub(crate) cli_options: CliOptions,
    pub(crate) configuration: Option<Configuration>,
    pub(crate) paths: Vec<OsString>,
    pub(crate) stdin_file_path: Option<String>,
}

/// Handler for the "lint" command of the Rome CLI
pub(crate) fn lint(
    mut session: CliSession,
    payload: LintCommandPayload,
) -> Result<(), CliDiagnostic> {
    let LintCommandPayload {
        apply,
        apply_unsafe,
        cli_options,
        configuration,
        paths,
        stdin_file_path,
    } = payload;

    let fix_file_mode = if apply && apply_unsafe {
        return Err(CliDiagnostic::incompatible_arguments(
            "--apply",
            "--apply-unsafe",
        ));
    } else if !apply && !apply_unsafe {
        None
    } else if apply && !apply_unsafe {
        Some(FixFileMode::SafeFixes)
    } else {
        Some(FixFileMode::SafeAndUnsafeFixes)
    };

    let LoadedConfiguration {
        configuration: mut fs_configuration,
        directory_path: configuration_path,
        ..
    } = load_configuration(&mut session, &cli_options)?
        .or_diagnostic(session.app.console, cli_options.verbose)?;

    fs_configuration.merge_with(configuration);

    // check if support of git ignore files is enabled
    let vcs_base_path = configuration_path.or(session.app.workspace.working_directory()?);
    store_path_to_ignore_from_vcs(
        &mut session,
        &mut fs_configuration,
        vcs_base_path,
        &cli_options,
    )?;

    let stdin = if let Some(stdin_file_path) = stdin_file_path {
        let console = &mut session.app.console;
        let input_code = console.read();
        if let Some(input_code) = input_code {
            let path = PathBuf::from(stdin_file_path);
            Some((path, input_code))
        } else {
            // we provided the argument without a piped stdin, we bail
            return Err(CliDiagnostic::missing_argument("stdin", "lint"));
        }
    } else {
        None
    };

    session
        .app
        .workspace
        .update_settings(UpdateSettingsParams {
            configuration: fs_configuration,
        })?;

    execute_mode(
        Execution::new(TraversalMode::Lint {
            fix_file_mode,
            stdin,
        }),
        session,
        &cli_options,
        paths,
    )
}
