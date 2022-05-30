use super::*;
use crate::reporters::TestReporter;
use rome_diagnostics::file::{FileId, SimpleFiles};
use rome_diagnostics::termcolor::Buffer;
use rome_diagnostics::{Diagnostic, Emitter, Severity};
use rome_js_parser::{parse, Parse};
use rome_js_syntax::{JsAnyRoot, JsSyntaxNode, SourceType};
use rome_rowan::SyntaxKind;
use std::fmt::Debug;
use std::panic::RefUnwindSafe;
use std::path::Path;
use walkdir::WalkDir;
use yastl::Pool;

pub(crate) struct TestRunResult {
    pub(crate) outcome: TestRunOutcome,
    pub(crate) test_case: String,
}

pub(crate) enum TestRunOutcome {
    Passed(TestCaseFiles),
    IncorrectlyPassed(TestCaseFiles),
    IncorrectlyErrored {
        files: TestCaseFiles,
        errors: Vec<ParseDiagnostic>,
    },
    Panicked(Box<dyn Any + Send + 'static>),
}

impl TestRunOutcome {
    pub fn is_failed(&self) -> bool {
        !matches!(self, TestRunOutcome::Passed(_))
    }

    pub fn files(&self) -> Option<&TestCaseFiles> {
        match self {
            TestRunOutcome::Passed(files)
            | TestRunOutcome::IncorrectlyPassed(files)
            | TestRunOutcome::IncorrectlyErrored { files, .. } => Some(files),
            _ => None,
        }
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

/// A test case may parse multiple files. Represents a single file of a test case
#[derive(Debug, Clone)]
pub(crate) struct TestCaseFile {
    /// The (test case relative) name of the file
    name: String,

    /// The code of the file
    code: String,

    /// The source type used to parse the file
    source_type: SourceType,

    id: FileId,
}

impl TestCaseFile {
    pub(crate) fn parse(&self) -> Parse<JsAnyRoot> {
        parse(&self.code, self.id, self.source_type.clone())
    }

    pub(crate) fn name(&self) -> &str {
        &self.name
    }

    pub(crate) fn id(&self) -> FileId {
        self.id
    }
}

pub(crate) fn create_unknown_node_in_tree_diagnostic(
    file_id: FileId,
    node: JsSyntaxNode,
) -> Diagnostic {
    assert!(node.kind().is_unknown());
    Diagnostic::new(
        file_id,
        Severity::Bug,
        "There are no parse errors but the parsed tree contains unknown nodes.",
    )
        .primary(node.text_trimmed_range(), "This unknown node is present in the parsed tree but the parser didn't emit a diagnostic for it. Change the parser to either emit a diagnostic, fix the grammar, or the parsing.")
}

#[derive(Clone, Debug)]
pub(crate) struct TestCaseFiles {
    files: Vec<TestCaseFile>,
}

impl TestCaseFiles {
    pub(crate) fn single(name: String, code: String, source_type: SourceType) -> Self {
        Self {
            files: vec![TestCaseFile {
                name,
                code,
                source_type,
                id: 0,
            }],
        }
    }

    pub(crate) fn new() -> Self {
        Self { files: vec![] }
    }

    pub(crate) fn add(&mut self, name: String, code: String, source_type: SourceType) {
        self.files.push(TestCaseFile {
            name,
            code,
            source_type,
            id: self.files.len(),
        })
    }

    pub(crate) fn is_empty(&self) -> bool {
        self.files.is_empty()
    }

    pub(crate) fn emit_errors(&self, errors: &[ParseDiagnostic], buffer: &mut Buffer) {
        let mut diag_files = SimpleFiles::new();

        for file in &self.files {
            diag_files.add(file.name.clone(), file.code.clone());
        }

        let mut emitter = Emitter::new(&diag_files);
        for error in errors {
            if let Err(err) = emitter.emit_with_writer(error, buffer) {
                eprintln!("Failed to print diagnostic: {}", err);
            }
        }
    }
}

impl<'a> IntoIterator for &'a TestCaseFiles {
    type Item = &'a TestCaseFile;
    type IntoIter = std::slice::Iter<'a, TestCaseFile>;

    fn into_iter(self) -> Self::IntoIter {
        self.files.iter()
    }
}

impl From<TestCaseFiles> for SimpleFiles {
    fn from(files: TestCaseFiles) -> Self {
        let mut result = SimpleFiles::new();

        for file in files.files {
            result.add(file.name, file.code);
        }

        result
    }
}

pub(crate) trait TestCase: RefUnwindSafe + Send + Sync {
    /// Returns the (display) name of the test case. Used to uniquely identify the test case
    fn name(&self) -> &str;

    /// Runs the test case
    fn run(&self) -> TestRunOutcome;
}

pub(crate) trait TestSuite: Send + Sync {
    fn name(&self) -> &str;
    fn base_path(&self) -> &str;
    fn is_test(&self, path: &Path) -> bool;
    fn load_test(&self, path: &Path) -> Option<Box<dyn TestCase>>;
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
                    test_case: result.test_case,
                    outcome: result.outcome.into(),
                });
            }
            test_results.store_results(results);
        });

        for test in instance.iter() {
            let tx = tx.clone();

            scope.execute(move || {
                let test_ref = test.as_ref();

                let outcome = match std::panic::catch_unwind(|| test_ref.run()) {
                    Ok(result) => result,
                    Err(panic) => {
                        let err = panic
                            .downcast_ref::<String>()
                            .map(|x| x.to_string())
                            .or_else(|| panic.downcast_ref::<&str>().map(|x| x.to_string()));
                        tracing::warn!("Test [{}] panicked: {err:?}", test.name());
                        TestRunOutcome::Panicked(panic)
                    }
                };

                tx.send(TestRunResult {
                    test_case: test.name().to_owned(),
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
                let normalized_path = path.to_string_lossy().replace('\\', "/");
                let normalized_query = filter.replace('\\', "/");
                normalized_path.contains(&normalized_query)
            } else {
                true
            }
        })
        .map(|file| file.path().to_owned())
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
                tx.send(suite.load_test(&path)).unwrap();
            });
        }

        drop(tx);
    });

    TestSuiteInstance::new(suite, tests)
}
