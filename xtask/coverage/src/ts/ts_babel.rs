use rome_js_parser::SourceType;
use rome_rowan::SyntaxKind;

use crate::runner::create_unknown_node_in_tree_diagnostic;
use crate::{
    check_file_encoding,
    runner::{TestCase, TestCaseFiles, TestRunOutcome, TestSuite},
};
use std::path::Path;

const CASES_PATH: &str = "xtask/coverage/babel/packages/babel-parser/test/fixtures/typescript";

struct BabelTypescriptTestCase {
    name: String,
    expected_to_fail: bool,
    code: String,
}

impl BabelTypescriptTestCase {
    fn new(path: &Path, code: String, expected_to_fail: bool) -> Self {
        let name = path
            .parent()
            .unwrap()
            .strip_prefix(CASES_PATH)
            .unwrap()
            .display()
            .to_string();

        Self {
            name,
            code,
            expected_to_fail,
        }
    }
}

impl TestCase for BabelTypescriptTestCase {
    fn name(&self) -> &str {
        &self.name
    }

    fn run(&self) -> TestRunOutcome {
        let source_type = SourceType::ts();
        let files = TestCaseFiles::single(
            self.name().to_string(),
            self.code.clone(),
            source_type.clone(),
        );

        let result = rome_js_parser::parse(&self.code, 0, source_type);

        if self.expected_to_fail && result.diagnostics().is_empty() {
            TestRunOutcome::IncorrectlyPassed(files)
        } else if self.expected_to_fail {
            TestRunOutcome::Passed(files)
        } else if !result.diagnostics().is_empty() {
            TestRunOutcome::IncorrectlyErrored {
                files,
                errors: result.diagnostics().to_vec(),
            }
        } else if let Some(unknown) = result
            .syntax()
            .descendants()
            .find(|descendant| descendant.kind().is_unknown())
        {
            TestRunOutcome::IncorrectlyErrored {
                files,
                errors: vec![create_unknown_node_in_tree_diagnostic(0, unknown)],
            }
        } else {
            TestRunOutcome::Passed(files)
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

        let output_json_path = path.with_file_name("output.json");
        let options_path = path.with_file_name("options.json");

        let should_fail = if output_json_path.exists() {
            check_file_encoding(&output_json_path)
                .map(|content| content.contains("\"errors\":"))
                .unwrap_or(false)
        } else if options_path.exists() {
            check_file_encoding(&options_path)
                .map(|content| content.contains("\"throws\":"))
                .unwrap_or(false)
        } else {
            false
        };

        Some(Box::new(BabelTypescriptTestCase::new(
            path,
            code,
            should_fail,
        )))
    }
}
