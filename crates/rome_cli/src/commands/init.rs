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


    <Info><Emphasis>"

What's next?
    
"</Emphasis></Info>
"   
    1. "<Emphasis>"Setup an editor extension      
"</Emphasis>
    "       Get live errors as you type and format when you save. Learn more:"<Underline>"https://rome.tools/#install-official-rome-vs-code-extension"</Underline>
"   
    2. "<Emphasis>"Try a command
"</Emphasis>
        <Italic>"      rome check"</Italic>" is used to validate your code, verify formatting, and check for lint errors. Run " <Italic>"rome --help"</Italic>" for a full list of commands and flags"
"   
    3. "<Emphasis>"Read documentation 
"</Emphasis>
        "       Our website serves as a comprehensive source of guides and documentation: "<Underline>"https://rome.tools/"</Underline>
"
    4. "<Emphasis>"Get involved in the community
"</Emphasis>
        "       Ask questions, get support, or contribute by participating on GitHub ("<Underline>"https://github.com/rome/tools"</Underline>"), or our community Discord ("<Underline>"https://discord.gg/rome"</Underline>")"

                };

    session.app.console.log(message);

    Ok(())
}
