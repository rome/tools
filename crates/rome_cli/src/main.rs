//! This is the main binary of Rome.
//!
//! If you're curious about how to use it, check Rome's [website]
//!
//! [website]: https://rome.tools

use rome_cli::{
    color_from_arguments, open_transport, setup_panic_handler, Arguments, CliDiagnostic, CliSession,
};
use rome_console::{markup, ConsoleExt, EnvConsole};
use rome_diagnostics::{set_bottom_frame, PrintDiagnostic};
use rome_service::workspace;
use std::process::{ExitCode, Termination};
use tokio::runtime::Runtime;

#[cfg(target_os = "windows")]
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

#[cfg(not(target_os = "windows"))]
#[global_allocator]
static GLOBAL: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;

fn main() -> ExitCode {
    setup_panic_handler();
    set_bottom_frame(main as usize);

    let args = Arguments::from_env();
    let mut console = EnvConsole::default();

    let result = run_workspace(args, &mut console);
    match result {
        Err(termination) => {
            console.error(markup! {
                {PrintDiagnostic::verbose(&termination)}
            });
            termination.report()
        }
        Ok(_) => ExitCode::SUCCESS,
    }
}

fn run_workspace(mut args: Arguments, console: &mut EnvConsole) -> Result<(), CliDiagnostic> {
    let colors = color_from_arguments(&mut args)?;
    console.set_color(colors);
    // If the `--use-server` CLI flag is set, try to open a connection to an
    // existing Rome server socket
    let workspace = if args.contains("--use-server") {
        let runtime = Runtime::new()?;
        match open_transport(runtime)? {
            Some(transport) => workspace::client(transport)?,
            None => return Err(CliDiagnostic::server_not_running()),
        }
    } else {
        workspace::server()
    };

    let session = CliSession::new(&*workspace, args, console)?;
    session.run()
}
