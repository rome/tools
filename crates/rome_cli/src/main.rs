//! This is the main binary of Rome.
//!
//! If you're curious about how to use it, check Rome's [website]
//!
//! [website]: https://rome.tools

use rome_cli::{open_transport, setup_panic_handler, Arguments, CliSession, Termination};
use rome_diagnostics::v2::set_bottom_frame;
use rome_service::workspace;
use tokio::runtime::Runtime;

#[cfg(target_os = "windows")]
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

#[cfg(not(target_os = "windows"))]
#[global_allocator]
static GLOBAL: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;

fn main() -> Result<(), Termination> {
    setup_panic_handler();
    set_bottom_frame(main as usize);

    let mut args = Arguments::from_vec(vec!["check".into(), "/Users/gurwindersi/x/a.js".into()]);

    // If the `--use-server` CLI flag is set, try to open a connection to an
    // existing Rome server socket
    let workspace = if args.contains("--use-server") {
        let runtime = Runtime::new()?;
        match open_transport(runtime)? {
            Some(transport) => workspace::client(transport)?,
            None => return Err(Termination::ServerNotRunning),
        }
    } else {
        workspace::server()
    };

    CliSession::new(&*workspace, args).run()
}
