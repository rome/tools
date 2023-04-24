use crate::parse_arguments::{
    apply_files_settings_from_cli, apply_format_settings_from_cli, apply_vcs_settings_from_cli,
};
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

/// Handler for the "ci" command of the Rome CLI
pub(crate) fn ci(mut session: CliSession) -> Result<(), CliDiagnostic> {
    let (mut configuration, diagnostics, configuration_path) =
        load_configuration(&mut session)?.consume();

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

    let formatter_enabled = session
        .args
        .opt_value_from_str("--formatter-enabled")
        .map_err(|source| CliDiagnostic::parse_error("--formatter-enabled", source))?;

    let linter_enabled = session
        .args
        .opt_value_from_str("--linter-enabled")
        .map_err(|source| CliDiagnostic::parse_error("--linter-enabled", source))?;

    let organize_imports_enabled = session
        .args
        .opt_value_from_str("--organize-imports-enabled")
        .map_err(|source| CliDiagnostic::parse_error("--organize-imports-enabled", source))?;

    let formatter = configuration
        .formatter_configuration
        .get_or_insert_with(FormatterConfiguration::default);

    if let Some(formatter_enabled) = formatter_enabled {
        formatter.enabled = formatter_enabled;
    }

    let linter = configuration
        .linter_configuration
        .get_or_insert_with(LinterConfiguration::default);

    if let Some(linter_enabled) = linter_enabled {
        linter.enabled = linter_enabled;
    }

    let organize_imports = configuration
        .organize_imports
        .get_or_insert_with(OrganizeImports::default);

    if let Some(organize_imports_enabled) = organize_imports_enabled {
        organize_imports.enabled = organize_imports_enabled;
    }

    // no point in doing the traversal if all the checks have been disabled
    if configuration.is_formatter_disabled()
        && configuration.is_linter_disabled()
        && configuration.is_organize_imports_disabled()
    {
        return Err(CliDiagnostic::incompatible_end_configuration("Formatter, linter and organize imports are disabled, can't perform the command. This is probably and error."));
    }

    apply_files_settings_from_cli(&mut session, &mut configuration)?;
    if !configuration.is_formatter_disabled() {
        apply_format_settings_from_cli(&mut session, &mut configuration)?;
    }

    apply_vcs_settings_from_cli(&mut session, &mut configuration)?;

    // check if support of git ignore files is enabled
    let vcs_base_path = configuration_path.or(session.app.fs.working_directory());
    store_path_to_ignore_from_vcs(&mut session, &mut configuration, vcs_base_path)?;

    session
        .app
        .workspace
        .update_settings(UpdateSettingsParams { configuration })?;

    execute_mode(Execution::new(TraversalMode::CI), session)
}
