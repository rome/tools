use crate::{
    traversal::{traverse, TraversalMode},
    CliSession, Termination,
};
use rome_service::load_config;
use rome_service::load_config::ConfigurationType;

use super::format::parse_format_options;

/// Handler for the "ci" command of the Rome CLI
pub(crate) fn ci(mut session: CliSession) -> Result<(), Termination> {
    // reading the configuration should not cause an error, rome should working even without it
    let configuration_path = session.app.fs.config_path();
    let configuration = if let Some(configuration_path) = configuration_path {
        load_config(&configuration_path, ConfigurationType::Root)?
    } else {
        None
    };
    parse_format_options(&mut session, configuration)?;
    traverse(TraversalMode::CI, session)
}
