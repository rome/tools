use crate::{CliSession, Termination};
use rome_console::{markup, ConsoleExt};
use rome_service::configuration::Configuration;
use rome_service::create_config;

pub(crate) fn init(mut session: CliSession) -> Result<(), Termination> {
    let fs = &mut session.app.fs;
    create_config(fs, Configuration::default())?;
    let message = markup! {
    <Info><Emphasis>"Files created:
        
"</Emphasis></Info>
    <Emphasis>"- rome.json: "</Emphasis>"Your project configuration. Documentation: "<Underline>"https://rome.tools/#project-configuration"</Underline>
    };

    session.app.console.log(message);

    Ok(())
}
