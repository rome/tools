use rslint_parser::SourceType;

use crate::{
    check_file_encoding,
    runner::{TestCase, TestCaseFiles, TestRunOutcome, TestSuite},
};
use std::path::PathBuf;

const CASES_PATH: &str = "xtask/coverage/babel/packages/babel-parser/test/fixtures/typescript";

struct BabelTypescriptTestCase {
    path: PathBuf,
    code: String,
}

impl BabelTypescriptTestCase {
    fn new(path: PathBuf, code: String) -> Self {
        Self { path, code }
    }
}

impl TestCase for BabelTypescriptTestCase {
    fn name(&self) -> &str {
        self.path
            .components()
            .rev()
            .nth(1)
            .and_then(|x| x.as_os_str().to_str())
            .unwrap_or("")
    }

    fn run(&self) -> TestRunOutcome {
        let ts = SourceType::ts();
        let r = rslint_parser::parse(&self.code, 0, ts.clone());

        let file = TestCaseFiles::single(self.name().to_string(), self.code.clone(), ts);
        if r.errors().len() == 0 {
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
pub(crate) struct BabelTypescriptTestSuite;

impl TestSuite for BabelTypescriptTestSuite {
    fn name(&self) -> &str {
        "ts/babel"
    }

    fn base_path(&self) -> &str {
        CASES_PATH
    }

    fn is_test(&self, path: &std::path::Path) -> bool {
        path.extension().map(|x| x == "ts").unwrap_or(false)
    }

    fn load_test(&self, path: &std::path::Path) -> Option<Box<dyn crate::runner::TestCase>> {
        let code = check_file_encoding(path)?;
        Some(Box::new(BabelTypescriptTestCase::new(
            path.to_path_buf(),
            code,
        )))
    }
}
