use regex::Regex;
use rslint_parser::Syntax;
use std::path::Path;

use crate::runner::{TestCase, TestCaseFiles, TestRunOutcome, TestSuite};

const CASES_PATH: &str = "xtask/coverage/Typescript/tests/cases";
const REFERENCE_PATH: &str = "xtask/coverage/Typescript/tests/baselines/reference";

#[derive(Debug)]
struct TypeScriptTestCase {
    code: String,
    name: String,
}

impl TypeScriptTestCase {
    fn new(path: &Path, code: String) -> Self {
        let name = path.strip_prefix(CASES_PATH).unwrap().display().to_string();
        Self { name, code }
    }
}

impl TestCase for TypeScriptTestCase {
    fn name(&self) -> &str {
        &self.name
    }

    fn run(&self) -> TestRunOutcome {
        let files = extract_files(&self.code, &self.name);
        let mut all_errors = Vec::new();

        for file in &files {
            if let Some(errors) = file.parse().ok().err() {
                all_errors.extend(errors);
            }
        }

        let error_reference_file = Path::new(REFERENCE_PATH).join(
            Path::new(&self.name)
                .with_extension("errors.txt")
                .file_name()
                .unwrap(),
        );

        let expected_errors = error_reference_file.exists();

        if all_errors.is_empty() && expected_errors {
            TestRunOutcome::IncorrectlyPassed(files)
        } else if !all_errors.is_empty() && !expected_errors {
            TestRunOutcome::IncorrectlyErrored {
                errors: all_errors,
                files,
            }
        } else {
            TestRunOutcome::Passed(files)
        }
    }
}

#[derive(Default)]
pub(crate) struct TypeScriptTestSuite;

impl TestSuite for TypeScriptTestSuite {
    fn name(&self) -> &str {
        "TS"
    }

    fn base_path(&self) -> &str {
        CASES_PATH
    }

    fn is_test(&self, path: &Path) -> bool {
        match path.extension() {
            None => false,
            Some(ext) => ext == "ts",
        }
    }

    fn load_test(&self, path: &Path) -> Option<Box<dyn TestCase>> {
        let code = check_file_encoding(path)?;
        Some(Box::new(TypeScriptTestCase::new(path, code)))
    }
}

fn check_file_encoding(path: &std::path::Path) -> Option<String> {
    let buffer = std::fs::read(path).unwrap();
    let bom = buffer.get(0..3);
    //Utf16Be or // Utf16Le
    if let Some(&[0xfe, 0xff, _] | &[0xff, 0xfe, _]) = bom {
        None
    } else {
        std::str::from_utf8(buffer.as_slice())
            .ok()
            .map(str::to_string)
    }
}

/// TypeScript supports multiple files in a single test case.
/// These files start with `// @<option-name>: <option-value>` and are followed by the file's content.
/// This function extracts the individual files with their content and drops unsupported files.
fn extract_files(code: &str, path: &str) -> TestCaseFiles {
    // Returns a match for a test option. Test options have the form `// @name: value`
    let options_regex =
        Regex::new(r"(?m)^/{2}\s*@(?P<name>\w+)\s*:\s*(?P<value>[^\r\n]*)").unwrap();

    let mut files = TestCaseFiles::new();
    let line_ending = infer_line_ending(code);
    let mut current_file_content = String::new();
    let mut current_file_name: Option<String> = None;

    for line in code.lines() {
        if let Some(option) = options_regex.captures(line) {
            let option_name = option.name("name").unwrap().as_str();
            let option_value = option.name("value").unwrap().as_str();

            // TODO support @declaration
            if option_name.to_lowercase() != "filename" {
                continue; // omit options from the file content
            }

            if let Some(current_name) = current_file_name.take() {
                add_file_if_supported(
                    &mut files,
                    current_name,
                    std::mem::take(&mut current_file_content),
                );
            }

            current_file_name = Some(option_value.trim().to_string());
        } else {
            // regular content line
            if current_file_content.is_empty() && line.trim().is_empty() {
                // skip leading whitespace
                continue;
            }

            current_file_content.push_str(&format!("{}{}", line, line_ending));
        }
    }

    if let Some(current_name) = current_file_name.take() {
        add_file_if_supported(&mut files, current_name, current_file_content)
    } else if files.is_empty() {
        // Single file case without any options
        files.add(
            Path::new(path)
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
                .to_string(),
            code.to_string(),
            Syntax::default().typescript(),
        )
    }

    files
}

fn add_file_if_supported(files: &mut TestCaseFiles, name: String, content: String) {
    let syntax = if name.ends_with(".json") {
        // Don't add files that we don't support parsing (like JSON)
        return;
    } else if name.ends_with(".js") {
        Syntax::default().module()
    } else {
        Syntax::default().typescript()
    };

    files.add(name, content, syntax);
}

/// Detect the line ending used in the file
fn infer_line_ending(code: &str) -> &'static str {
    if let Some(index) = code.find('\r') {
        if index + 1 < code.len() && code.as_bytes()[index + 1] == b'\n' {
            "\r\n"
        } else {
            "\r"
        }
    } else {
        "\n"
    }
}
