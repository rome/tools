use crate::runner::create_unknown_node_in_tree_diagnostic;
use crate::{
    check_file_encoding,
    runner::{TestCase, TestCaseFiles, TestRunOutcome, TestSuite},
};
use rome_js_parser::parse;
use rome_js_syntax::{ModuleKind, SourceType};
use rome_rowan::SyntaxKind;
use std::path::Path;

const OK_PATH: &str = "xtask/coverage/babel/packages/babel-parser/test/fixtures/jsx/basic";

struct BabelJsxTestCase {
    name: String,
    code: String,
}

impl BabelJsxTestCase {
    fn new(path: &Path, code: String) -> Self {
        let name = path
            .components()
            .rev()
            .nth(1)
            .and_then(|x| x.as_os_str().to_str())
            .unwrap_or("")
            .to_string();

        Self { name, code }
    }
}

impl TestCase for BabelJsxTestCase {
    fn name(&self) -> &str {
        &self.name
    }

    fn run(&self) -> TestRunOutcome {
        let source_type = SourceType::jsx().with_module_kind(ModuleKind::Script);
        let files = TestCaseFiles::single(self.name().to_string(), self.code.clone(), source_type);
        let result = parse(&self.code, 0, source_type);

        if result.diagnostics().is_empty() {
            if let Some(unknown) = result
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
        } else {
            TestRunOutcome::IncorrectlyErrored {
                files,
                errors: result.diagnostics().to_vec(),
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
        Some(Box::new(BabelJsxTestCase::new(path, code)))
    }
}
