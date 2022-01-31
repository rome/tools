use pico_args::Arguments;
use std::str::FromStr;
use xtask::{project_root, pushd, Result};
use xtask_bench::{run, FeatureToBenchmark};

#[cfg(feature = "dhat-on")]
use dhat::DhatAlloc;

#[cfg(feature = "dhat-on")]
#[global_allocator]
static ALLOCATOR: DhatAlloc = DhatAlloc;

fn main() -> Result<()> {
    #[cfg(feature = "dhat-on")]
    let dhat = dhat::Dhat::start_heap_profiling();

    let _d = pushd(project_root());
    let mut args = Arguments::from_env();

    // on pr branch, run
    // git checkout main
    // cargo benchmark --save-baseline main
    // git checkout -
    // cargo benchmark --save-baseline pr
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
    let feature: Option<String> = args.opt_value_from_str("--feature").unwrap();

    run(
        filter,
        criterion,
        baseline,
        FeatureToBenchmark::from_str(feature.unwrap_or_else(|| "parser".to_string()).as_str())
            .unwrap(),
    );

    Ok(())
}
