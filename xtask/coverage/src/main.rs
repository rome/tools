use pico_args::Arguments;
use xtask::{project_root, pushd, Result};

use xtask_coverage::{compare::coverage_compare, run, SummaryDetailLevel};

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
    compare             Compares output between two --json outputs
OPTIONS
    --markdown          Emits supported output into markdown format. Supported by `compare` subcommand
    --json              Prints the test results in JSON. This mode will send all other test output and user messages to stderr.
    --detailed=[rast]   Prints a detailed summary at the end for all failing tests. Includes the RAST output if `rast` is passed.
    --language=[js|ts]  Runs a specific test suite
    --filter=<file>     Filters out tests that don't match the query
    --help              Prints this help
			"
        );
        return Ok(());
    }

    let json = args.contains("--json");
    let language: Option<String> = args.opt_value_from_str("--language").unwrap();
    let filter: Option<String> = args.opt_value_from_str("--filter").unwrap();

    let detail_level: Option<SummaryDetailLevel> =
        args.opt_value_from_str("--detailed").unwrap_or_else(|_| {
            if args.contains("--detailed") {
                Some(SummaryDetailLevel::Failing)
            } else {
                Some(SummaryDetailLevel::Coverage)
            }
        });

    run(
        language.as_deref(),
        filter.as_deref(),
        json,
        detail_level.unwrap_or(SummaryDetailLevel::Coverage),
    );

    Ok(())
}
