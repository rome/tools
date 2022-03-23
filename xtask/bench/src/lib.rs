mod features;
mod utils;

use rome_js_parser::{parse, SourceType};
use std::collections::HashMap;
use std::convert::TryInto;
use std::fmt::{Display, Formatter};
use std::path::Path;
use std::str::FromStr;
use std::time::Duration;

pub use crate::features::formatter::benchmark_format_lib;
use crate::features::formatter::{run_format, FormatterMeasurement};
pub use crate::features::parser::benchmark_parse_lib;
use crate::features::parser::{run_parse, ParseMeasurement};
pub use utils::get_code;

/// What feature to benchmark
pub enum FeatureToBenchmark {
    /// benchmark of the parser
    Parser,
    /// benchmark of the formatter
    Formatter,
}

impl FromStr for FeatureToBenchmark {
    type Err = pico_args::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "parser" => Ok(Self::Parser),
            "formatter" => Ok(Self::Formatter),
            _ => Err(pico_args::Error::OptionWithoutAValue("feature")),
        }
    }
}

impl Display for FeatureToBenchmark {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            FeatureToBenchmark::Parser => write!(f, "parser"),
            FeatureToBenchmark::Formatter => write!(f, "formatter"),
        }
    }
}

/// If groups the summary by their category and creates a small interface
/// where each bench result can create their summary
pub enum BenchmarkSummary {
    Parser(ParseMeasurement),
    Formatter(FormatterMeasurement),
}

impl BenchmarkSummary {
    pub fn summary(&self) -> String {
        match self {
            BenchmarkSummary::Parser(result) => result.summary(),
            BenchmarkSummary::Formatter(result) => result.summary(),
        }
    }
}

impl Display for BenchmarkSummary {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            BenchmarkSummary::Parser(result) => std::fmt::Display::fmt(&result, f),
            BenchmarkSummary::Formatter(result) => std::fmt::Display::fmt(&result, f),
        }
    }
}

fn err_to_string<E: std::fmt::Debug>(e: E) -> String {
    format!("{:?}", e)
}

pub struct RunArgs {
    pub filter: String,
    pub criterion: bool,
    pub baseline: Option<String>,
    pub feature: FeatureToBenchmark,
    pub suites: String,
}

pub fn run(args: RunArgs) {
    let regex = regex::Regex::new(args.filter.as_str()).unwrap();

    let mut all_suites = HashMap::new();
    all_suites.insert("js", include_str!("libs-js.txt"));
    all_suites.insert("ts", include_str!("libs-ts.txt"));

    let mut libs = vec![];
    let suites_to_run = args.suites.split(',');
    for suite in suites_to_run {
        match suite {
            "*" => {
                libs.extend(all_suites["js"].lines());
                libs.extend(all_suites["ts"].lines());
            }
            "js" => libs.extend(all_suites["js"].lines()),
            "ts" => libs.extend(all_suites["ts"].lines()),
            unknown => {
                eprintln!("Unknown suite: {}", unknown);
            }
        }
    }

    let mut summary = vec![];

    for lib in libs {
        if !regex.is_match(lib) {
            continue;
        }

        let code = get_code(lib);

        match code {
            Ok((id, code)) => {
                let code = code.as_str();

                let source_type: SourceType = Path::new(&id).try_into().unwrap();

                // Do all steps with criterion now
                if args.criterion {
                    let mut criterion = criterion::Criterion::default()
                        .without_plots()
                        .measurement_time(Duration::new(10, 0));
                    if let Some(ref baseline) = args.baseline {
                        criterion = criterion.save_baseline(baseline.to_string());
                    }
                    let mut group = criterion.benchmark_group(args.feature.to_string());
                    group.throughput(criterion::Throughput::Bytes(code.len() as u64));

                    group.bench_function(&id, |b| match args.feature {
                        FeatureToBenchmark::Parser => b.iter(|| {
                            criterion::black_box(run_parse(code, source_type.clone()));
                        }),
                        FeatureToBenchmark::Formatter => {
                            let root = parse(code, 0, source_type.clone()).syntax();
                            b.iter(|| {
                                criterion::black_box(run_format(&root));
                            })
                        }
                    });
                    group.finish();
                } else {
                    //warmup
                    match args.feature {
                        FeatureToBenchmark::Parser => {
                            run_parse(code, source_type.clone());
                        }
                        FeatureToBenchmark::Formatter => {
                            let root = parse(code, 0, source_type.clone()).syntax();
                            run_format(&root);
                        }
                    }
                }

                let result = match args.feature {
                    FeatureToBenchmark::Parser => benchmark_parse_lib(&id, code, source_type),
                    FeatureToBenchmark::Formatter => {
                        let root = parse(code, 0, source_type).syntax();
                        benchmark_format_lib(&id, &root)
                    }
                };

                summary.push(result.summary());

                println!("Benchmark: {}", lib);
                println!("{}", result);
            }
            Err(e) => println!("{:?}", e),
        }
    }

    println!("Summary");
    println!("-------");
    for l in summary {
        println!("{}", l);
    }
}
