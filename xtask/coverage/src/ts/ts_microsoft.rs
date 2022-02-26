use crate::check_file_encoding;
use crate::runner::{
    create_unknown_node_in_tree_diagnostic, TestCase, TestCaseFiles, TestRunOutcome, TestSuite,
};
use regex::Regex;
use rome_rowan::SyntaxKind;
use rslint_parser::{AstNode, ModuleKind, SourceType};
use std::path::Path;

const CASES_PATH: &str = "xtask/coverage/Typescript/tests/cases";
const REFERENCE_PATH: &str = "xtask/coverage/Typescript/tests/baselines/reference";

#[derive(Debug)]
struct MicrosoftTypeScriptTestCase {
    code: String,
    name: String,
}

impl MicrosoftTypeScriptTestCase {
    fn new(path: &Path, code: String) -> Self {
        let name = path.strip_prefix(CASES_PATH).unwrap().display().to_string();
        Self { name, code }
    }
}

impl TestCase for MicrosoftTypeScriptTestCase {
    fn name(&self) -> &str {
        &self.name
    }

    fn run(&self) -> TestRunOutcome {
        let TestCaseMetadata { files, run_options } = extract_metadata(&self.code, &self.name);

        let mut all_errors = Vec::new();
        let mut unknowns_errors = Vec::new();

        for file in &files {
            match file.parse().ok() {
                Ok(root) => {
                    if let Some(unknown) = root
                        .syntax()
                        .descendants()
                        .find(|descendant| descendant.kind().is_unknown())
                    {
                        unknowns_errors
                            .push(create_unknown_node_in_tree_diagnostic(file.id(), unknown));
                    }
                }
                Err(errors) => all_errors.extend(errors),
            }
        }

        let expected_errors = should_error(&self.name, &run_options);

        if all_errors.is_empty() && expected_errors {
            TestRunOutcome::IncorrectlyPassed(files)
        } else if !all_errors.is_empty() && !expected_errors {
            TestRunOutcome::IncorrectlyErrored {
                errors: all_errors,
                files,
            }
        } else if !unknowns_errors.is_empty() {
            TestRunOutcome::IncorrectlyErrored {
                errors: unknowns_errors,
                files,
            }
        } else {
            TestRunOutcome::Passed(files)
        }
    }
}

#[derive(Default)]
pub(crate) struct MicrosoftTypescriptTestSuite;

impl TestSuite for MicrosoftTypescriptTestSuite {
    fn name(&self) -> &str {
        "ts/microsoft"
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
        Some(Box::new(MicrosoftTypeScriptTestCase::new(path, code)))
    }
}

struct TestCaseMetadata {
    files: TestCaseFiles,
    run_options: Vec<String>,
}

/// TypeScript supports multiple files in a single test case.
/// These files start with `// @<option-name>: <option-value>` and are followed by the file's content.
/// This function extracts the individual files with their content and drops unsupported files.
fn extract_metadata(code: &str, path: &str) -> TestCaseMetadata {
    // Returns a match for a test option. Test options have the form `// @name: value`
    let options_regex =
        Regex::new(r"(?m)^/{2}\s*@(?P<name>\w+)\s*:\s*(?P<value>[^\r\n]*)").unwrap();

    let mut files = TestCaseFiles::new();
    let line_ending = infer_line_ending(code);
    let mut current_file_content = String::new();
    let mut current_file_name: Option<String> = None;
    let mut run_options: Vec<String> = vec![];

    for line in code.lines() {
        if let Some(option) = options_regex.captures(line) {
            let option_name = option.name("name").unwrap().as_str().to_lowercase();
            let option_value = option.name("value").unwrap().as_str().trim();

            if option_name == "alwaysstrict" {
                current_file_content.push_str(&format!("\"use strict\";{}", line_ending))
            } else if matches!(option_name.as_str(), "module" | "target") && files.is_empty() {
                run_options.extend(
                    option_value
                        .split(',')
                        .into_iter()
                        .map(|module_value| format!("{option_name}={}", module_value.trim())),
                );
            }

            if option_name != "filename" {
                continue; // omit options from the file content
            }

            if let Some(current_name) = current_file_name.take() {
                add_file_if_supported(
                    &mut files,
                    current_name,
                    std::mem::take(&mut current_file_content),
                );
            }

            current_file_name = Some(option_value.to_string());
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
        let path = Path::new(path);
        // Single file case without any options
        add_file_if_supported(
            &mut files,
            path.file_name().unwrap().to_str().unwrap().to_string(),
            current_file_content,
        );
    }

    TestCaseMetadata { files, run_options }
}

fn add_file_if_supported(files: &mut TestCaseFiles, name: String, content: String) {
    let path = Path::new(&name);
    // Skip files that aren't JS/TS files (JSON, CSS...)
    if let Some(mut source_type) = SourceType::from_path(path) {
        let is_module_regex = Regex::new("(import|export)\\s").unwrap();
        // A very basic heuristic to determine if a module is a `Script` or a `Module`.
        // The TypeScript parser automatically detects whatever a file is a module or a script
        // by the presence of any module syntax. Rome's parser doesn't support this today
        // because it would require moving any "strict mode" or "module" specific checks
        // into a second compiler pass. The reason this is needed is that the module syntax
        // may appear at the very end of the file after the parser has already processed
        // some syntax that is invalid in strict mode (for example, an "arguments" variable).
        if !is_module_regex.is_match(&content) {
            source_type = source_type.with_module_kind(ModuleKind::Script);
        }

        files.add(name, content, source_type)
    }
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

fn should_error(name: &str, run_options: &[String]) -> bool {
    let error_reference_file = Path::new(REFERENCE_PATH).join(
        Path::new(name)
            .with_extension("errors.txt")
            .file_name()
            .unwrap(),
    );

    if error_reference_file.exists() {
        return true;
    }

    run_options.iter().any(|option| {
        let errors_file_name = Path::new(name)
            .file_stem()
            .and_then(|name| name.to_str())
            .map(|name| format!("{name}({option}).errors.txt"))
            .unwrap();

        let path = Path::new(REFERENCE_PATH).join(&errors_file_name);

        path.exists()
    })
}
