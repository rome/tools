use crate::{
    check_file_encoding,
    runner::{TestCase, TestCaseFiles, TestRunOutcome, TestSuite},
};
use std::path::PathBuf;

const OK_PATH: &str = "xtask/coverage/babel/packages/babel-parser/test/fixtures/jsx/basic";

struct BabelJsxTestCase {
    path: PathBuf,
    code: String,
}

impl BabelJsxTestCase {
    fn new(path: PathBuf, code: String) -> Self {
        Self { path, code }
    }
}

impl TestCase for BabelJsxTestCase {
    fn name(&self) -> &str {
        self.path
            .components()
            .rev()
            .nth(1)
            .and_then(|x| x.as_os_str().to_str())
            .unwrap_or("")
    }

    fn run(&self) -> TestRunOutcome {
        let jsx = rslint_parser::SourceType::jsx();
        let r = rslint_parser::parse(&self.code, 0, jsx.clone());

        let file = TestCaseFiles::single(self.name().to_string(), self.code.clone(), jsx);
        if r.errors().is_empty() {
            TestRunOutcome::Passed(file)
        } else {
            TestRunOutcome::IncorrectlyErrored {
                files: file,
                errors: r.errors().to_vec(),
            }
        }
    }
}

#[derive(Default)]
pub(crate) struct BabelJsxTestSuite;

impl TestSuite for BabelJsxTestSuite {
    fn name(&self) -> &str {
        "jsx/babel"
    }

    fn base_path(&self) -> &str {
        OK_PATH
    }

    fn is_test(&self, path: &std::path::Path) -> bool {
        path.extension().map(|x| x == "js").unwrap_or(false)
    }

    fn load_test(&self, path: &std::path::Path) -> Option<Box<dyn crate::runner::TestCase>> {
        let code = check_file_encoding(path)?;
        Some(Box::new(BabelJsxTestCase::new(path.to_path_buf(), code)))
    }
}
