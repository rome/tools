use pico_args::Arguments;
use xtask::{project_root, pushd, Result};

use xtask_coverage::{compare::coverage_compare, run};

fn main() -> Result<()> {
    let _d = pushd(project_root());

    let mut args = Arguments::from_env();
    let sub_command = args.subcommand()?;

    if sub_command.as_deref() == Some("compare") {
        // on pr branch, run
        // git checkout main
        // cargo coverage js --json > base_results.json
        // git checkout -
        // cargo coverage js --json > new_results.json
        // cargo coverage compare ./base_results.json ./new_results.json --markdown
        let markdown = args.contains("--markdown");
        let free = args.free()?;
        let base_result_path = free.get(0).map(String::as_str);
        let new_result_path = free.get(1).map(String::as_str);
        coverage_compare(base_result_path, new_result_path, markdown);
        return Ok(());
    }

    if args.contains("--help") {
        eprintln!(
            "\
cargo coverage
Run coverage command.
USAGE:
	cargo coverage <SUBCOMMAND> [option]
SUBCOMMANDS:
	compare      		Compares output between two --json outputs
OPTIONS
	--markdown   		Emits supported output into markdown format. Supported by compare subcommand
	--json       		Emits supported output into json format. Supported by js subcommand
	--language=[js|ts]  Runs a specific test suite
	--help				Prints this help
			"
        );
        return Ok(());
    }

    let json = args.contains("--json");
    let show_rast = args.contains("--show-rast");
    let show_diagnostics = args.contains("--show-diagnostics");
    let language: Option<String> = args.opt_value_from_str("--language").unwrap();
    let free = args.free()?;
    let query = free.get(0).map(String::as_str);
    run(
        language.as_deref(),
        query,
        json,
        show_rast,
        show_diagnostics,
    );

    Ok(())
}
