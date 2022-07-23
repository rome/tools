//! This is the main binary of Rome.
//!
//! If you're curios about how to use it, check Rome's [website]
//!
//! [website]: https://rome.tools

use rome_cli::{run_cli, setup_panic_handler, CliSession, Termination};

#[cfg(target_os = "windows")]
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

///
/// To run this example, run:
///
/// ```bash
/// cargo run --example run_cli
/// ```
/// Add arguments like:
///
/// ```bash
///   cargo run --example run_cli -- --help
///   cargo run --example run_cli format
/// ```
///
/// To run a valid example:
///
/// ```bash
///  cargo run --example run_cli format examples/input.json
/// ```
fn main() -> Result<(), Termination> {
    setup_panic_handler();
    run_cli(CliSession::from_env())
}
