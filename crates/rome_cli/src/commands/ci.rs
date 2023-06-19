use crate::cli_options::CliOptions;
use crate::configuration::LoadedConfiguration;
use crate::vcs::store_path_to_ignore_from_vcs;
use crate::{
    configuration::load_configuration, execute_mode, CliDiagnostic, CliSession, Execution,
    TraversalMode,
};
use rome_console::{markup, ConsoleExt};
use rome_diagnostics::{DiagnosticExt, PrintDiagnostic};
use rome_service::configuration::organize_imports::OrganizeImports;
use rome_service::configuration::{FormatterConfiguration, LinterConfiguration};
use rome_service::workspace::UpdateSettingsParams;
use rome_service::{Configuration, MergeWith};
use std::ffi::OsString;

pub(crate) struct CiCommandPayload {
    pub(crate) formatter_enabled: Option<bool>,
    pub(crate) linter_enabled: Option<bool>,
    pub(crate) organize_imports_enabled: Option<bool>,
    pub(crate) paths: Vec<OsString>,
    pub(crate) rome_configuration: Configuration,
    pub(crate) cli_options: CliOptions,
}

/// Handler for the "ci" command of the Rome CLI
pub(crate) fn ci(mut session: CliSession, payload: CiCommandPayload) -> Result<(), CliDiagnostic> {
    let LoadedConfiguration {
        mut configuration,
        diagnostics,
        directory_path: configuration_path,
        file_path,
    } = load_configuration(&mut session, &payload.cli_options)?;

    if !diagnostics.is_empty() {
        let console = &mut session.app.console;
        console.log(markup!{
           <Error>"Found errors in the configuration file, Rome will use its defaults for the sections that are incorrect."</Error>
        });
        for diagnostic in diagnostics {
            let diagnostic = if let Some(file_path) = &file_path {
                diagnostic.with_file_path(file_path.display().to_string())
            } else {
                diagnostic
            };
            console.error(markup! {
				{if payload.cli_options.verbose { PrintDiagnostic::verbose(&diagnostic) } else { PrintDiagnostic::simple(&diagnostic) }}
            })
        }
        return Err(CliDiagnostic::incompatible_end_configuration(
            "The deserialization of the configuration resulted into an error.",
        ));
    }

    let formatter = configuration
        .formatter
        .get_or_insert_with(FormatterConfiguration::default);

    if !matches!(payload.formatter_enabled, None) {
        formatter.enabled = payload.formatter_enabled;
    }

    let linter = configuration
        .linter
        .get_or_insert_with(LinterConfiguration::default);

    if !matches!(payload.linter_enabled, None) {
        linter.enabled = payload.linter_enabled;
    }

    let organize_imports = configuration
        .organize_imports
        .get_or_insert_with(OrganizeImports::default);

    if !matches!(payload.organize_imports_enabled, None) {
        organize_imports.enabled = payload.organize_imports_enabled;
    }

    // no point in doing the traversal if all the checks have been disabled
    if configuration.is_formatter_disabled()
        && configuration.is_linter_disabled()
        && configuration.is_organize_imports_disabled()
    {
        return Err(CliDiagnostic::incompatible_end_configuration("Formatter, linter and organize imports are disabled, can't perform the command. This is probably and error."));
    }

    configuration.merge_with(payload.rome_configuration.files);
    configuration.merge_with(payload.rome_configuration.vcs);
    configuration.merge_with_if(
        payload.rome_configuration.formatter,
        !configuration.is_formatter_disabled(),
    );
    configuration.merge_with_if(
        payload.rome_configuration.organize_imports,
        !configuration.is_organize_imports_disabled(),
    );

    // check if support of git ignore files is enabled
    let vcs_base_path = configuration_path.or(session.app.fs.working_directory());
    store_path_to_ignore_from_vcs(
        &mut session,
        &mut configuration,
        vcs_base_path,
        &payload.cli_options,
    )?;

    session
        .app
        .workspace
        .update_settings(UpdateSettingsParams { configuration })?;

    execute_mode(
        Execution::new(TraversalMode::CI),
        session,
        &payload.cli_options,
        payload.paths,
    )
}
