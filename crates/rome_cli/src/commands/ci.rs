use crate::{
    configuration::load_configuration, execute_mode, CliSession, Execution, Termination,
    TraversalMode,
};
use rome_service::configuration::{FormatterConfiguration, LinterConfiguration};
use rome_service::workspace::UpdateSettingsParams;

use super::format::apply_format_settings_from_cli;

/// Handler for the "ci" command of the Rome CLI
pub(crate) fn ci(mut session: CliSession) -> Result<(), Termination> {
    let mut configuration = load_configuration(&mut session)?;

    let formatter_enabled = session
        .args
        .opt_value_from_str("--formatter-enabled")
        .map_err(|source| Termination::ParseError {
            argument: "--formatter-enabled",
            source,
        })?;

    let linter_enabled = session
        .args
        .opt_value_from_str("--linter-enabled")
        .map_err(|source| Termination::ParseError {
            argument: "--linter-enabled",
            source,
        })?;

    let formatter = configuration
        .formatter
        .get_or_insert_with(FormatterConfiguration::default);

    formatter.enabled = formatter_enabled.unwrap_or(true);

    let linter = configuration
        .linter
        .get_or_insert_with(LinterConfiguration::default);

    linter.enabled = linter_enabled.unwrap_or(true);

    // no point in doing the traversal if all the checks have been disabled
    if configuration.is_formatter_disabled() && configuration.is_linter_disabled() {
        return Err(Termination::IncompatibleEndConfiguration("Formatter and Linter are both disabled, can't perform the command. This probably and error."));
    }

    if !configuration.is_formatter_disabled() {
        apply_format_settings_from_cli(&mut session, &mut configuration)?;
    }

    session
        .app
        .workspace
        .update_settings(UpdateSettingsParams { configuration })?;

    execute_mode(Execution::new(TraversalMode::CI), session)
}
