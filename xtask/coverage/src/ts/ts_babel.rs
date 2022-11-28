use crate::runner::create_bogus_node_in_tree_diagnostic;
use crate::{
    check_file_encoding,
    runner::{TestCase, TestCaseFiles, TestRunOutcome, TestSuite},
};
use rome_diagnostics::location::FileId;
use rome_js_syntax::{LanguageVariant, SourceType};
use rome_rowan::SyntaxKind;
use std::path::Path;

const CASES_PATH: &str = "xtask/coverage/babel/packages/babel-parser/test/fixtures/typescript";

struct BabelTypescriptTestCase {
    name: String,
    expected_to_fail: bool,
    code: String,
    variant: LanguageVariant,
}

impl BabelTypescriptTestCase {
    fn new(path: &Path, code: String, expected_to_fail: bool, variant: LanguageVariant) -> Self {
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
            variant,
        }
    }
}

impl TestCase for BabelTypescriptTestCase {
    fn name(&self) -> &str {
        &self.name
    }

    fn run(&self) -> TestRunOutcome {
        let source_type = SourceType::ts().with_variant(self.variant);
        let files = TestCaseFiles::single(self.name().to_string(), self.code.clone(), source_type);

        let result = rome_js_parser::parse(&self.code, FileId::zero(), source_type);

        if self.expected_to_fail && result.diagnostics().is_empty() {
            TestRunOutcome::IncorrectlyPassed(files)
        } else if self.expected_to_fail {
            TestRunOutcome::Passed(files)
        } else if !result.diagnostics().is_empty() {
            TestRunOutcome::IncorrectlyErrored {
                files,
                errors: result.diagnostics().to_vec(),
            }
        } else if let Some(bogus) = result
            .syntax()
            .descendants()
            .find(|descendant| descendant.kind().is_bogus())
        {
            TestRunOutcome::IncorrectlyErrored {
                files,
                errors: vec![create_bogus_node_in_tree_diagnostic(FileId::zero(), bogus)],
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

        let mut should_fail = false;
        let mut variant = LanguageVariant::Standard;

        if output_json_path.exists() {
            if let Some(content) = check_file_encoding(&output_json_path) {
                should_fail = content.contains("\"errors\":");
            }
        }

        if options_path.exists() {
            if let Some(content) = check_file_encoding(&options_path) {
                should_fail = should_fail || content.contains("\"throws\":");

                if content.contains("jsx") {
                    variant = LanguageVariant::Jsx;
                }
            }
        };

        Some(Box::new(BabelTypescriptTestCase::new(
            path,
            code,
            should_fail,
            variant,
        )))
    }
}
