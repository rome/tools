//! This is the main binary of Rome.
//!
//! If you're curious about how to use it, check Rome's [website]
//!
//! [website]: https://rome.tools

use rome_cli::{open_transport, setup_panic_handler, Arguments, CliSession, Termination};
use rome_service::workspace;
use tokio::runtime::Runtime;

#[cfg(target_os = "windows")]
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

fn main() -> Result<(), Termination> {
    setup_panic_handler();

    let args = Arguments::from_env();

    // Try to open a connection to an existing Rome server socket, or create an
    // in-process Workspace server instance if no daemon process is found
    let runtime = Runtime::new()?;
    let workspace = match open_transport(runtime)? {
        Some(transport) => workspace::client(transport)?,
        None => workspace::server(),
    };

    CliSession::new(&*workspace, args).run()
}
