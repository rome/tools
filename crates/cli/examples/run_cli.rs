use cli::run_cli;

///
/// To run this example, run:
///
///	```bash
/// cargo run --example run_cli
/// ```
/// Add arguments like:
///
/// ```bash
/// 	cargo run --example run_cli -- --help
/// 	cargo run --example run_cli format
/// ```
///
///	To run a valid example:
///
///	```bash
///		cargo run --example run_cli format examples/input.jso
///	```
///
fn main() {
	run_cli();
}
