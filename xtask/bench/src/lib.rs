mod features;
mod utils;

use rslint_parser::{parse, Syntax};
use std::fmt::{Display, Formatter};
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

pub fn run(filter: String, criterion: bool, baseline: Option<String>, feature: FeatureToBenchmark) {
    let regex = regex::Regex::new(filter.as_str()).unwrap();
    let libs = include_str!("libs.txt").lines();

    let mut summary = vec![];

    for lib in libs {
        if !regex.is_match(lib) {
            continue;
        }

        let code = get_code(lib);

        match code {
            Ok((id, code)) => {
                let code = code.as_str();

                // Do all steps with criterion now
                if criterion {
                    let mut criterion = criterion::Criterion::default()
                        .without_plots()
                        .measurement_time(Duration::new(10, 0));
                    if let Some(ref baseline) = baseline {
                        criterion = criterion.save_baseline(baseline.to_string());
                    }
                    let mut group = criterion.benchmark_group(feature.to_string());
                    group.throughput(criterion::Throughput::Bytes(code.len() as u64));
                    group.bench_function(&id, |b| match feature {
                        FeatureToBenchmark::Parser => b.iter(|| {
                            criterion::black_box(run_parse(code));
                        }),
                        FeatureToBenchmark::Formatter => {
                            let syntax = Syntax::default().module();
                            let root = parse(code, 0, syntax).syntax();
                            b.iter(|| {
                                criterion::black_box(run_format(&root));
                            })
                        }
                    });
                    group.finish();
                } else {
                    //warmup
                    match feature {
                        FeatureToBenchmark::Parser => {
                            run_parse(code);
                        }
                        FeatureToBenchmark::Formatter => {
                            let syntax = Syntax::default().module();
                            let root = parse(code, 0, syntax).syntax();
                            run_format(&root);
                        }
                    }
                }

                let result = match feature {
                    FeatureToBenchmark::Parser => benchmark_parse_lib(&id, code),
                    FeatureToBenchmark::Formatter => {
                        let syntax = Syntax::default().module();
                        let root = parse(code, 0, syntax).syntax();
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
