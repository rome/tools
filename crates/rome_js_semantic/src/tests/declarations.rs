use std::collections::{BTreeMap, HashMap};

use crate::assert_semantics;
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

assert_semantics! {
    ok_declaration_import, "/*START GLOBAL*/ import a/*#a @GLOBAL*/ from 'a'",
    ok_declaration_at_global_scope, "/*START GLOBAL*/ let b/*#b @GLOBAL*/ = 1;",
    ok_declaration_if, ";if/*START A*/ (true) { let b/*#b @A*/ = 1; }",
    ok_declaration_function, ";function/*START A*/ f(a/*#a @A*/) { let b/*#b @A*/ = 1; }",
    ok_declaration_arrow_function, ";(/*START A*/ a/*#a @A*/) => { let b/*#b @A*/ = 1; }",
    ok_declaration_at_for, ";for/*START A */ (let a/*#a @A*/;;) { let b/*#b @A*/ = 1; }",
    ok_declaration_at_for_of, ";for/*START A */ (const a/*#a @A*/ of []) { let b/*#b @A*/ = 1; }",
    ok_declaration_at_for_in, ";for/*START A */ (const a/*#a @A*/ in []) { let b/*#b @A*/ = 1; }",
    ok_declaration_class_constructor, ";class A { constructor/*START A*/ (a/*#a @A*/) { let b/*#b @A*/ = 1; } }",
    ok_declaration_class_getter, ";class A { get/*START A*/ name() { let b/*#b @A*/ = 1;} }",
    ok_declaration_class_setter, ";class A { set/*START A*/ name(a/*#a @A*/) { let b/*#b @A*/ = 1;} }",
    ok_declaration_try_catch, ";try {/*START A*/ let a/*#a @A*/ = 1;} catch/*START B*/ (b) { let b/*#b @B*/ = 1; }",
    ok_declaration_try_catch_finally, ";try {/*START A*/ let a/*#a @A*/ = 1;} catch/*START B*/ (b) { let b/*#b @B*/ = 1; } finally/*START C*/ { let c/*#c @C*/ = 1; }",
    ok_declaration_with_inner_scopes, r#";
function/*START SCOPE1*/ f() {
    let a/*#a1 @SCOPE1*/ = 1;
    console.log(a);
    if/*START SCOPE2*/ (true) {
        let a/*#a2 @SCOPE2*/ = 2;
        console.log(a);
    }
    console.log(a);
}
f();
"#,
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
fn assert(code: &str, test_name: &str) {
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
                    extract_declaration_assert(
                        &token,
                        &mut declarations_assertions,
                        code,
                        test_name,
                    );
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
                                    error_declaration_pointing_to_unknown_scope(
                                        code,
                                        token_range,
                                        test_name,
                                    );
                                }
                                assert_eq!(scope_start_range.start(), *scope_started_at);
                            }
                            None => error_declaration_pointing_to_unknown_scope(
                                code,
                                token_range,
                                test_name,
                            ),
                        }
                    }
                }
                _ => error_declaration_assertion_not_attached_to_a_declaration(
                    code,
                    token_range,
                    test_name,
                ),
            }
        } else {
            error_declaration_assertion_not_attached_to_a_declaration(code, token_range, test_name);
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
    test_name: &str,
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
        let files = SimpleFile::new(test_name.to_string(), code.into());
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
    test_name: &str,
) {
    let mut fix = code[assertion_range]
        .split("/*#")
        .next()
        .unwrap()
        .to_string();
    fix.push(' ');
    let files = SimpleFile::new(test_name.to_string(), code.into());
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

fn error_declaration_pointing_to_unknown_scope(
    code: &str,
    assertion_range: TextRange,
    test_name: &str,
) {
    let files = SimpleFile::new(test_name.to_string(), code.into());
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
