//! This is the main binary of Rome.
//!
//! If you're curious about how to use it, check Rome's [website]
//!
//! [website]: https://rome.tools

use rome_cli::{
    color_from_arguments, open_transport, setup_panic_handler, Arguments, CliSession,
    TerminationDiagnostic,
};
use rome_console::{markup, Console, ConsoleExt, EnvConsole};
use rome_diagnostics::{set_bottom_frame, DiagnosticExt, PrintDiagnostic};
use rome_service::workspace;
use std::env;
use std::process::{exit, ExitCode, Termination};
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

    let mut args = Arguments::from_env();
    let colors = match color_from_arguments(&mut args) {
        Ok(colors) => colors,
        Err(err) => {
            eprintln!("{:?}", err);
            exit(1)
        }
    };
    let mut console = EnvConsole::new(colors);
    let result = run_workspace(args, &mut console);
    match result {
        Err(termination) => {
            let args: String = env::args().collect();
            let error = termination.with_file_source_code(args);
            console.error(markup! {
                {PrintDiagnostic::verbose(&error)}
            });
            error.report()
        }
        Ok(_) => exit(0),
    }
}

fn run_workspace(
    mut args: Arguments,
    console: &mut impl Console,
) -> Result<(), TerminationDiagnostic> {
    // If the `--use-server` CLI flag is set, try to open a connection to an
    // existing Rome server socket
    let workspace = if args.contains("--use-server") {
        let runtime = Runtime::new()?;
        match open_transport(runtime)? {
            Some(transport) => workspace::client(transport)?,
            None => return Err(TerminationDiagnostic::server_not_running()),
        }
    } else {
        workspace::server()
    };

    let session = CliSession::new(&*workspace, args, console)?;
    session.run()
}
