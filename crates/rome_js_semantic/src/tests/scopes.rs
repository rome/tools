use std::collections::{BTreeMap, HashMap};

use crate::{semantic_events, SemanticEvent};
use rome_console::{markup, ConsoleExt, EnvConsole};
use rome_diagnostics::{file::SimpleFile, Applicability, Diagnostic};
use rome_js_syntax::{JsSyntaxToken, SourceType, TextRange, TextSize, WalkEvent};
use rome_rowan::{Direction, NodeOrToken};

use super::{extract_scope_assertion, ScopeAssertionType};

#[test]
pub fn ok_scope_blocks() {
    assert("/*START GLOBAL*//*END GLOBAL*/");
    assert("if (true) {/*START A*/ }/*END A*/");
    assert("function f() {/*START A*/ }/*END A*/");
    assert("for (const a of []) {/*START A*/ }/*END A*/");
    assert("for (const a in []) {/*START A*/ }/*END A*/");
    assert("() => {/*START A*/ }/*END A*/");

    assert("class A { constructor () {/*START A*/ }/*END A*/ }");
    assert("class A { get name() {/*START A*/ }/*END A*/ }");
    assert("class A { set name(v) {/*START A*/ }/*END A*/ }");

    assert("try {/*START A*/ }/*END A*/ catch(e) {/*START B*/ }/*END B*/ finally {/*START C*/ }/*END C*/");
}

/// This method helps testing scope resolution. It does this
/// iterating [SemanticEventIterator] and storing where each scope start and end. Later it iterates
/// the tree looking at tokens with trailing comments following a specifically patterns
/// specifying if a scope has started or ended.
///
/// ### Available Patterns
///
/// #### Scope Start Assertion
///
/// Test if the attached token starts a new scope.  
/// Pattern: ```/*START <LABEL>*/```
///
/// Every scope start assertion will be tested if it matches a [SemanticEvent::ScopeStarted].
///
/// Example:
/// ```js
/// function f() {/*START SCOPE1*/ }
/// ```
///
/// /// #### Scope End Assertion
///
/// Test if the attached token ends a scope.  
/// Pattern: ```/*END <LABEL>*/```
///
/// Every scope end assertion will be tested if it matches a [SemanticEvent::ScopeEnded].
/// This assertion also tests if the event and the assertion start scope matches.
///
/// Example:
/// ```js
/// function f() {/*START SCOPE1*/ }/*END SCOPE1*/
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

    let mut events_by_range: HashMap<TextSize, Vec<SemanticEvent>> = HashMap::new();
    for event in semantic_events(r.syntax()) {
        match &event {
            SemanticEvent::ScopeStarted { range } => {
                let v = events_by_range.entry(range.start()).or_default();
                v.push(event);
            }
            SemanticEvent::ScopeEnded { range, .. } => {
                let v = events_by_range.entry(range.end()).or_default();
                v.push(event);
            }
            _ => {}
        }
    }

    // Extract assertions inside comments

    let mut scope_start_assertions = BTreeMap::new();
    let mut scope_end_assertions = BTreeMap::new();

    for node in r.syntax().preorder_with_tokens(Direction::Next) {
        if let WalkEvent::Enter(NodeOrToken::Token(token)) = node {
            let pieces = token
                .leading_trivia()
                .pieces()
                .chain(token.trailing_trivia().pieces());
            for piece in pieces {
                let text = piece.text();
                if text.contains("/*START") {
                    extract_scope_assertion(
                        &token,
                        &piece,
                        &mut scope_start_assertions,
                        code,
                        ScopeAssertionType::Start,
                    );
                } else if text.contains("/*END") {
                    extract_scope_assertion(
                        &token,
                        &piece,
                        &mut scope_end_assertions,
                        code,
                        ScopeAssertionType::End,
                    );
                }
            }
        }
    }

    // Check every scope start assertion is ok

    for assertion_range in scope_start_assertions.values() {
        if let Some(events) = events_by_range.get(&assertion_range.start()) {
            let is_at_least_one_scope_start = events
                .iter()
                .any(|e| matches!(e, SemanticEvent::ScopeStarted { .. }));
            if !is_at_least_one_scope_start {
                error_scope_assertion_not_attached_to_a_scope_event(
                    code,
                    *assertion_range,
                    ScopeAssertionType::Start,
                )
            }
        } else {
            error_scope_assertion_not_attached_to_a_scope_event(
                code,
                *assertion_range,
                ScopeAssertionType::Start,
            );
        }
    }

    // Check every scope end assertion is ok

    for (scope_end_assertion_label, scope_end_assertion_range) in scope_end_assertions {
        // Check we have a scope start with the same label.
        let scope_start_assertions_range =
            match scope_start_assertions.get(&scope_end_assertion_label) {
                Some(scope_start_assertions_range) => *scope_start_assertions_range,
                None => {
                    error_scope_end_assertion_points_to_non_existing_scope_start_assertion(
                        code,
                        &scope_end_assertion_range,
                    );
                    continue;
                }
            };

        if let Some(events) = events_by_range.get(&scope_end_assertion_range.end()) {
            // At least one of the events should be a scope start starting
            // where we expect
            let e = events.iter().find(|event| match event {
                SemanticEvent::ScopeEnded { started_at, .. }
                    if *started_at == scope_start_assertions_range.start() =>
                {
                    true
                }
                _ => false,
            });

            if e.is_none() {
                todo!()
            }
        } else {
            error_scope_assertion_not_attached_to_a_scope_event(
                code,
                scope_end_assertion_range,
                ScopeAssertionType::End,
            );
        }
    }
}

fn error_scope_assertion_not_attached_to_a_scope_event(
    code: &str,
    assertion_range: TextRange,
    assertion_type: ScopeAssertionType,
) {
    let trim_start = match assertion_type {
        ScopeAssertionType::Start => "/*START",
        ScopeAssertionType::End => "/*END",
    };

    let mut fix = code[assertion_range]
        .split(trim_start)
        .next()
        .unwrap()
        .to_string();
    fix.push(' ');
    let files = SimpleFile::new(std::file!().to_string(), code.into());
    let d = Diagnostic::error(
        0,
        "",
        match assertion_type {
            ScopeAssertionType::Start => "Scope start assertions must be attached to scope starts.",
            ScopeAssertionType::End => "Scope end assertions must be attached to scope ends.",
        },
    )
    .suggestion(
        assertion_range,
        match assertion_type {
            ScopeAssertionType::Start => {
                "Remove this assertion because no scope start was found here."
            }
            ScopeAssertionType::End => "Remove this assertion because no scope end was found here.",
        },
        fix,
        Applicability::Always,
    );
    let mut console = EnvConsole::new(false);
    console.log(markup! {
        {d.display(&files)}
    });
}

fn error_scope_end_assertion_points_to_non_existing_scope_start_assertion(
    code: &str,
    range: &TextRange,
) {
    let files = SimpleFile::new(std::file!().to_string(), code.into());
    let d = Diagnostic::error(0, "", "Scope start assertion not found.").primary(
        range,
        "This scope end assertion points to a non-existing scope start assertion.",
    );

    let mut console = EnvConsole::new(false);
    console.log(markup! {
        {d.display(&files)}
    });
    panic!("Scope start assertion not found.");
}
