use crate::{
    traversal::{traverse, TraversalMode},
    CliSession, Termination,
};

/// Handler for the "check" command of the Rome CLI
pub(crate) fn check(session: CliSession) -> Result<(), Termination> {
    traverse(TraversalMode::Check, session)
}
