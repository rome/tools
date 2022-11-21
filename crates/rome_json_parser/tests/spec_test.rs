use rome_console::fmt::{Formatter, Termcolor};
use rome_console::markup;
use rome_diagnostics::display::PrintDiagnostic;
use rome_diagnostics::termcolor;
use rome_diagnostics::DiagnosticExt;
use rome_diagnostics::FileId;
use rome_json_parser::Lexer;
use std::fmt::Write;
use std::fs;
use std::path::Path;

#[derive(Copy, Clone)]
pub enum ExpectedOutcome {
    Pass,
    Fail,
    Undefined,
}

pub fn run(test_case: &str, _snapshot_name: &str, test_directory: &str, outcome: &str) {
    let outcome = match outcome {
        "ok" => ExpectedOutcome::Pass,
        "error" => ExpectedOutcome::Fail,
        "undefined" => ExpectedOutcome::Undefined,
        _ => panic!("Invalid expected outcome {outcome}"),
    };

    let test_case_path = Path::new(test_case);

    let file_name = test_case_path
        .file_name()
        .expect("Expected test to have a file name")
        .to_str()
        .expect("File name to be valid UTF8");

    let content = fs::read_to_string(test_case_path)
        .expect("Expected test path to be a readable file in UTF8 encoding");

    let mut lexer = Lexer::from_str(&content, FileId::zero());

    let tokens: Vec<_> = lexer.by_ref().collect();
    let diagnostics = lexer.finish();

    let mut snapshot = String::new();
    writeln!(snapshot, "\n## Input\n\n```json\n{content}\n```\n\n").unwrap();

    if !diagnostics.is_empty() {
        let mut diagnostics_buffer = termcolor::Buffer::no_color();

        let termcolor = &mut Termcolor(&mut diagnostics_buffer);
        let mut formatter = Formatter::new(termcolor);

        for diagnostic in diagnostics {
            let error = diagnostic
                .clone()
                .with_file_path(&file_name)
                .with_file_source_code(&content);

            formatter
                .write_markup(markup! {
                    {PrintDiagnostic(&error)}
                })
                .expect("failed to emit diagnostic");
        }

        let formatted_diagnostics =
            std::str::from_utf8(diagnostics_buffer.as_slice()).expect("non utf8 in error buffer");

        if matches!(outcome, ExpectedOutcome::Pass) {
            panic!("Expected no errors to be present in a test case that is expected to pass but the following diagnostics are present:\n{formatted_diagnostics}")
        }

        writeln!(snapshot, "## Diagnostics\n\n```").unwrap();
        snapshot.write_str(formatted_diagnostics).unwrap();

        writeln!(snapshot, "```\n").unwrap();
    }

    writeln!(snapshot, "## Tokens").unwrap();
    writeln!(snapshot, "\n```").unwrap();

    for token in tokens {
        writeln!(
            snapshot,
            "{:?}: `{}` ({:?})",
            token.kind(),
            &content[token.range()],
            token.range()
        )
        .unwrap();
    }

    writeln!(snapshot, "```").unwrap();

    insta::with_settings!({
        prepend_module_to_snapshot => false,
        snapshot_path => &test_directory,
    }, {
        insta::assert_snapshot!(file_name, snapshot);
    });
}
