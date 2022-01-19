use pico_args::Arguments;
use xtask::{
    codegen::{self, Mode},
    compare, coverage,
    glue::pushd,
    project_root, run_rustfmt, unicode, Result,
};

#[cfg(feature = "dhat-on")]
use dhat::DhatAlloc;

#[cfg(feature = "dhat-on")]
#[global_allocator]
static ALLOCATOR: DhatAlloc = DhatAlloc;

fn main() -> Result<()> {
    #[cfg(feature = "dhat-on")]
    let _dhat = dhat::Dhat::start_heap_profiling();

    let _d = pushd(project_root());

    let mut args = Arguments::from_env();
    let subcommand = args.subcommand()?.unwrap_or_default();
    match subcommand.as_str() {
        "codegen" => {
            args.finish()?;
            codegen::generate_parser_tests(Mode::Overwrite)?;
            Ok(())
        }
        "syntax" => {
            args.finish()?;
            codegen::generate_ast(Mode::Overwrite)?;
            Ok(())
        }
        "format" => {
            args.finish()?;
            run_rustfmt(Mode::Overwrite)
        }
        "compare" => {
            let markdown = args.contains("--markdown");
            let free = args.free()?;
            let base_result_path = free.get(0).map(String::as_str);
            let new_result_path = free.get(1).map(String::as_str);

            compare::coverage_compare(base_result_path, new_result_path, markdown);
            Ok(())
        }
        // "docgen" => {
        //     args.finish()?;
        //     docgen::run();
        //     Ok(())
        // }
        "coverage" => {
            let json = args.contains("--json");
            let show_rast = args.contains("--show-rast");
            let show_diagnostics = args.contains("--show-diagnostics");

            let free = args.free()?;
            let query = free.get(0).map(String::as_str);

            let pool = yastl::ThreadConfig::new().stack_size(8 << 30);
            coverage::run(
                query,
                yastl::Pool::with_config(num_cpus::get(), pool),
                json,
                show_rast,
                show_diagnostics,
            );
            Ok(())
        }
        "coverage-libs" => {
            // on pr branch, run
            // git checkout main
            // cargo run -p xtask --release -- coverage-libs --save-baseline main
            // git checkout -
            // cargo run -p xtask --release -- coverage-libs --save-baseline pr
            // critcmp main pr # (cargo install critcmp)
            let filter: String = args
                .opt_value_from_str("--filter")
                .unwrap()
                .unwrap_or_else(|| ".*".to_string());
            let criterion: bool = args
                .opt_value_from_str("--criterion")
                .unwrap()
                .unwrap_or(true);
            let baseline: Option<String> = args.opt_value_from_str("--save-baseline").unwrap();
            xtask::libs::run(filter, criterion, baseline);
            Ok(())
        }
<<<<<<< HEAD
        "unicode" => {
            args.finish()?;
            unicode::generate_tables()
        }
        _ => {
            eprintln!(
                "\
>>>>>>> f7d668584 (multiple dianostics on invalid string lietral)
=======
        _ => {
            eprintln!(
                "\
>>>>>>> 36359a9ad (validate unicode escape on string literals)
>>>>>>> af5c25965 (removing validation of escaped identifiers)
cargo xtask
Run custom build command.
USAGE:
    cargo xtask <SUBCOMMAND> [option]
SUBCOMMANDS:
    format
    codegen
    syntax
    docgen
    coverage [--json] [--show-diagnostics] [--show-rast]
    coverage-libs
    compare [--markdown]
    unicode
OPTIONS
    --markdown   Emits supported output into markdown format. Supported by compare subcommand
    --json       Emits supported output into json format. Supported by coverage subcommand
            "
            );
            Ok(())
        }
    }
}
