use rome_cli::{run_cli, setup_panic_handler, CliSession, Termination};

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
///
fn main() -> Result<(), Termination> {
    setup_panic_handler();
    run_cli(CliSession::from_env())
}
