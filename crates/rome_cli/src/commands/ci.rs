use crate::{
    traversal::{traverse, TraversalMode},
    CliSession, Termination,
};
use rome_service::load_config;
use rome_service::load_config::ConfigurationType;

use super::format::parse_format_options;

/// Handler for the "ci" command of the Rome CLI
pub(crate) fn ci(mut session: CliSession) -> Result<(), Termination> {
    let configuration = load_config(&session.app.fs, ConfigurationType::Root)?;

    parse_format_options(&mut session, configuration)?;
    traverse(TraversalMode::CI, session)
}
