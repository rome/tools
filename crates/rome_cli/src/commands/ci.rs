use crate::parse_arguments::{apply_files_settings_from_cli, apply_format_settings_from_cli};
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
    let (mut configuration, diagnostics) = load_configuration(&mut session)?.consume();

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
        .formatter
        .get_or_insert_with(FormatterConfiguration::default);

    if let Some(formatter_enabled) = formatter_enabled {
        formatter.enabled = formatter_enabled;
    }

    let linter = configuration
        .linter
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
    if configuration.is_formatter_disabled() && configuration.is_linter_disabled() {
        return Err(CliDiagnostic::incompatible_end_configuration("Formatter and Linter are both disabled, can't perform the command. This is probably and error."));
    }

    apply_files_settings_from_cli(&mut session, &mut configuration)?;
    if !configuration.is_formatter_disabled() {
        apply_format_settings_from_cli(&mut session, &mut configuration)?;
    }

    session
        .app
        .workspace
        .update_settings(UpdateSettingsParams { configuration })?;

    execute_mode(Execution::new(TraversalMode::CI), session)
}
