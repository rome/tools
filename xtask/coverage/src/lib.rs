pub mod compare;
mod reporters;
pub mod results;
mod runner;
pub mod test262;
pub mod typescript;

use crate::reporters::{
    CliProgressReporter, DiagnosticsReporter, JsonReporter, MulticastTestReporter, RastReporter,
    SummaryReporter,
};
use crate::runner::{run_test_suite, TestRunContext, TestSuite};
use crate::test262::Test262TestSuite;
use crate::typescript::TypeScriptTestSuite;
use rslint_parser::ParserError;
use serde::{Deserialize, Serialize};
use std::any::Any;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize)]
pub struct TestResult {
    #[serde(rename = "o")]
    pub outcome: Outcome,
    #[serde(rename = "h")]
    pub path: PathBuf,
}

#[derive(Debug, PartialEq, Serialize, Deserialize, Copy, Clone)]
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
            .filter(|res| matches!(res.outcome, Outcome::Panicked))
            .count()
    }

    pub fn errored_tests(&self) -> usize {
        self.details
            .iter()
            .filter(|res| matches!(res.outcome, Outcome::Failed))
            .count()
    }

    pub fn passed_tests(&self) -> usize {
        self.details
            .iter()
            .filter(|res| res.outcome == Outcome::Passed)
            .count()
    }
}

pub fn run(
    language: &str,
    query: Option<&str>,
    json: bool,
    show_rast: bool,
    show_diagnostics: bool,
) {
    let language = language.to_lowercase();
    let loader: Box<dyn TestSuite> = match language.as_str() {
        "javascript" | "js" => Box::new(Test262TestSuite),
        "typescript" | "ts" => Box::new(TypeScriptTestSuite),
        other => panic!("Unknown language: {}", other),
    };

    let mut reporters = MulticastTestReporter::new(Box::new(CliProgressReporter::default()));

    if json {
        reporters.add(Box::new(JsonReporter));
    } else {
        reporters.add(Box::new(SummaryReporter::default()));
    }

    if show_rast {
        reporters.add(Box::new(RastReporter));
    }

    if show_diagnostics {
        reporters.add(Box::new(DiagnosticsReporter));
    }

    let context = TestRunContext {
        query: query.map(|s| s.to_string()),
        reporter: &mut reporters,
        pool: &yastl::Pool::with_config(
            num_cpus::get(),
            yastl::ThreadConfig::new().stack_size(8 << 30),
        ),
    };

    let results = run_test_suite(loader.as_ref(), context);

    if results.passed_tests() == 0 {
        std::process::exit(1);
    }
}
