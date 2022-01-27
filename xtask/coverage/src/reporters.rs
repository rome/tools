use crate::runner::{TestRunOutcome, TestRunResult, TestSuite, TestSuiteInstance};
use crate::TestResults;
use ascii_table::{AsciiTable, Column};
use colored::Colorize;
use indicatif::{ProgressBar, ProgressDrawTarget};
use rslint_errors::file::SimpleFile;
use rslint_errors::Emitter;
use rslint_parser::parse;
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
}

impl Default for SummaryReporter {
    fn default() -> Self {
        Self {
            start: Instant::now(),
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

        let panicked = results.summary.panics;
        let errored = results.summary.failed;
        let passed = results.summary.passed;
        let coverage = format!("{:.2}", results.summary.coverage);

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
}

pub(crate) struct JsonReporter;

impl TestReporter for JsonReporter {
    fn test_suite_completed(&mut self, _suite: &TestSuiteInstance, result: &TestResults) {
        let json = serde_json::to_string(result).unwrap();
        println!("{}", json);
    }
}

pub(crate) struct CompositeTestReporter(Vec<Box<dyn TestReporter>>);

impl CompositeTestReporter {
    pub fn new(reporter: Box<dyn TestReporter>) -> Self {
        Self(vec![reporter])
    }

    pub fn add(&mut self, reporter: Box<dyn TestReporter>) {
        self.0.push(reporter);
    }
}

impl TestReporter for CompositeTestReporter {
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
}

/// Prints the rast output for all (not panicking) tests to the console
pub(crate) struct RastReporter;

impl TestReporter for RastReporter {
    fn test_completed(&mut self, result: &TestRunResult) {
        if let Some(syntax) = result.outcome.syntax() {
            let program = parse(&result.code, 0, syntax.clone());
            println!("{:#?}", program.syntax());
        }
    }
}

/// Prints the diagnostics of all not panicking test to the console
pub(crate) struct DiagnosticsReporter;

impl TestReporter for DiagnosticsReporter {
    fn test_completed(&mut self, result: &TestRunResult) {
        if let Some(syntax) = result.outcome.syntax() {
            let program = parse(&result.code, 0, syntax.clone());

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
