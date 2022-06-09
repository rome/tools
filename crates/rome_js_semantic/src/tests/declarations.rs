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

use super::extract_scope_assertion;
use super::ScopeAssertionType;

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

#[test]
pub fn ok_variable_declaration_scope() {
    assert("/*START GLOBAL*/ let b/*#b @GLOBAL*/ = 1;");
    assert("if (true) {/*START A*/ let b/*#b @A*/ = 1;}");
    assert("function f() {/*START A*/ let b/*#b @A*/ = 1;}");
    assert("for (const a of []) {/*START A*/ let b/*#b @A*/ = 1;}");
    assert("for (const a in []) {/*START A*/ let b/*#b @A*/ = 1;}");
    assert("() => {/*START A*/ let b/*#b @A*/ = 1;}");

    assert("class A { constructor () {/*START A*/ let b/*#b @A*/ = 1;} }");
    assert("class A { get name() {/*START A*/ let b/*#b @A*/ = 1;} }");
    assert("class A { set name(v) {/*START A*/ let b/*#b @A*/ = 1;} }");

    assert("try {/*START A*/ let b/*#b1 @A*/ = 1;} catch(e) {/*START B*/ let b/*#b2 @B*/ = 1;} finally {/*START C*/ let b/*#b3 @C*/ = 1;}");
}

#[test]
pub fn ok_variable_declaration_with_inner_scope() {
    assert(
        r#"
function f() {/*START SCOPE1*/
    let a/*#a1 @SCOPE1*/ = 1;
    console.log(a);
    if (true) {/*START SCOPE2*/
        let a/*#a2 @SCOPE2*/ = 2;
        console.log(a);
    }
    console.log(a);
}
f();
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
///
/// To also assert the scope of a declaration use the '@'. The only requirement is that
/// we need to specify where the scope start. Example:
/// ```js
/// function f() {/*START FSCOPE*/let a/*#A @FSCOPE*/ = 1;}
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
        match &event {
            SemanticEvent::DeclarationFound { range, .. } => {
                event_by_range.insert(*range, event);
            }
            SemanticEvent::ScopeStarted { range } => {
                event_by_range.insert(*range, event);
            }
            _ => (),
        }
    }

    // Extract assertions inside comments

    let mut declarations_assertions = BTreeMap::new();
    let mut scope_start_assertions = BTreeMap::new();

    for node in r.syntax().preorder_with_tokens() {
        if let WalkEvent::Enter(NodeOrToken::Token(token)) = node {
            let pieces = token
                .leading_trivia()
                .pieces()
                .chain(token.trailing_trivia().pieces());
            for piece in pieces {
                let text = piece.text();
                if text.contains('#') {
                    extract_declaration_assert(&token, &mut declarations_assertions, code);
                } else if text.contains("/*START") {
                    extract_scope_assertion(
                        &token,
                        &piece,
                        &mut scope_start_assertions,
                        code,
                        ScopeAssertionType::Start,
                    );
                }
            }
        }
    }

    // Check every declaration assertion is ok

    for (_, DeclarationAssertion { token_range, scope }) in declarations_assertions {
        if let Some(symbol) = event_by_range.get(&token_range) {
            match symbol {
                SemanticEvent::DeclarationFound {
                    scope_started_at, ..
                } => {
                    // Test this declaration is pointing to the correct scope start
                    if let Some(scope) = scope {
                        match scope_start_assertions.get(&scope) {
                            Some(scope_start_range) => {
                                if scope_start_range.start() != *scope_started_at {
                                    error_declaration_pointing_to_unknown_scope(code, token_range);
                                }
                                assert_eq!(scope_start_range.start(), *scope_started_at);
                            }
                            None => error_declaration_pointing_to_unknown_scope(code, token_range),
                        }
                    }
                }
                _ => error_declaration_assertion_not_attached_to_a_declaration(code, token_range),
            }
        } else {
            error_declaration_assertion_not_attached_to_a_declaration(code, token_range);
        }
    }
}

struct DeclarationAssertion {
    token_range: TextRange,
    scope: Option<String>,
}

fn extract_declaration_assert(
    token: &JsSyntaxToken,
    declarations_assertions: &mut BTreeMap<String, DeclarationAssertion>,
    code: &str,
) {
    let trivia = token.trailing_trivia();
    let text = trivia.text();

    let parts = text
        .trim()
        .trim_start_matches("/*#")
        .trim_end_matches("*/")
        .trim()
        .to_string();
    let mut parts = parts.split(" ");

    let decl = parts.next().unwrap();
    let mut scope = None;

    for part in parts {
        if part.starts_with("@") {
            scope = Some(part.trim_start_matches('@').to_string());
        }
    }

    let old = declarations_assertions.insert(
        decl.to_string(),
        DeclarationAssertion {
            token_range: token.text_range(),
            scope,
        },
    );

    // If there is already an assertion with the same name. Suggest a rename

    if let Some(old) = old {
        let files = SimpleFile::new(std::file!().to_string(), code.into());
        let d = Diagnostic::error(0, "", "Assertion label conflict.")
            .primary(
                token.text_range(),
                "There is already a assertion with the same name. Consider renaming this one.",
            )
            .secondary(old.token_range, "Previous assertion");

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

fn error_declaration_pointing_to_unknown_scope(code: &str, assertion_range: TextRange) {
    let files = SimpleFile::new(std::file!().to_string(), code.into());
    let d = Diagnostic::error(
        0,
        "",
        "Declaration assertions is pointing to the wrong scope",
    )
    .primary(assertion_range, "");

    let mut console = EnvConsole::new(false);
    console.log(markup! {
        {d.display(&files)}
    });
}
