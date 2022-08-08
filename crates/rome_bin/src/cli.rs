use rome_cli::{run_cli, setup_panic_handler, Arguments, CliSession, Termination};
use rome_service::workspace;
use tokio::runtime::Runtime;

use crate::service::open_transport;

pub fn run_cli_session(args: Arguments) -> Result<(), Termination> {
    setup_panic_handler();

    // Try to open a connection to an existing Rome server socket, or create an
    // in-process Workspace server instance if no daemon process is found
    let runtime = Runtime::new().expect("could not instantiate the Tokio runtime");
    let workspace = match open_transport(runtime)? {
        Some(transport) => workspace::client(transport)?,
        None => workspace::server(),
    };

    let session = CliSession::new(&*workspace, args);
    run_cli(session)
}
