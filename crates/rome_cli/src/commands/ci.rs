use crate::cli_options::CliOptions;
use crate::vcs::store_path_to_ignore_from_vcs;
use crate::{
    configuration::load_configuration, execute_mode, CliDiagnostic, CliSession, Execution,
    TraversalMode,
};
use rome_console::{markup, ConsoleExt};
use rome_diagnostics::PrintDiagnostic;
use rome_service::configuration::organize_imports::OrganizeImports;
use rome_service::configuration::{FormatterConfiguration, LinterConfiguration};
use rome_service::workspace::UpdateSettingsParams;
use rome_service::{MergeWith, RomeConfiguration};
use std::ffi::OsString;

pub(crate) struct CiCommandPayload {
    pub(crate) formatter_enabled: bool,
    pub(crate) linter_enabled: bool,
    pub(crate) organize_imports_enabled: bool,
    pub(crate) paths: Vec<OsString>,
    pub(crate) rome_configuration: RomeConfiguration,
    pub(crate) cli_options: CliOptions,
}

/// Handler for the "ci" command of the Rome CLI
pub(crate) fn ci(mut session: CliSession, payload: CiCommandPayload) -> Result<(), CliDiagnostic> {
    let (mut configuration, diagnostics, configuration_path) =
        load_configuration(&mut session, &payload.cli_options)?.consume();

    if !diagnostics.is_empty() {
        let console = &mut session.app.console;
        for diagnostic in diagnostics {
            console.error(markup! {
                {PrintDiagnostic::verbose(&diagnostic)}
            })
        }
        return Err(CliDiagnostic::incompatible_end_configuration(
            "The deserialization of the configuration resulted into an error.",
        ));
    }

    let formatter = configuration
        .formatter_configuration
        .get_or_insert_with(FormatterConfiguration::default);

    formatter.enabled = Some(payload.formatter_enabled);

    let linter = configuration
        .linter_configuration
        .get_or_insert_with(LinterConfiguration::default);

    linter.enabled = Some(payload.linter_enabled);

    let organize_imports = configuration
        .organize_imports
        .get_or_insert_with(OrganizeImports::default);

    organize_imports.enabled = Some(payload.organize_imports_enabled);

    // no point in doing the traversal if all the checks have been disabled
    if configuration.is_formatter_disabled()
        && configuration.is_linter_disabled()
        && configuration.is_organize_imports_disabled()
    {
        return Err(CliDiagnostic::incompatible_end_configuration("Formatter, linter and organize imports are disabled, can't perform the command. This is probably and error."));
    }

    configuration.merge_with(payload.rome_configuration.files_configuration);
    configuration.merge_with(payload.rome_configuration.vcs_configuration);
    configuration.merge_with_if(
        payload.rome_configuration.formatter_configuration,
        !configuration.is_formatter_disabled(),
    );
    configuration.merge_with_if(
        payload.rome_configuration.organize_imports,
        !configuration.is_formatter_disabled(),
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
