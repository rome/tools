use std::collections::{BTreeMap, HashMap};

use crate::semantic_events;
use crate::SemanticEvent;

use rome_console::{ConsoleExt, EnvConsole};
use rome_diagnostics::{file::SimpleFile, Applicability, Diagnostic};
use rome_js_syntax::JsSyntaxToken;
use rome_js_syntax::SourceType;
use rome_js_syntax::TextRange;
use rome_js_syntax::WalkEvent;
use rome_markup::markup;
use rome_rowan::NodeOrToken;

#[test]
pub fn ok_variable_declaration() {
    assert(
        r#"import a/*#A*/ from 'a';
let a/*#B*/ = 1;
function f(a/*#C*/) {}
(a/*#D*/) => {}
class A {
    constructor(a/*#E*/) {}
    set prop(a/*#F*/) {}
    f(a/*#G*/) {}
}
"#,
    );
}

/// This method helps testing the extraction of semantic events from parsed trees. It does this
/// iterating [SemanticEventIterator] and storing the range of each event. Later it iterates
/// the tree, but looking at tokens and its trailing comments, specifically looking at patterns
/// specifying which [SemanticEvent] should have being generated for a node containg the current token.
///
/// ### Available Patterns
///
/// #### Declaration Assertion
///
/// Test if the attached token is a declaration.  
/// Pattern: ```/*#<LABEL>*/```
///
/// Every declaration assertion will be tested if it matches a [SemanticEvent::Declaration].
///
/// Example:
/// ```js
/// let a/*#A*/ = 1;
/// ```
fn assert(code: &str) {
    let r = rome_js_parser::parse(code, 0, SourceType::js_module());

    if r.has_errors() {
        let files = SimpleFile::new(std::file!().to_string(), code.into());
        let mut console = EnvConsole::new(false);
        for diag in r.diagnostics() {
            console.log(markup! {
                {diag.display(&files)}
            });
        }
        panic!("Compilation error");
    }

    // Extract semantic events and index by range

    let mut event_by_range = HashMap::new();
    for event in semantic_events(r.syntax()) {
        if let SemanticEvent::DeclarationFound { range, .. } = &event {
            event_by_range.insert(*range, event);
        }
    }

    // Extract assertions inside comments

    let mut declarations_assertions = BTreeMap::new();

    for node in r.syntax().preorder_with_tokens() {
        if let WalkEvent::Enter(NodeOrToken::Token(token)) = node {
            let trivia = token.trailing_trivia();
            let text = trivia.text();

            if text.contains('#') {
                extract_declaration_assert(&token, &mut declarations_assertions, code);
            }
        }
    }

    // Check every declaration assertion is ok

    for (_, assertion_range) in declarations_assertions {
        if let Some(symbol) = event_by_range.get(&assertion_range) {
            match symbol {
                SemanticEvent::DeclarationFound { .. } => {
                    // No need to check anything on declarations
                }
                _ => {
                    error_declaration_assertion_not_attached_to_a_declaration(code, assertion_range)
                }
            }
        } else {
            error_declaration_assertion_not_attached_to_a_declaration(code, assertion_range);
        }
    }
}

fn extract_declaration_assert(
    token: &JsSyntaxToken,
    declarations_assertions: &mut BTreeMap<String, TextRange>,
    code: &str,
) {
    let trivia = token.trailing_trivia();
    let text = trivia.text();
    let old = declarations_assertions.insert(
        text.trim()
            .trim_start_matches("/*#")
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

fn error_declaration_assertion_not_attached_to_a_declaration(
    code: &str,
    assertion_range: TextRange,
) {
    let mut fix = code[assertion_range]
        .split("/*#")
        .next()
        .unwrap()
        .to_string();
    fix.push(' ');
    let files = SimpleFile::new(std::file!().to_string(), code.into());
    let d = Diagnostic::error(
        0,
        "",
        "Declaration assertions must be attached to symbols declarations.",
    )
    .suggestion(
        assertion_range,
        "Remove this assertion because no symbol declaration was found here.",
        fix,
        Applicability::Always,
    );
    let mut console = EnvConsole::new(false);
    console.log(markup! {
        {d.display(&files)}
    });
}
