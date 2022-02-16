use super::*;
use crate::reporters::TestReporter;
use rslint_parser::Syntax;
use std::fmt::Debug;
use std::panic::RefUnwindSafe;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;
use yastl::Pool;

#[derive(Debug)]
pub(crate) struct TestRunResult {
    pub(crate) outcome: TestRunOutcome,
    pub(crate) code: String,
    pub(crate) path: PathBuf,
}

#[derive(Debug)]
pub(crate) enum TestRunOutcome {
    Passed(Syntax),
    IncorrectlyPassed(Syntax),
    IncorrectlyErrored {
        syntax: Syntax,
        errors: Vec<ParserError>,
    },
    Panicked(Box<dyn Any + Send + 'static>),
}

impl TestRunOutcome {
    pub fn syntax(&self) -> Option<&Syntax> {
        match self {
            TestRunOutcome::IncorrectlyErrored { syntax, .. }
            | TestRunOutcome::IncorrectlyPassed(syntax)
            | TestRunOutcome::Passed(syntax) => Some(syntax),
            _ => None,
        }
    }

    pub fn is_failed(&self) -> bool {
        !matches!(self, TestRunOutcome::Passed(_))
    }

    pub fn panic_message(&self) -> Option<&str> {
        match self {
            TestRunOutcome::Panicked(panic) => panic
                .downcast_ref::<String>()
                .map(|s| s.as_str())
                .or_else(|| panic.downcast_ref::<&'static str>().copied()),

            _ => None,
        }
    }
}

impl From<TestRunOutcome> for Outcome {
    fn from(run_outcome: TestRunOutcome) -> Self {
        match run_outcome {
            TestRunOutcome::Passed(_) => Outcome::Passed,
            TestRunOutcome::IncorrectlyPassed(_) | TestRunOutcome::IncorrectlyErrored { .. } => {
                Outcome::Failed
            }
            TestRunOutcome::Panicked(_) => Outcome::Panicked,
        }
    }
}

pub(crate) trait TestCase: RefUnwindSafe + Send + Sync {
    fn path(&self) -> &Path;
    fn code(&self) -> &str;
    fn run(&self) -> TestRunOutcome;
}

pub(crate) trait TestSuite: Send + Sync {
    fn name(&self) -> &str;
    fn base_path(&self) -> &str;
    fn is_test(&self, path: &Path) -> bool;
    fn load_test(&self, entry: PathBuf) -> Option<Box<dyn TestCase>>;
}

pub(crate) struct TestSuiteInstance {
    name: String,
    tests: Vec<Box<dyn TestCase>>,
}

impl TestSuiteInstance {
    pub fn new(suite: &dyn TestSuite, tests: Vec<Box<dyn TestCase>>) -> Self {
        Self {
            tests,
            name: suite.name().to_string(),
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn len(&self) -> usize {
        self.tests.len()
    }

    pub fn iter(&self) -> impl Iterator<Item = &Box<dyn TestCase>> {
        self.tests.iter()
    }
}

pub(crate) struct TestRunContext<'a> {
    pub(crate) filter: Option<String>,
    pub(crate) reporter: &'a mut dyn TestReporter,
    pub(crate) pool: &'a Pool,
}

pub(crate) fn run_test_suite(
    test_suite: &dyn TestSuite,
    context: &mut TestRunContext,
) -> TestResults {
    context.reporter.test_suite_started(test_suite);
    let instance = load_tests(test_suite, context);
    context.reporter.test_suite_run_started(&instance);

    std::panic::set_hook(Box::new(|_| {}));

    let mut test_results = TestResults::new();
    let (tx, rx) = std::sync::mpsc::channel();

    context.pool.scoped(|scope| {
        scope.execute(|| {
            let mut results: Vec<TestResult> = Vec::with_capacity(instance.len());
            for result in rx {
                context.reporter.test_completed(&result);
                results.push(TestResult {
                    path: result.path,
                    outcome: result.outcome.into(),
                });
            }
            test_results.store_results(results);
        });

        for test in instance.iter() {
            let tx = tx.clone();

            scope.execute(move || {
                let test_ref = test.as_ref();
                let run_result = std::panic::catch_unwind(|| test_ref.run());

                let outcome = run_result.unwrap_or_else(|panic| TestRunOutcome::Panicked(panic));

                tx.send(TestRunResult {
                    code: test.code().to_string(),
                    path: test.path().to_path_buf(),
                    outcome,
                })
                .unwrap();
            });
        }

        drop(tx);
    });

    context
        .reporter
        .test_suite_completed(&instance, &test_results);

    let _ = std::panic::take_hook();

    test_results
}

fn load_tests(suite: &dyn TestSuite, context: &mut TestRunContext) -> TestSuiteInstance {
    let paths = WalkDir::new(suite.base_path())
        .into_iter()
        .filter_map(Result::ok)
        .filter(|file| {
            let path = file.path();
            if !path.is_file() {
                return false;
            }

            if !suite.is_test(path) {
                return false;
            }

            if let Some(filter) = &context.filter {
                let normalized_path = path.to_string_lossy().replace("\\", "/");
                let normalized_query = filter.replace("\\", "/");
                normalized_path.contains(&normalized_query)
            } else {
                true
            }
        })
        .map(|file| file.path().to_path_buf())
        .collect::<Vec<_>>();

    context.reporter.tests_discovered(suite, paths.len());

    let (tx, rx) = std::sync::mpsc::channel();
    let mut tests: Vec<Box<dyn TestCase>> = Vec::with_capacity(paths.len());

    context.pool.scoped(|scope| {
        scope.execute(|| {
            for test in rx {
                context.reporter.test_loaded();
                if let Some(test) = test {
                    tests.push(test);
                }
            }
        });

        for path in paths {
            let tx = tx.clone();

            scope.execute(move || {
                tx.send(suite.load_test(path)).unwrap();
            });
        }

        drop(tx);
    });

    TestSuiteInstance::new(suite, tests)
}
