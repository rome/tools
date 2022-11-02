use crate::{CliSession, Termination};
use rome_console::{markup, ConsoleExt, HorizontalLine};
use rome_service::configuration::Configuration;
use rome_service::create_config;

pub(crate) fn init(mut session: CliSession) -> Result<(), Termination> {
    let fs = &mut session.app.fs;
    create_config(fs, Configuration::default())?;

    session.app.console.log(markup! {
"\n"<Inverse>"Welcome to Rome! Let's get you started..."</Inverse>"

"<Info><Emphasis>"Files created "</Emphasis></Info>{HorizontalLine::new(136)}"

  "<Dim>"- "</Dim><Emphasis>"rome.json: "</Emphasis>"Your project configuration. Documentation: https://rome.tools/configuration

"<Info><Emphasis>"Next Steps "</Emphasis></Info>{HorizontalLine::new(139)}"

  "<Dim>"1."</Dim>" "<Emphasis>"Setup an editor extension"</Emphasis>"
     Get live errors as you type and format when you save. Learn more: https://rome.tools/editors

  "<Dim>"2."</Dim>" "<Emphasis>"Try a command"</Emphasis>"
     "<Italic>"rome ci"</Italic>" checks for lint errors and verifies formatting. Run " <Italic>"rome --help"</Italic>" for a full list of commands and options.

  "<Dim>"3."</Dim>" "<Emphasis>"Read the documentation"</Emphasis>"
     Our website serves as a comprehensive source of guides and documentation: https://rome.tools/docs

  "<Dim>"4."</Dim>" "<Emphasis>"Get involved in the community"</Emphasis>"
     Ask questions, get support, or contribute by participating on GitHub (https://github.com/rome/tools), or join our community Discord (https://discord.gg/rome)"
        });

    Ok(())
}
