use std::collections::BTreeMap;

use rome_console::{markup, ConsoleExt, EnvConsole};
use rome_diagnostics::{file::SimpleFile, Diagnostic};
use rome_js_syntax::{JsLanguage, JsSyntaxToken, TextRange};
use rome_rowan::SyntaxTriviaPiece;

pub mod declarations;
pub mod scopes;

enum ScopeAssertionType {
    Start,
    End,
}

fn extract_scope_assertion(
    token: &JsSyntaxToken,
    piece: &SyntaxTriviaPiece<JsLanguage>,
    assertions: &mut BTreeMap<String, TextRange>,
    code: &str,
    assertion_type: ScopeAssertionType,
) {
    let trim_start = match assertion_type {
        ScopeAssertionType::Start => "/*START",
        ScopeAssertionType::End => "/*END",
    };

    let text = piece.text();
    let old = assertions.insert(
        text.trim()
            .trim_start_matches(trim_start)
            .trim_end_matches("*/")
            .trim()
            .to_string(),
        token.text_range(),
    );

    // If there is already an assertion with the same name. Suggest a rename

    if let Some(old) = old {
        let files = SimpleFile::new(std::file!().to_string(), code.into());
        let d = Diagnostic::error(0, "", "Assertion label conflict.")
            .primary(
                token.text_range(),
                "There is already a assertion with the same name. Consider renaming this one.",
            )
            .secondary(old, "Previous assertion");

        let mut console = EnvConsole::new(false);
        console.log(markup! {
            {d.display(&files)}
        });

        panic!("Assertion label conflict")
    }
}
