mod features;

use rslint_parser::{parse, SourceType};
use serde::Serialize;
use std::convert::TryInto;
use std::fmt::{Display, Formatter};
use std::path::Path;
use std::str::FromStr;
use std::time::Duration;
use thiserror::Error;

pub use crate::features::formatter::benchmark_format_lib;
use crate::features::formatter::{run_format, FormatterMeasurement};
pub use crate::features::parser::benchmark_parse_lib;
use crate::features::parser::{run_parse, ParseMeasurement};

#[cfg(feature = "dhat-on")]
use dhat::DhatAlloc;

#[cfg(feature = "dhat-on")]
#[global_allocator]
static ALLOCATOR: DhatAlloc = DhatAlloc;

#[derive(Debug, PartialEq, Error)]
pub enum BenchmarkError {
    #[error("unknown feature to benchmark")]
    UnknownFeature,
}

/// What feature to benchmark
pub enum FeatureToBenchmark {
    /// benchmark of the parser
    Parser,
    /// benchmark of the formatter
    Formatter,
}

impl FromStr for FeatureToBenchmark {
    type Err = BenchmarkError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "parser" => Ok(Self::Parser),
            "formatter" => Ok(Self::Formatter),
            _ => Err(BenchmarkError::UnknownFeature),
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
#[derive(Serialize)]
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

pub struct Code {
    pub id: String,
    pub url: String,
    pub source: String,
}

pub struct Benchmarker {
    pub criterion: bool,
    pub baseline: Option<String>,
    pub feature: FeatureToBenchmark,
    pub code: Vec<Code>,
}

impl Benchmarker {
    pub fn benchmark(&self) -> Vec<BenchmarkSummary> {
        let mut summaries = vec![];

        for code in &self.code {
            let source_type: SourceType = Path::new(&code.id).try_into().unwrap();

            // Do all steps with criterion now
            if self.criterion {
                let mut criterion = criterion::Criterion::default()
                    .without_plots()
                    .measurement_time(Duration::new(10, 0));
                if let Some(ref baseline) = self.baseline {
                    criterion = criterion.save_baseline(baseline.to_string());
                }
                let mut group = criterion.benchmark_group(self.feature.to_string());
                group.throughput(criterion::Throughput::Bytes(code.source.len() as u64));

                group.bench_function(&code.id, |b| match self.feature {
                    FeatureToBenchmark::Parser => b.iter(|| {
                        criterion::black_box(run_parse(&code.source, source_type.clone()));
                    }),
                    FeatureToBenchmark::Formatter => {
                        let root = parse(&code.source, 0, source_type.clone()).syntax();
                        b.iter(|| {
                            criterion::black_box(run_format(&root));
                        })
                    }
                });
                group.finish();
            } else {
                //warmup
                match self.feature {
                    FeatureToBenchmark::Parser => {
                        run_parse(&code.source, source_type.clone());
                    }
                    FeatureToBenchmark::Formatter => {
                        let root = parse(&code.source, 0, source_type.clone()).syntax();
                        run_format(&root);
                    }
                }
            }

            let result = match self.feature {
                FeatureToBenchmark::Parser => {
                    benchmark_parse_lib(&code.id, &code.source, source_type)
                }
                FeatureToBenchmark::Formatter => {
                    let root = parse(&code.source, 0, source_type).syntax();
                    benchmark_format_lib(&code.id, &root)
                }
            };

            println!("Benchmark: {}", code.url);
            println!("{}", result);

            summaries.push(result);
        }

        summaries
    }
}
