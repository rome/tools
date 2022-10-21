use crate::{CliSession, Termination};
use rome_console::{markup, ConsoleExt};
use rome_service::configuration::Configuration;
use rome_service::create_config;

pub(crate) fn init(mut session: CliSession) -> Result<(), Termination> {
    let fs = &mut session.app.fs;
    create_config(fs, Configuration::default())?;
    let message = markup! {
                    <Info><Emphasis>"Files created:\n\n"</Emphasis></Info>
                    <Emphasis>"\t- rome.json: "</Emphasis>"Your project configuration. Documentation: "<Underline>"https://rome.tools/docs/#configuration \n\n"</Underline>
    <Info><Emphasis>"What's next?\n\n"</Emphasis></Info>
    "\t1. "<Emphasis>"Setup an editor extension\n"</Emphasis>
            "\t\tGet live errors as you type and format when you save. Learn more:"<Underline>"https://rome.tools/#editor-setup \n"</Underline>
    "\t2. "<Emphasis>"Try a command\n"</Emphasis>
            <Italic>"\t\trome ci"</Italic>" is used to validate your code, verify formatting, and check for lint errors. Run " <Italic>"rome --help"</Italic>" for a full list of commands and flags\n"
    "\t3. "<Emphasis>"Read documentation \n"</Emphasis>
            "\t\tOur website serves as a comprehensive source of guides and documentation: "<Underline>"https://rome.tools/docs \n"</Underline>
    "\t4. "<Emphasis>"Get involved in the community\n"</Emphasis>
            "\t\tAsk questions, get support, or contribute by participating on GitHub ("<Underline>"https://github.com/rome/tools"</Underline>"), or our community Discord ("<Underline>"https://discord.gg/rome"</Underline>")"

                    };

    session.app.console.log(message);

    Ok(())
}
