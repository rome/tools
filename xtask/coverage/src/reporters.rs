use crate::runner::{TestRunOutcome, TestRunResult, TestSuite, TestSuiteInstance};
use crate::{Summary, TestResults};
use ascii_table::{AsciiTable, Column};
use colored::Colorize;
use indicatif::{ProgressBar, ProgressDrawTarget};
use rslint_errors::file::SimpleFile;
use rslint_errors::Emitter;
use rslint_parser::parse;
use serde_json::Value;
use std::collections::HashMap;
use std::time::Instant;

pub(crate) trait TestReporter: Send + Sync {
    /// Called at the beginning of processing a test suite
    fn test_suite_started(&mut self, _suite: &dyn TestSuite) {}
    /// Called after all potential tests have been discovered for a test suite
    fn tests_discovered(&mut self, _suite: &dyn TestSuite, _count: usize) {}
    /// Called after loading a single test of a test suite
    fn test_loaded(&mut self) {}
    /// Called after all tests for a test suite have been loaded, right before the runner executes the tests
    fn test_suite_run_started(&mut self, _suite: &TestSuiteInstance) {}
    /// A test run completed
    fn test_completed(&mut self, _result: &TestRunResult) {}
    /// A test suite completed
    fn test_suite_completed(&mut self, _suite: &TestSuiteInstance, _result: &TestResults) {}

    /// Called when all test suites have completed
    fn run_completed(&mut self) {}
}

pub(crate) struct CliProgressReporter {
    pb: ProgressBar,
    detailed: bool,
}

impl Default for CliProgressReporter {
    fn default() -> Self {
        Self {
            pb: ProgressBar::hidden(),
            detailed: false,
        }
    }
}

impl TestReporter for CliProgressReporter {
    fn tests_discovered(&mut self, _suite: &dyn TestSuite, count: usize) {
        self.pb.finish_and_clear();

        let pb = ProgressBar::new(count as u64);
        pb.set_message(format!("{} test files", "Loading".bold().cyan()));
        pb.set_style(
            indicatif::ProgressStyle::default_bar()
                .template("{msg} [{bar:40}]")
                .progress_chars("=> "),
        );
        self.pb = pb;
    }

    fn test_loaded(&mut self) {
        self.pb.inc(1);
    }

    fn test_suite_run_started(&mut self, suite: &TestSuiteInstance) {
        self.pb.finish_and_clear();

        self.detailed = suite.len() < 10;

        let pb = ProgressBar::hidden();
        pb.set_length(suite.len() as u64);
        pb.set_message(format!("{} tests", "Running".bold().cyan()));
        // Redrawing on each test adds a significant overhead, batch some redraws together
        pb.set_draw_delta(10);

        self.pb = pb;
    }

    fn test_completed(&mut self, result: &TestRunResult) {
        if self.pb.position() == 0 {
            self.pb.set_draw_target(ProgressDrawTarget::stderr());
        }

        self.pb.inc(1);

        if self.detailed {
            print_detailed_test_result(&self.pb, result);
        }

        let reason = match &result.outcome {
            TestRunOutcome::Passed(_) => return,
            TestRunOutcome::IncorrectlyPassed(_) => "incorrectly passed parsing",
            TestRunOutcome::IncorrectlyErrored { .. } => "incorrectly threw an error",
            TestRunOutcome::Panicked(_) => "panicked while parsing",
        };

        let msg = format!(
            "{} '{}' {}",
            "Test".bold().red(),
            result.path.display(),
            reason.bold()
        );
        self.pb.println(msg);
    }

    fn test_suite_completed(&mut self, _suite: &TestSuiteInstance, _results: &TestResults) {
        self.pb.finish_and_clear();
    }
}

fn print_detailed_test_result(pb: &ProgressBar, result: &TestRunResult) {
    let test_name = result.path.display().to_string();

    match &result.outcome {
        TestRunOutcome::Panicked(panic) => {
            let msg = panic.downcast_ref::<String>();

            let header = format!(
                "    This test caused a{} panic inside the parser{}",
                if msg.is_none() { "n unknown" } else { "" },
                if msg.is_none() { "" } else { ":\n" }
            )
            .bold();

            let msg = if let Some(msg) = msg {
                format!(
                    "{}    {}\n\n    For more information about the panic run the file manually",
                    header, msg
                )
            } else {
                format!("\n{} '{}' {}\n", "Test".bold(), test_name, "failed".bold())
                    .red()
                    .underline()
                    .to_string()
            };

            pb.println(format!("{}{}", header, msg))
        }
        TestRunOutcome::IncorrectlyErrored { errors, .. } => {
            let header = format!("\n{} '{}' {}\n", "Test".bold(), test_name, "failed".bold())
                .red()
                .underline()
                .to_string();

            let sub_header =
                "    This test threw errors but expected to pass parsing without errors:\n"
                    .to_string();
            let file = SimpleFile::new(test_name, result.code.clone());
            let mut emitter = Emitter::new(&file);
            let mut buf = rslint_errors::termcolor::Buffer::ansi();
            for error in errors.iter() {
                emitter
                    .emit_with_writer(error, &mut buf)
                    .expect("failed to emit error");
            }
            let errors = String::from_utf8(buf.into_inner()).expect("errors are not utf-8");
            pb.println(format!("{}{}\n{}", header, sub_header, errors))
        }
        TestRunOutcome::IncorrectlyPassed(_) => {
            let header = format!("\n{} '{}' {}\n", "Test".bold(), test_name, "failed".bold())
                .red()
                .underline()
                .to_string();
            pb.println(format!(
                "{}    Expected this test to fail, but instead it passed without errors.",
                header
            ))
        }
        TestRunOutcome::Passed(_) => {}
    }
}

