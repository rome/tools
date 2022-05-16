use crate::{
    traversal::{traverse, TraversalMode},
    CliSession, Termination,
};

use super::format::parse_format_options;

/// Handler for the "ci" command of the Rome CLI
pub(crate) fn ci(mut session: CliSession) -> Result<(), Termination> {
    let options = parse_format_options(&mut session)?;
    traverse(TraversalMode::CI { options }, session)
}
