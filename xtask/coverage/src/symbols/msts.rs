use crate::check_file_encoding;
use crate::runner::{
    create_unknown_node_in_tree_diagnostic, TestCase, TestCaseFile, TestCaseFiles, TestRunOutcome,
    TestSuite,
};
use regex::Regex;
use rome_js_parser::{ModuleKind, SourceType};
use rome_rowan::{AstNode, SyntaxKind};
use std::convert::TryFrom;
use std::path::{Path, PathBuf};
use std::str::FromStr;

const CASES_PATH: &str = "xtask/coverage/Typescript/tests/baselines/reference";
const BASE_PATH: &str = "xtask/coverage/Typescript";

#[derive(Debug)]
struct SymbolsMicrosoftTsTestCase {
    path: PathBuf,
    name: String,
}

impl SymbolsMicrosoftTsTestCase {
    fn new(path: &Path) -> Self {
        Self {
            path: path.to_path_buf(),
            name: path.file_name().unwrap().to_string_lossy().to_string(),
        }
    }
}

impl TestCase for SymbolsMicrosoftTsTestCase {
    fn name(&self) -> &str {
        &self.name
    }

    fn run(&self) -> TestRunOutcome {
        let code = "".to_string();

        let symbols = check_file_encoding(&self.path).unwrap();
        let expected = load_symbols_file(&symbols);

        let mut full_path = PathBuf::from_str(BASE_PATH).unwrap();
        full_path.push(expected.code_file);

        if !full_path.exists() {
            // We may be able to recover the code from the .symbols file
            let t = TestCaseFiles::single(self.name.clone(), code, SourceType::tsx());
            return TestRunOutcome::Passed(t);
        }

        let code = std::fs::read_to_string(&full_path).unwrap();
        let t = TestCaseFiles::single(self.name.clone(), code.clone(), SourceType::tsx());

        let r = rome_js_parser::parse(&code, 0, SourceType::tsx());
        let actual = rome_js_parser::symbols(r.syntax());

        if expected.symbols.len() != actual.symbols.len() {
            TestRunOutcome::IncorrectlyErrored {
                files: t,
                errors: vec![],
            }
        } else {
            for (expected, actual) in expected.symbols.iter().zip(actual.symbols) {
                if expected.name != actual.name {
                    return TestRunOutcome::IncorrectlyErrored {
                        files: t,
                        errors: vec![],
                    };
                }
            }

            TestRunOutcome::Passed(t)
        }
    }
}

#[derive(Default)]
pub(crate) struct SymbolsMicrosoftTsTestSuite;

impl TestSuite for SymbolsMicrosoftTsTestSuite {
    fn name(&self) -> &str {
        "symbols/msts"
    }

    fn base_path(&self) -> &str {
        CASES_PATH
    }

    fn is_test(&self, path: &Path) -> bool {
        match path.extension() {
            None => false,
            Some(ext) => ext == "symbols",
        }
    }

    fn load_test(&self, path: &Path) -> Option<Box<dyn TestCase>> {
        Some(Box::new(SymbolsMicrosoftTsTestCase::new(path)))
    }
}

#[derive(Debug)]
struct Symbol {
    name: String,
}

#[derive(Debug)]
struct SymbolsFile {
    code_file: PathBuf,
    symbols: Vec<Symbol>,
}

/// see xtask\coverage\Typescript\src\harness\typeWriter.ts
fn load_symbols_file(txt: &str) -> SymbolsFile {
    // const declText = `Decl(${ fileName }, ${ isLibFile ? "--" : declLineAndCharacter.line }, ${ isLibFile ? "--" : declLineAndCharacter.character })`;

    let mut lines = txt.lines();

    // === tests/cases/compiler/2dArrays.ts ===
    let code_file = lines.next().unwrap().replace("===", "").trim().to_string();

    let mut symbols = vec![];

    while let Some(line) = lines.next() {
        // >Cell : Symbol(Cell, Decl(2dArrays.ts, 0, 0))
        if line.starts_with(">") {
            let name = line
                .split(":")
                .next()
                .unwrap()
                .trim()
                .trim_start_matches(">");
            symbols.push(Symbol {
                name: name.to_string(),
            });
        }
    }

    SymbolsFile {
        code_file: PathBuf::from(code_file),
        symbols,
    }
}
