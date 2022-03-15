use rome_cli::{run_cli, CliSession, Termination};

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
/// or
///
/// ```bash
/// cargo run --example run_cli format examples/input.js
/// ```
fn main() -> Result<(), Termination> {
    run_cli(CliSession::from_env())
}
