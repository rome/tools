use crate::{
    traversal::{traverse, TraversalMode},
    CliSession, Termination,
};

/// Handler for the "check" command of the Rome CLI
pub(crate) fn check(mut session: CliSession) -> Result<(), Termination> {
    let mode = if session.args.contains("--apply") {
        TraversalMode::Fix
    } else {
        TraversalMode::Check
    };

    traverse(mode, session)
}
