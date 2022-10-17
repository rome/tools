use rome_console::{markup, ConsoleExt};

use crate::{CliSession, Termination, VERSION};

/// Handler for the `--version` argument. Prints a brief rome version.
pub(crate) fn brief_version(mut session: CliSession) -> Result<(), Termination> {
    session
        .app
        .console
        .log(markup!("Rome CLI version "{VERSION}));

    Ok(())
}

/// Handle of the `version` command. Prints a more in detail version of rome.
pub(crate) fn full_version(mut session: CliSession) -> Result<(), Termination> {
    session.app.console.log(markup! {
    "CLI:        "{VERSION}
    });

    match session.app.workspace.server_info() {
        None => {
            session.app.console.log(markup! {
                "Server:     "<Dim>"not connected"</Dim>
            });
        }
        Some(info) => {
            let version = info.version.as_deref().unwrap_or("-");

            session.app.console.log(markup! {
"Server:
  Name:     "{info.name}"
  Version:  "{version}
            });
        }
    };

    Ok(())
}
