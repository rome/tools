use crate::{
    traversal::{traverse, TraversalMode},
    CliSession, Termination,
};
use rome_diagnostics::MAXIMUM_DISPLAYABLE_DIAGNOSTICS;
use rome_service::load_config;
use rome_service::load_config::ConfigurationType;

/// Handler for the "check" command of the Rome CLI
pub(crate) fn check(mut session: CliSession) -> Result<(), Termination> {
    let configuration = load_config(&session.app.fs, ConfigurationType::Root)?;
    let formatter_disabled = configuration
        .as_ref()
        .map_or(false, |c| c.is_formatter_disabled());
    let linter_disabled = configuration
        .as_ref()
        .map_or(false, |c| c.is_linter_disabled());

    let mode = if session.args.contains("--apply") {
        TraversalMode::Fix {
            linter_disabled,
            formatter_disabled,
        }
    } else {
        let max_diagnostics: Option<u8> = session
            .args
            .opt_value_from_str("--max-diagnostics")
            .map_err(|source| Termination::ParseError {
                argument: "--max-diagnostics",
                source,
            })?;
        let max_diagnostics = if let Some(max_diagnostics) = max_diagnostics {
            if max_diagnostics > MAXIMUM_DISPLAYABLE_DIAGNOSTICS {
                return Err(Termination::OverflowNumberArgument(
                    "--max-diagnostics",
                    "50",
                ));
            }

            max_diagnostics
        } else {
            // default value
            20
        };

        TraversalMode::Check {
            max_diagnostics,
            linter_disabled,
            formatter_disabled,
        }
    };

    traverse(mode, session)
}
