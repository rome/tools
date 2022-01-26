pub mod compare;
pub mod files;
pub mod results;
pub mod test262;
pub mod typescript;

use ascii_table::{AsciiTable, Column};
use colored::Colorize;
use rslint_parser::ParserError;
use serde::{Deserialize, Serialize};
use std::any::Any;
use std::path::PathBuf;

enum ExecRes {
    Errors(Vec<ParserError>),
    ParseCorrectly,
    ParserPanic(Box<dyn Any + Send + 'static>),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TestResult {
    #[serde(skip)]
    pub fail: Option<FailReason>,
    #[serde(rename = "o")]
    pub outcome: Outcome,
    #[serde(rename = "h")]
    pub path: PathBuf,
    #[serde(skip)]
    pub code: String,
}

#[derive(Debug)]
pub enum FailReason {
    IncorrectlyPassed,
    IncorrectlyErrored(Vec<ParserError>),
    ParserPanic(Box<dyn Any + Send + 'static>),
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
pub enum Outcome {
    Passed,
    Failed,
    Panicked,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TestResults {
    #[serde(rename = "s")]
    pub summary: Summary,
    #[serde(rename = "p")]
    pub details: Vec<TestResult>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Summary {
    #[serde(rename = "a")]
    pub tests_ran: u32,
    #[serde(rename = "pa")]
    pub passed: u32,
    #[serde(rename = "f")]
    pub failed: u32,
    #[serde(rename = "pc")]
    pub panics: u32,
    #[serde(rename = "c")]
    pub coverage: f64,
}

impl Default for TestResults {
    fn default() -> Self {
        Self {
            summary: Summary {
                tests_ran: 0,
                passed: 0,
                failed: 0,
                panics: 0,
                coverage: 0.0,
            },
            details: vec![],
        }
    }
}

impl TestResults {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn store_results(&mut self, results: Vec<TestResult>) {
        self.details = results;
        let passed = self.passed_tests() as u32;
        let tests_ran = self.details.len();
        let coverage = (passed as f64 / tests_ran as f64) * 100.0;
        self.summary = Summary {
            tests_ran: self.details.len() as u32,
            passed,
            failed: self.errored_tests() as u32,
            panics: self.panicked_tests() as u32,
            coverage,
        };
    }

    pub fn panicked_tests(&self) -> usize {
        self.details
            .iter()
            .filter(|res| matches!(res.fail, Some(FailReason::ParserPanic(_))))
            .count()
    }

    pub fn errored_tests(&self) -> usize {
        self.details
            .iter()
            .filter(|res| {
                matches!(
                    res.fail,
                    Some(FailReason::IncorrectlyErrored(_)) | Some(FailReason::IncorrectlyPassed)
                )
            })
            .count()
    }

    pub fn passed_tests(&self) -> usize {
        self.details.iter().filter(|res| res.fail.is_none()).count()
    }

    /// Prints results of the coverage to STDOUT in JSON format
    pub fn dump_to_json(&self) {
        let json = serde_json::to_string(&self).unwrap();
        println!("{}", json);
    }
}

fn default_bar_style() -> indicatif::ProgressStyle {
    indicatif::ProgressStyle::default_bar()
        .template("{msg} [{bar:40}]")
        .progress_chars("=> ")
}

fn draw_table(test_results: &TestResults) {
    let panicked = test_results.summary.panics;
    let errored = test_results.summary.failed;
    let passed = test_results.summary.passed;
    let coverage = format!("{:.2}", test_results.summary.coverage);

    let total = panicked + errored + passed;

    let mut table = AsciiTable::default();

    let mut counter = 0usize;
    let mut create_column = |name: colored::ColoredString| {
        let column = Column {
            header: name.to_string(),
            align: ascii_table::Align::Center,
            ..Column::default()
        };
        table.columns.insert(counter, column);
        counter += 1;
    };
    create_column("Tests ran".into());
    create_column("Passed".green());
    create_column("Failed".red());
    create_column("Panics".red());
    create_column("Coverage".cyan());
    let numbers: Vec<&dyn std::fmt::Display> =
        vec![&total, &passed, &errored, &panicked, &coverage];

    table.print(vec![numbers]);
}

pub fn run(
    language: &str,
    query: Option<&str>,
    json: bool,
    show_rast: bool,
    show_diagnostics: bool,
) {
    let pool = yastl::ThreadConfig::new().stack_size(8 << 30);

    let language = language.to_lowercase();
    match language.as_str() {
        "javascript" | "js" => {
            test262::run_js(
                query,
                yastl::Pool::with_config(num_cpus::get(), pool),
                json,
                show_rast,
                show_diagnostics,
            );
        }
        "typescript" | "ts" => {
            typescript::run_ts(
                query,
                yastl::Pool::with_config(num_cpus::get(), pool),
                json,
                show_rast,
                show_diagnostics,
            );
        }
        other => panic!("Unkown language: {}", other),
    }
}
