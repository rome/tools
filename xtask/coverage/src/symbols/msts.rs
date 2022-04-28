use crate::check_file_encoding;
use crate::runner::{TestCase, TestCaseFiles, TestRunOutcome, TestSuite};
use rome_js_parser::SourceType;
use std::path::{Path, PathBuf};
use std::str::FromStr;

use super::utils::{parse_separated_list, parse_str, parse_until_chr, parse_whitespace0};

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
        let mut actual: Vec<_> = rome_js_parser::symbols::symbols(r.syntax())
            .filter(|x| !x.name().contains("\""))
            .collect();
        actual.sort_by(|l, r| l.range().start().cmp(&r.range().start()));

        if std::env::var("PRINT_CMP").is_ok() {
            let mut expecteds = expected.symbols.iter();
            let mut actuals = actual.iter();
            loop {
                let e = expecteds.next();
                let a = actuals.next();

                if e.is_none() && a.is_none() {
                    break;
                }

                if let Some(s) = e {
                    print!("{}", s.name);
                }

                print!(" - ");

                if let Some(s) = a {
                    print!("{:?}", s);
                }

                match (e, a) {
                    (Some(e), Some(a)) if e.name != a.name() => {
                        println!(" <<<<<<<<<<<<<<<<<<<< Diff here")
                    }
                    _ => {}
                }

                println!();
            }
        }

        if expected.symbols.len() != actual.len() {
            TestRunOutcome::IncorrectlyErrored {
                files: t,
                errors: vec![],
            }
        } else {
            for (expected, actual) in expected.symbols.iter().zip(actual) {
                let are_names_eq = expected.name == actual.name();
                // let are_paths_eq = expected.name == actual.name;
                //TODO check decls
                if !are_names_eq
                /*|| !are_paths_eq*/
                {
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
            Some(ext) if ext == "symbols" => {
                // only accepts if there is no *.errors.txt file
                let fullpath = path.with_extension("errors.txt");
                std::fs::metadata(fullpath).is_err()
            }
            _ => false,
        }
    }

    fn load_test(&self, path: &Path) -> Option<Box<dyn TestCase>> {
        Some(Box::new(SymbolsMicrosoftTsTestCase::new(path)))
    }
}

#[derive(Debug)]
#[allow(dead_code)]
struct Decl {
    file: String,
    row_start: Option<usize>,
    col_start: Option<usize>,
}

#[allow(dead_code)]
#[derive(Debug)]
struct Symbol {
    name: String,
    path: String,
    decls: Vec<Decl>,
}

struct SymbolsFile {
    code_file: PathBuf,
    symbols: Vec<Symbol>,
}

fn parse_decl(input: &str) -> Option<(&str, Decl)> {
    let (input, _) = parse_str(input, "Decl")?;
    let (input, _) = parse_whitespace0(input);
    let (input, _) = parse_str(input, "(")?;
    let (input, _) = parse_whitespace0(input);
    let (input, file) = parse_until_chr(input, |x| x.is_whitespace() || x == ',')?;
    let (input, _) = parse_whitespace0(input);
    let (input, _) = parse_str(input, ",")?;
    let (input, _) = parse_whitespace0(input);
    let (input, row_start) = parse_until_chr(input, |x| x.is_whitespace() || x == ',')?;
    let (input, _) = parse_whitespace0(input);
    let (input, _) = parse_str(input, ",")?;
    let (input, _) = parse_whitespace0(input);
    let (input, col_start) = parse_until_chr(input, |x| x.is_whitespace() || x == ')')?;
    let (input, _) = parse_whitespace0(input);
    let (input, _) = parse_str(input, ")")?;
    Some((
        input,
        Decl {
            file: file.to_string(),
            row_start: row_start.parse().ok(),
            col_start: col_start.parse().ok(),
        },
    ))
}

/// see xtask\coverage\Typescript\src\harness\typeWriter.ts
/// to understand how the symbol line is generated
/// example:
/// >Cell : Symbol(Cell, Decl(2dArrays.ts, 0, 0))
fn parse_symbol(input: &str) -> Option<Symbol> {
    let (input, _) = parse_str(input, ">")?;
    let (input, name) = parse_until_chr(input, |x| x.is_whitespace() || x == ':')?;
    if name.contains(".") || name.contains("[") || name.contains("\"") || name == "undefined" {
        return None;
    }
    let (input, _) = parse_whitespace0(input);
    let (input, _) = parse_str(input, ":")?;
    let (input, _) = parse_whitespace0(input);
    let (input, _) = parse_str(input, "Symbol")?;
    let (input, _) = parse_whitespace0(input);
    let (input, _) = parse_str(input, "(")?;
    let (input, path) = parse_until_chr(input, |x| x.is_whitespace() || x == ',' || x == ')')?;
    let (input, _) = parse_whitespace0(input);
    let decls = if !input.starts_with(")") {
        let (input, _) = parse_str(input, ",")?;
        let (input, _) = parse_whitespace0(input);

        let (_, decls) = parse_separated_list(
            input,
            parse_decl,
            |s| parse_str(s, ",").map(|x| x.0).unwrap_or(s),
            |s| parse_whitespace0(s).0,
        );
        decls
    } else {
        vec![]
    };

    Some(Symbol {
        name: name.to_string(),
        path: path.to_string(),
        decls,
    })
}

fn load_symbols_file(txt: &str) -> SymbolsFile {
    let mut lines = txt.lines();

    // first line example
    // === tests/cases/compiler/2dArrays.ts ===
    let code_file = lines.next().unwrap().replace("===", "").trim().to_string();

    let mut symbols = vec![];

    for line in lines {
        if let Some(symbol) = parse_symbol(line) {
            symbols.push(symbol);
        }
    }

    SymbolsFile {
        code_file: PathBuf::from(code_file),
        symbols,
    }
}
