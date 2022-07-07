use crate::{
    traversal::{traverse, TraversalMode},
    CliSession, Termination,
};
use rome_diagnostics::MAXIMUM_DISPLAYABLE_DIAGNOSTICS;

/// Handler for the "check" command of the Rome CLI
pub(crate) fn check(mut session: CliSession) -> Result<(), Termination> {
    let mode = if session.args.contains("--apply") {
        TraversalMode::Fix
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

        TraversalMode::Check { max_diagnostics }
    };

    traverse(mode, session)
}
