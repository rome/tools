use rome_js_semantic::SemanticEvent;
use rome_js_syntax::SourceType;

use crate::check_file_encoding;
use crate::runner::{TestCase, TestCaseFiles, TestRunOutcome, TestSuite};
use std::path::{Path, PathBuf};
use std::str::FromStr;

use super::utils::{parse_separated_list, parse_str, parse_until_chr, parse_whitespace0};

const CASES_PATH: &str = "xtask/coverage/Typescript/tests/baselines/reference";
const BASE_PATH: &str = "xtask/coverage/Typescript";

#[derive(Debug)]
struct SymbolsMicrosoftTestCase {
    path: PathBuf,
    name: String,
}

impl SymbolsMicrosoftTestCase {
    fn new(path: &Path) -> Self {
        Self {
            path: path.to_path_buf(),
            name: path.file_name().unwrap().to_string_lossy().to_string(),
        }
    }
}

impl TestCase for SymbolsMicrosoftTestCase {
    fn name(&self) -> &str {
        &self.name
    }

    fn run(&self) -> TestRunOutcome {
        let symbols = check_file_encoding(&self.path).unwrap();
        let expected = load_symbols_file(&symbols);

        let mut full_path = PathBuf::from_str(BASE_PATH).unwrap();
        full_path.push(expected.code_file);

        // Some .symbols files point to .ts files that do no exist.
        // In this case, the best we can do is recover the original source from
        // the .symbol file itself.
        let code = if !full_path.exists() {
            tracing::warn!("Not Found: {full_path:?}");
            symbols.lines().fold(String::new(), |mut s, line| {
                if !line.starts_with('>')
                    && !line.starts_with("=== ")
                    && !line.starts_with("///<reference ")
                {
                    s.push_str(line);
                    s.push('\n');
                }
                s
            })
        } else {
            match std::fs::read_to_string(&full_path) {
                Ok(code) => code,
                Err(_) => {
                    return TestRunOutcome::IncorrectlyErrored {
                        files: TestCaseFiles::single(
                            self.name.clone(),
                            "".to_string(),
                            SourceType::tsx(),
                        ),
                        errors: vec![],
                    }
                }
            }
        };

        let t = TestCaseFiles::single(self.name.clone(), code.clone(), SourceType::tsx());

        let r = rome_js_parser::parse(&code, 0, SourceType::tsx());
        let mut actual: Vec<_> = rome_js_semantic::semantic_events(r.syntax())
            .into_iter()
            .filter(|x| {
                // We filter any event pointing to string literals.
                // We do the same below because TS classifies some string literals as symbols and we also
                // filter them below.
                let name = x.str(&code);
                if name.contains('\"') || name.contains('\'') {
                    false
                } else {
                    match x {
                        SemanticEvent::DeclarationFound { .. } | SemanticEvent::Read { .. } => true,
                        _ => false,
                    }
                }
            })
            .collect();
        actual.sort_by_key(|x| x.range().start());

        // Print to debug! detailed information
        // on symbols that are different from the
        // expected
        let mut expecteds = expected.symbols.iter();
        let mut actuals = actual.iter();
        loop {
            let expected = expecteds.next();
            let actual = actuals.next();

            if expected.is_none() && actual.is_none() {
                break;
            }

            let mut debug_text = String::new();

            debug_text.push_str("expected: ");

            if let Some(symbol) = expected {
                debug_text.push_str(&format!("[{}]", &symbol.name));
            }

            debug_text.push_str(" - actual: ");

            if let Some(actual) = actual {
                let name = actual.str(&code).trim();
                debug_text.push_str(&format!("[{}]", name));
            }

            match (expected, actual) {
                (Some(expected), Some(actual)) if expected.name != actual.str(&code).trim() => {
                    debug_text.push_str(" <<<<<<<<<<<<<<<<<<<< Diff here");
                }
                _ => {}
            }

            tracing::debug!("{}", debug_text);
        }

        if expected.symbols.len() != actual.len() {
            TestRunOutcome::IncorrectlyErrored {
                files: t,
                errors: vec![],
            }
        } else {
            for (expected, actual) in expected.symbols.iter().zip(actual) {
                let are_names_eq = expected.name == actual.str(&code);
                if !are_names_eq {
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
pub(crate) struct SymbolsMicrosoftTestSuite;

impl TestSuite for SymbolsMicrosoftTestSuite {
    fn name(&self) -> &str {
        "symbols/microsoft"
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
        Some(Box::new(SymbolsMicrosoftTestCase::new(path)))
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

#[derive(Debug)]
struct SymbolsFile {
    code_file: PathBuf,
    symbols: Vec<Symbol>,
}

/// This function parses lines like:
/// >Cell : Symbol(Cell, Decl(2dArrays.ts, 0, 0))
///   |              |     |     |         \--+---> line and column ofthe first char of the leading trivia where the declaration
///   |              |     |     \--> File where the declaration of this symbol is
///   |              |     \--> States that this Symbol is a declaration
///   |              \--> Complete Path of the Symbol
///   \--> text of the symbol
/// To understand how the Typescript codebase generate this line
/// see xtask\coverage\Typescript\src\harness\typeWriter.ts
fn parse_symbol(input: &str) -> Option<Symbol> {
    let (input, _) = parse_str(input, ">")?;
    let (input, name) = parse_until_chr(input, |x| x.is_whitespace() || x == ':')?;
    if name.contains('.')
        || name.contains('[')
        || name.contains('\"')
        || name.contains('\'')
        || name == "undefined"
    {
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
    let decls = if !input.starts_with(')') {
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

/// This method will load .symbols file
/// from the Typescript test suite.
/// Each file is composed of:
/// first line pointing to the original ts file;
/// For each line of the source file: the actual ts line;
/// and if the line contains any symbols, one line per symbol as described by the method [parse_symbol];
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
