use pico_args::Arguments;
use xtask::{project_root, pushd, Result};
use xtask_bench::{run, FeatureToBenchmark};

#[cfg(feature = "dhat-on")]
use dhat::DhatAlloc;

#[cfg(feature = "dhat-on")]
#[global_allocator]
static ALLOCATOR: DhatAlloc = DhatAlloc;

fn main() -> Result<(), pico_args::Error> {
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
    let suites = args
        .opt_value_from_str("--suites")
        .unwrap()
        .unwrap_or("*".to_string());

    let baseline: Option<String> = args.opt_value_from_str("--save-baseline").unwrap();
    // "feature" is a mandatory option and will throw an error if it's missing or incorrect
    let feature: FeatureToBenchmark = args.value_from_str("--feature")?;

    run(filter, criterion, baseline, feature, &suites);
    Ok(())
}
