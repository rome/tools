mod features;
mod utils;

use std::fmt::{Display, Formatter};
use std::str::FromStr;
use std::time::Duration;

pub use crate::features::formatter::benchmark_format_lib;
use crate::features::formatter::BenchmarkFormatterResult;
pub use crate::features::parser::benchmark_parse_lib;
use crate::features::parser::BenchmarkParseResult;
pub use utils::get_code;

/// What feature to benchmark
pub enum FeatureToBenchmark {
    /// benchmark of the parser
    Parser,
    /// benchmark of the formatter
    Formatter,
}

impl FromStr for FeatureToBenchmark {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "parser" => Ok(Self::Parser),
            "formatter" => Ok(Self::Formatter),
            _ => Ok(Self::Parser),
        }
    }
}

impl Display for FeatureToBenchmark {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let _ = match self {
            FeatureToBenchmark::Parser => writeln!(f, "parser"),
            FeatureToBenchmark::Formatter => writeln!(f, "formatter"),
        };
        Ok(())
    }
}

/// If groups the summary by their category and creates a small interface
/// where each bench result can create their summary
pub enum BenchSummary {
    Parser(BenchmarkParseResult),
    Formatter(BenchmarkFormatterResult),
}

impl BenchSummary {
    pub fn summary(&self) -> String {
        match self {
            BenchSummary::Parser(result) => result.summary(),
            BenchSummary::Formatter(result) => result.summary(),
        }
    }
}

impl Display for BenchSummary {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            BenchSummary::Parser(result) => std::fmt::Display::fmt(&result, f),
            BenchSummary::Formatter(result) => std::fmt::Display::fmt(&result, f),
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
                    let mut group = criterion.benchmark_group("parser");
                    group.throughput(criterion::Throughput::Bytes(code.len() as u64));
                    group.bench_function(&id, |b| {
                        b.iter(|| {
                            let _ = criterion::black_box(rslint_parser::parse_module(code, 0));
                        })
                    });
                    group.finish();
                } else {
                    //warmup
                    rslint_parser::parse_module(code, 0);
                }

                let result = match feature {
                    FeatureToBenchmark::Parser => benchmark_parse_lib(&id, code),
                    FeatureToBenchmark::Formatter => benchmark_format_lib(&id, code),
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
