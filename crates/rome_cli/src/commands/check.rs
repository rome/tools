use crate::cli_options::CliOptions;
use crate::configuration::load_configuration;
use crate::vcs::store_path_to_ignore_from_vcs;
use crate::{execute_mode, CliDiagnostic, CliSession, Execution, TraversalMode};
use rome_console::{markup, ConsoleExt};
use rome_diagnostics::{DiagnosticExt, PrintDiagnostic, Severity};
use rome_service::configuration::organize_imports::OrganizeImports;
use rome_service::configuration::{FormatterConfiguration, LinterConfiguration};
use rome_service::workspace::{FixFileMode, UpdateSettingsParams};
use rome_service::{Configuration, MergeWith};
use std::ffi::OsString;
use std::path::PathBuf;

pub(crate) struct CheckCommandPayload {
    pub(crate) apply: bool,
    pub(crate) apply_unsafe: bool,
    pub(crate) cli_options: CliOptions,
    pub(crate) configuration: Option<Configuration>,
    pub(crate) paths: Vec<OsString>,
    pub(crate) stdin_file_path: Option<String>,
    pub(crate) formatter_enabled: Option<bool>,
    pub(crate) linter_enabled: Option<bool>,
    pub(crate) organize_imports_enabled: Option<bool>,
}

/// Handler for the "check" command of the Rome CLI
pub(crate) fn check(
    mut session: CliSession,
    payload: CheckCommandPayload,
) -> Result<(), CliDiagnostic> {
    let CheckCommandPayload {
        apply,
        apply_unsafe,
        cli_options,
        configuration,
        paths,
        stdin_file_path,
        linter_enabled,
        organize_imports_enabled,
        formatter_enabled,
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

    let (mut fs_configuration, diagnostics, configuration_path) =
        load_configuration(&mut session, &cli_options)?.consume();
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

    let formatter = fs_configuration
        .formatter
        .get_or_insert_with(FormatterConfiguration::default);

    if !matches!(formatter_enabled, None) {
        formatter.enabled = formatter_enabled;
    }

    let linter = fs_configuration
        .linter
        .get_or_insert_with(LinterConfiguration::default);

    if !matches!(linter_enabled, None) {
        linter.enabled = linter_enabled;
    }

    let organize_imports = fs_configuration
        .organize_imports
        .get_or_insert_with(OrganizeImports::default);

    if !matches!(organize_imports_enabled, None) {
        organize_imports.enabled = organize_imports_enabled;
    }

    fs_configuration.merge_with(configuration);

    // check if support of git ignore files is enabled
    let vcs_base_path = configuration_path.or(session.app.fs.working_directory());
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
            return Err(CliDiagnostic::missing_argument("stdin", "check"));
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
        Execution::new(TraversalMode::Check {
            fix_file_mode,
            stdin,
        }),
        session,
        &cli_options,
        paths,
    )
}
