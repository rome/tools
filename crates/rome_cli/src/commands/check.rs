use crate::configuration::load_configuration;
use crate::parse_arguments::{apply_files_settings_from_cli, apply_format_settings_from_cli};
use crate::vcs::read_vcs_ignore_file;
use crate::{execute_mode, CliDiagnostic, CliSession, Execution, TraversalMode};
use indexmap::IndexSet;
use rome_console::{markup, ConsoleExt};
use rome_diagnostics::{DiagnosticExt, PrintDiagnostic, Severity};
use rome_service::configuration::FilesConfiguration;
use rome_service::workspace::{FixFileMode, UpdateSettingsParams};

/// Handler for the "check" command of the Rome CLI
pub(crate) fn check(mut session: CliSession) -> Result<(), CliDiagnostic> {
    let (mut configuration, diagnostics, configuration_path) =
        load_configuration(&mut session)?.consume();
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

    // check if support of git ignore files is enabled

    let files_to_ignore =
        if let (Some(vcs), Some(configuration_path)) = (&configuration.vcs, configuration_path) {
            read_vcs_ignore_file(&mut session, configuration_path, vcs)?
        } else {
            vec![]
        };
    if files_to_ignore.len() > 0 {
        let files = configuration
            .files
            .get_or_insert_with(FilesConfiguration::default);
        let ignored_files = files.ignore.get_or_insert_with(IndexSet::new);
        ignored_files.extend(files_to_ignore.into_iter());
    }

    session
        .app
        .workspace
        .update_settings(UpdateSettingsParams { configuration })?;

    let apply = session.args.contains("--apply");
    let apply_suggested = session.args.contains("--apply-unsafe");

    let fix_file_mode = if apply && apply_suggested {
        return Err(CliDiagnostic::incompatible_arguments(
            "--apply",
            "--apply-unsafe",
        ));
    } else if !apply && !apply_suggested {
        None
    } else if apply && !apply_suggested {
        Some(FixFileMode::SafeFixes)
    } else {
        Some(FixFileMode::SafeAndUnsafeFixes)
    };

    execute_mode(
        Execution::new(TraversalMode::Check { fix_file_mode }),
        session,
    )
}
