use crate::{
    traversal::{traverse, TraversalMode},
    CliSession, Termination,
};

/// Handler for the "fix" command of the Rome CLI
pub(crate) fn fix(session: CliSession) -> Result<(), Termination> {
    traverse(TraversalMode::Fix, session)
}