/// Reporter that prints a summary for each phase (tests loaded, test suite completed) to the console output
pub(crate) struct SummaryReporter {
    start: Instant,
    results: HashMap<String, Summary>,
}

impl Default for SummaryReporter {
    fn default() -> Self {
        Self {
            start: Instant::now(),
            results: HashMap::default(),
        }
    }
}

impl TestReporter for SummaryReporter {
    fn test_suite_started(&mut self, _suite: &dyn TestSuite) {
        self.start = Instant::now()
    }

    fn test_suite_run_started(&mut self, suite: &TestSuiteInstance) {
        println!(
            "{} {} test files in {:.2}s",
            "Loaded".bold().bright_green(),
            suite.len(),
            self.start.elapsed().as_secs_f32()
        );

        self.start = Instant::now();
    }

    fn test_suite_completed(&mut self, suite: &TestSuiteInstance, results: &TestResults) {
        println!(
            "\n{}: {} {} tests in {:.2}s\n",
            suite.name(),
            "Ran".bold().bright_green(),
            suite.len(),
            self.start.elapsed().as_secs_f32()
        );

        self.results
            .insert(suite.name().to_string(), results.summary.clone());
    }

    fn run_completed(&mut self) {
        let mut table = AsciiTable::default();
        let results = std::mem::take(&mut self.results);
        let has_multiple_test_suites = results.len() > 1;

        if has_multiple_test_suites {
            table.columns.insert(
                0,
                Column {
                    header: "Test suite".into(),
                    align: ascii_table::Align::Left,
                    ..Column::default()
                },
            );
        }

        let mut create_number_column = |name: colored::ColoredString| {
            let column = Column {
                header: name.to_string(),
                align: ascii_table::Align::Right,
                ..Column::default()
            };
            table.columns.insert(table.columns.len(), column);
        };

        create_number_column("Tests ran".into());
        create_number_column("Passed".green());
        create_number_column("Failed".red());
        create_number_column("Panics".red());
        create_number_column("Coverage".cyan());

        let rows = results.into_iter().map(|(suite, summary)| {
            let panicked = summary.panics;
            let errored = summary.failed;
            let passed = summary.passed;
            let coverage = format!("{:.2}", summary.coverage);

            let total = panicked + errored + passed;

            let mut values = if has_multiple_test_suites {
                vec![suite]
            } else {
                Vec::default()
            };

            values.extend([
                total.to_string(),
                passed.to_string(),
                errored.to_string(),
                panicked.to_string(),
                coverage,
            ]);

            values
        });

        table.print(rows);
    }
}

#[derive(Default)]
pub(crate) struct JsonReporter {
    results: HashMap<String, Value>,
}

impl TestReporter for JsonReporter {
    fn test_suite_completed(&mut self, suite: &TestSuiteInstance, result: &TestResults) {
        self.results.insert(
            suite.name().to_string(),
            serde_json::to_value(result).unwrap(),
        );
    }

    fn run_completed(&mut self) {
        let results = std::mem::take(&mut self.results);
        println!("{}", serde_json::to_string(&results).unwrap());
    }
}

/// Test reporter that forwards the event to multiple reporters.
/// Allows composing different reporters for a single test run
pub(crate) struct MulticastTestReporter(Vec<Box<dyn TestReporter>>);

impl MulticastTestReporter {
    pub fn new(reporter: Box<dyn TestReporter>) -> Self {
        Self(vec![reporter])
    }

    pub fn add(&mut self, reporter: Box<dyn TestReporter>) {
        self.0.push(reporter);
    }
}

impl TestReporter for MulticastTestReporter {
    fn test_suite_started(&mut self, test_suite: &dyn TestSuite) {
        for reporter in &mut self.0 {
            reporter.test_suite_started(test_suite);
        }
    }

    fn tests_discovered(&mut self, test_suite: &dyn TestSuite, count: usize) {
        for reporter in &mut self.0 {
            reporter.tests_discovered(test_suite, count);
        }
    }

    fn test_loaded(&mut self) {
        for reporter in &mut self.0 {
            reporter.test_loaded();
        }
    }

    fn test_suite_run_started(&mut self, suite: &TestSuiteInstance) {
        for reporter in &mut self.0 {
            reporter.test_suite_run_started(suite);
        }
    }

    fn test_completed(&mut self, result: &TestRunResult) {
        for reporter in &mut self.0 {
            reporter.test_completed(result);
        }
    }

    fn test_suite_completed(&mut self, suite: &TestSuiteInstance, result: &TestResults) {
        for reporter in &mut self.0 {
            reporter.test_suite_completed(suite, result);
        }
    }

    fn run_completed(&mut self) {
        for reporter in &mut self.0 {
            reporter.run_completed();
        }
    }
}

/// Prints the rast output for all (not panicking) tests to the console
pub(crate) struct RastReporter;

impl TestReporter for RastReporter {
    fn test_completed(&mut self, result: &TestRunResult) {
        if let Some(syntax) = result.outcome.syntax() {
            let program = parse(&result.code, 0, *syntax);
            println!("{:#?}", program.syntax());
        }
    }
}

/// Prints the diagnostics of all not panicking test to the console
pub(crate) struct DiagnosticsReporter;

impl TestReporter for DiagnosticsReporter {
    fn test_completed(&mut self, result: &TestRunResult) {
        if let Some(syntax) = result.outcome.syntax() {
            let program = parse(&result.code, 0, *syntax);

            let file = rslint_errors::file::SimpleFile::new(
                result.path.display().to_string(),
                result.code.to_string(),
            );

            let mut emitter = rslint_errors::Emitter::new(&file);

            for diagnostic in program.errors() {
                emitter.emit_stdout(diagnostic, true).unwrap();
            }
        }
    }
}
