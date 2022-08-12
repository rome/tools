//! This is the main binary of Rome.
//!
//! If you're curios about how to use it, check Rome's [website]
//!
//! [website]: https://rome.tools

use cli::run_cli_session;
use rome_cli::{Arguments, Termination};
use server::{print_server_socket, run_server_session};

mod cli;
mod server;
mod service;

#[cfg(target_os = "windows")]
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

fn main() -> Result<(), Termination> {
    let mut args = Arguments::from_env();

    if args.contains("__print_socket") {
        print_server_socket()
    } else if args.contains("__run_server") {
        run_server_session()
    } else {
        run_cli_session(args)
    }
}
