//! This is the main binary of Rome.
//!
//! If you're curious about how to use it, check Rome's [website]
//!
//! [website]: https://rome.tools

use bpaf::{Args, ParseFailure};
use rome_cli::{
    open_transport, parse_command, setup_panic_handler, to_color_mode, CliDiagnostic, CliSession,
    RomeCommand,
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

    let mut console = EnvConsole::default();
    let command = parse_command().run_inner(Args::current_args());
    match command {
        Ok(command) => {
            let color_mode = to_color_mode(command.get_color());
            console.set_color(color_mode);
            let result = run_workspace(&mut console, command);
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
        Err(failure) => {
            return if let ParseFailure::Stdout(help) = &failure {
                console.log(markup! {{help}});
                ExitCode::SUCCESS
            } else {
                let diagnostic = CliDiagnostic::parse_error_bpaf(failure);
                console.error(markup! { {PrintDiagnostic::simple(&diagnostic)}});
                ExitCode::FAILURE
            }
        }
    }
}

fn run_workspace(console: &mut EnvConsole, command: RomeCommand) -> Result<(), CliDiagnostic> {
    // If the `--use-server` CLI flag is set, try to open a connection to an
    // existing Rome server socket
    let workspace = if command.should_use_server() {
        let runtime = Runtime::new()?;
        match open_transport(runtime)? {
            Some(transport) => workspace::client(transport)?,
            None => return Err(CliDiagnostic::server_not_running()),
        }
    } else {
        workspace::server()
    };

    let session = CliSession::new(&*workspace, console)?;
    session.run(command)
}
