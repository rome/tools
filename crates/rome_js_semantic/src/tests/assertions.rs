use crate::{semantic_events, SemanticEvent};
use rome_console::{markup, ConsoleExt, EnvConsole};
use rome_diagnostics::{file::SimpleFile, Diagnostic};
use rome_js_syntax::{JsAnyRoot, JsSyntaxToken, SourceType, TextRange, TextSize, WalkEvent};
use rome_rowan::{AstNode, NodeOrToken};
use std::collections::{BTreeMap, HashMap};

/// This method helps testing scope resolution. It does this
/// iterating [SemanticEventIterator] and storing where each scope start and end. Later it iterates
/// the tree looking at tokens with trailing comments following a specifically patterns
/// specifying if a scope has started or ended.
///
/// ### Available Patterns
///
/// #### Declaration Assertion
///
/// Test if the attached token is a declaration.
/// Pattern: ```/*# <LABEL> */
///
/// Every declaration assertion will be tested if it matches a [SemanticEvent::DeclarationFound].
///
/// Example:
/// ```js
/// let a/*#A*/ = 1;
/// ```
///
/// #### Read Assertion
///
/// Test if the attached token is reference "reading" the value of a symbol.
/// Pattern: ```/*READ <LABEL> */
///
/// /// Example:
/// ```js
/// let a/*#A*/ = 1;
/// let b = a/*READ A*/ + 1;
/// ```
///
/// #### At Scope Assertion
///
/// Test if the attached token is a declaration that lives inside the specified scope.
/// Pattern: ```/*@ <LABEL> */```
///
/// Every at scope assertion will be tested if it matches the  ```scope_started_at``` field of [SemanticEvent::DeclarationFound].
///
/// Example:
/// ```js
/// function f() {/*START A*/ let a/*#a*//*@A*/ = 1; }
/// ```
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
/// #### Scope End Assertion
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
///
/// #### Unique Assertion
///
/// Test if only one event is attached to the token.
/// Pattern: ```/*UNIQUE*/```
///
/// Example:
/// ```js
/// "for(;;) ;/*UNIQUE*/;"
/// ```
///
/// #### No events Assertion
///
/// Test with there are no events attached to the token.
///
/// Example:
/// ```js
/// if(true) ;/*NOEVENT*/;
/// ```
pub fn assert(code: &str, test_name: &str) {
    let r = rome_js_parser::parse(code, 0, SourceType::js_module());

    if r.has_errors() {
        let files = SimpleFile::new(test_name.to_string(), code.into());
        let mut console = EnvConsole::new(false);
        for diag in r.diagnostics() {
            console.log(markup! {
                {diag.display(&files)}
            });
        }
        panic!("Compilation error");
    }

    // Extract semantic events and index by range

    let mut events_by_pos: HashMap<TextSize, Vec<SemanticEvent>> = HashMap::new();
    for event in semantic_events(r.syntax()) {
        let pos = match &event {
            SemanticEvent::DeclarationFound { range, .. } => range.start(),
            SemanticEvent::ScopeStarted { range } => range.start(),
            SemanticEvent::ScopeEnded { range, .. } => range.end(),
            SemanticEvent::Read { range, .. } => range.start(),
            SemanticEvent::HoistedRead { range, .. } => range.start(),
            SemanticEvent::UnresolvedReference { range } => range.start(),
        };

        let v = events_by_pos.entry(pos).or_default();
        v.push(event);
    }

    let assertions = SemanticAssertions::from_root(r.tree(), code, test_name);

    // check

    assertions.check(code, test_name, events_by_pos);
}

#[derive(Clone, Debug)]
struct DeclarationAssertion {
    range: TextRange,
    declaration_name: String,
}

#[derive(Clone, Debug)]
struct ReadAssertion {
    range: TextRange,
    declaration_asertion_name: String,
}

#[derive(Clone, Debug)]
struct AtScopeAssertion {
    range: TextRange,
    scope_name: String,
}

#[derive(Clone, Debug)]
struct ScopeStartAssertion {
    range: TextRange,
    scope_name: String,
}

#[derive(Clone, Debug)]
struct ScopeEndAssertion {
    range: TextRange,
    scope_name: String,
}

#[derive(Clone, Debug)]
struct NoEventAssertion {
    range: TextRange,
}

#[derive(Clone, Debug)]
struct UniqueAssertion {
    range: TextRange,
}

#[derive(Clone, Debug)]
enum SemanticAssertion {
    Declaration(DeclarationAssertion),
    Read(ReadAssertion),
    ScopeStart(ScopeStartAssertion),
    ScopeEnd(ScopeEndAssertion),
    AtScope(AtScopeAssertion),
    NoEvent(NoEventAssertion),
    Unique(UniqueAssertion),
}

impl SemanticAssertion {
    fn try_from(token: &JsSyntaxToken, assertion_text: &str) -> Option<Self> {
        if assertion_text.starts_with("/*#") {
            let name = assertion_text
                .trim()
                .trim_start_matches("/*#")
                .trim_end_matches("*/")
                .trim()
                .to_string();

            Some(SemanticAssertion::Declaration(DeclarationAssertion {
                range: token.text_range(),
                declaration_name: name,
            }))
        } else if assertion_text.starts_with("/*READ ") {
            let symbol_name = assertion_text
                .trim()
                .trim_start_matches("/*READ ")
                .trim_end_matches("*/")
                .trim()
                .to_string();

            Some(SemanticAssertion::Read(ReadAssertion {
                range: token.text_range(),
                declaration_asertion_name: symbol_name,
            }))
        } else if assertion_text.contains("/*START") {
            let scope_name = assertion_text
                .trim()
                .trim_start_matches("/*START")
                .trim_end_matches("*/")
                .trim()
                .to_string();
            Some(SemanticAssertion::ScopeStart(ScopeStartAssertion {
                range: token.text_range(),
                scope_name,
            }))
        } else if assertion_text.contains("/*END") {
            let scope_name = assertion_text
                .trim()
                .trim_start_matches("/*END")
                .trim_end_matches("*/")
                .trim()
                .to_string();
            Some(SemanticAssertion::ScopeEnd(ScopeEndAssertion {
                range: token.text_range(),
                scope_name,
            }))
        } else if assertion_text.starts_with("/*@") {
            let scope_name = assertion_text
                .trim()
                .trim_start_matches("/*@")
                .trim_end_matches("*/")
                .trim()
                .to_string();
            Some(SemanticAssertion::AtScope(AtScopeAssertion {
                range: token.text_range(),
                scope_name,
            }))
        } else if assertion_text.contains("/*NOEVENT") {
            Some(SemanticAssertion::NoEvent(NoEventAssertion {
                range: token.text_range(),
            }))
        } else if assertion_text.contains("/*UNIQUE") {
            Some(SemanticAssertion::Unique(UniqueAssertion {
                range: token.text_range(),
            }))
        } else {
            None
        }
    }
}

#[derive(Debug)]
struct SemanticAssertions {
    declarations_assertions: BTreeMap<String, DeclarationAssertion>,
    read_assertions: Vec<ReadAssertion>,
    at_scope_assertions: Vec<AtScopeAssertion>,
    scope_start_assertions: BTreeMap<String, ScopeStartAssertion>,
    scope_end_assertions: Vec<ScopeEndAssertion>,
    no_events: Vec<NoEventAssertion>,
    uniques: Vec<UniqueAssertion>,
}

impl SemanticAssertions {
    fn from_root(root: JsAnyRoot, code: &str, test_name: &str) -> Self {
        let mut declarations_assertions: BTreeMap<String, DeclarationAssertion> = BTreeMap::new();
        let mut read_assertions = vec![];
        let mut at_scope_assertions = vec![];
        let mut scope_start_assertions: BTreeMap<String, ScopeStartAssertion> = BTreeMap::new();
        let mut scope_end_assertions = vec![];
        let mut no_events = vec![];
        let mut uniques = vec![];

        for node in root
            .syntax()
            .preorder_with_tokens(rome_rowan::Direction::Next)
        {
            if let WalkEvent::Enter(NodeOrToken::Token(token)) = node {
                let pieces = token
                    .leading_trivia()
                    .pieces()
                    .chain(token.trailing_trivia().pieces());
                for piece in pieces {
                    let text = piece.text();

                    let assertion = SemanticAssertion::try_from(&token, text);
                    match assertion {
                        Some(SemanticAssertion::Declaration(assertion)) => {
                            // Declaration assertions names cannot clash
                            let old = declarations_assertions
                                .insert(assertion.declaration_name.clone(), assertion)
                                .map(|x| x.range);
                            if let Some(old) = old {
                                error_assertion_name_clash(&token, code, test_name, old);
                            }
                        }
                        Some(SemanticAssertion::Read(assertion)) => {
                            read_assertions.push(assertion);
                        }
                        Some(SemanticAssertion::ScopeStart(assertion)) => {
                            // Scope start assertions names cannot clash
                            let old = scope_start_assertions
                                .insert(assertion.scope_name.clone(), assertion)
                                .map(|x| x.range);
                            if let Some(old) = old {
                                error_assertion_name_clash(&token, code, test_name, old);
                            }
                        }
                        Some(SemanticAssertion::ScopeEnd(assertion)) => {
                            scope_end_assertions.push(assertion);
                        }
                        Some(SemanticAssertion::AtScope(assertion)) => {
                            at_scope_assertions.push(assertion);
                        }
                        Some(SemanticAssertion::NoEvent(assertion)) => {
                            no_events.push(assertion);
                        }
                        Some(SemanticAssertion::Unique(assertion)) => {
                            uniques.push(assertion);
                        }

                        None => {}
                    };
                }
            }
        }

        Self {
            declarations_assertions,
            read_assertions,
            at_scope_assertions,
            scope_start_assertions,
            scope_end_assertions,
            no_events,
            uniques,
        }
    }

    fn check(
        &self,
        code: &str,
        test_name: &str,
        events_by_pos: HashMap<TextSize, Vec<SemanticEvent>>,
    ) {
        // Check every declaration assertion is ok

        for (_, assertion) in self.declarations_assertions.iter() {
            if let Some(events) = events_by_pos.get(&assertion.range.start()) {
                match &events[0] {
                    SemanticEvent::DeclarationFound { .. } => {
                        // OK because we are attached to a declaration
                    }
                    _ => {
                        println!("Assertion: {:?}", assertion);
                        println!("Events: {:#?}", events_by_pos);
                        error_assertion_not_attached_to_a_declaration(
                            code,
                            assertion.range,
                            test_name,
                        )
                    }
                }
            } else {
                println!("Assertion: {:?}", assertion);
                println!("Events: {:#?}", events_by_pos);
                error_assertion_not_attached_to_a_declaration(code, assertion.range, test_name);
            }
        }

        // Check every read assertion is ok

        for assertion in self.read_assertions.iter() {
            let decl = match self
                .declarations_assertions
                .get(&assertion.declaration_asertion_name)
            {
                Some(decl) => decl,
                None => {
                    panic!(
                        "No declaration found with name: {}",
                        assertion.declaration_asertion_name
                    );
                }
            };

            let events = match events_by_pos.get(&assertion.range.start()) {
                Some(events) => events,
                None => {
                    println!("Assertion: {:?}", assertion);
                    println!("Events: {:#?}", events_by_pos);
                    panic!("No read event found at this range");
                }
            };

            let at_least_one_match = events.iter().any(|e| {
                let declaration_at_range = match &e {
                    SemanticEvent::Read { declaration_at, .. } => declaration_at.clone(),
                    SemanticEvent::HoistedRead { declaration_at, .. } => Some(*declaration_at),
                    _ => None,
                };

                if let Some(declaration_at_range) = declaration_at_range {
                    code[declaration_at_range] == code[decl.range]
                } else {
                    false
                }
            });

            if !at_least_one_match {
                println!("Assertion: {:?}", assertion);
                println!("Events: {:#?}", events_by_pos);
                panic!("No matching read event found at this range");
            }
        }

        // Check every at scope assertion is ok

        for assertion in self.at_scope_assertions.iter() {
            if let Some(events) = events_by_pos.get(&assertion.range.start()) {
                // Needs to be a unique event for now
                match &events[0] {
                    SemanticEvent::DeclarationFound {
                        scope_started_at, ..
                    } => match self.scope_start_assertions.get(&assertion.scope_name) {
                        Some(scope_start_assertion) => {
                            if scope_start_assertion.range.start() != *scope_started_at {
                                error_declaration_pointing_to_unknown_scope(
                                    code,
                                    assertion.range,
                                    test_name,
                                );
                            }
                            assert_eq!(scope_start_assertion.range.start(), *scope_started_at);
                        }
                        None => error_declaration_pointing_to_unknown_scope(
                            code,
                            assertion.range,
                            test_name,
                        ),
                    },
                    _ => {
                        error_assertion_not_attached_to_a_declaration(
                            code,
                            assertion.range,
                            test_name,
                        );
                    }
                }
            }
        }

        // Check every scope start assertion is ok

        for assertion in self.scope_start_assertions.values() {
            if let Some(events) = events_by_pos.get(&assertion.range.start()) {
                let is_at_least_one_scope_start = events
                    .iter()
                    .any(|e| matches!(e, SemanticEvent::ScopeStarted { .. }));

                if !is_at_least_one_scope_start {
                    panic!("error_scope_assertion_not_attached_to_a_scope_event");
                }
            } else {
                panic!("error_scope_assertion_not_attached_to_a_scope_event");
            }
        }

        // Check every scope end assertion is ok

        for scope_end_assertion in self.scope_end_assertions.iter() {
            // Check we have a scope start with the same label.
            let scope_start_assertions_range = match self
                .scope_start_assertions
                .get(&scope_end_assertion.scope_name)
            {
                Some(scope_start_assertions) => scope_start_assertions.range,
                None => {
                    error_scope_end_assertion_points_to_non_existing_scope_start_assertion(
                        code,
                        &scope_end_assertion.range,
                        test_name,
                    );
                    continue;
                }
            };

            if let Some(events) = events_by_pos.get(&scope_end_assertion.range.end()) {
                // At least one of the events should be a scope start starting
                // where we expect
                let e = events.iter().find(|event| match event {
                    SemanticEvent::ScopeEnded { started_at, .. } => {
                        println!(
                            "started_at: {:?} scope_start_assertions_range: {:?}",
                            started_at, scope_start_assertions_range
                        );
                        *started_at == scope_start_assertions_range.start()
                    }
                    _ => false,
                });

                if e.is_none() {
                    error_scope_end_assertion_points_to_the_wrong_scope_start(
                        code,
                        &scope_end_assertion.range,
                        &scope_start_assertions_range,
                        test_name,
                    );
                }
            } else {
                panic!("error_scope_assertion_not_attached_to_a_scope_event");
            }
        }

        // Check every no event assertion

        for assertion in self.no_events.iter() {
            match events_by_pos.get(&assertion.range.start()) {
                Some(_) => panic!("unexpected event at this position"),
                None => {
                    // Ok
                }
            }

            match events_by_pos.get(&assertion.range.end()) {
                Some(_) => panic!("unexpected event at this position"),
                None => {
                    // Ok
                }
            }
        }

        // Check every unique  assertion

        for unique in self.uniques.iter() {
            match events_by_pos.get(&unique.range.start()) {
                Some(v) => {
                    if v.len() > 1 {
                        panic!("unexpected more than one event");
                    } else if v.is_empty() {
                        panic!("unexpected no events");
                    }
                }
                None => {
                    // Ok
                }
            }

            match events_by_pos.get(&unique.range.end()) {
                Some(v) => {
                    if v.len() > 1 {
                        panic!("unexpected more than one event");
                    } else if v.is_empty() {
                        panic!("unexpected no events");
                    }
                }
                None => {
                    // Ok
                }
            }
        }
    }
}

fn error_assertion_not_attached_to_a_declaration(
    code: &str,
    assertion_range: TextRange,
    test_name: &str,
) {
    let files = SimpleFile::new(test_name.to_string(), code.into());
    let d = Diagnostic::error(
        0,
        "",
        "This assertion must be attached to a SemanticEvent::DeclarationFound.",
    )
    .primary(assertion_range, "");

    let mut console = EnvConsole::new(false);
    console.log(markup! {
        {d.display(&files)}
    });
    panic!("This assertion must be attached to a SemanticEvent::DeclarationFound.");
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

fn error_assertion_name_clash(
    token: &JsSyntaxToken,
    code: &str,
    test_name: &str,
    old_range: TextRange,
) {
    // If there is already an assertion with the same name. Suggest a rename

    let files = SimpleFile::new(test_name.to_string(), code.into());
    let d = Diagnostic::error(0, "", "Assertion label conflict.")
        .primary(
            token.text_range(),
            "There is already a assertion with the same name. Consider renaming this one.",
        )
        .secondary(old_range, "Previous assertion");

    let mut console = EnvConsole::new(false);
    console.log(markup! {
        {d.display(&files)}
    });

    panic!("Assertion label conflict");
}

fn error_scope_end_assertion_points_to_non_existing_scope_start_assertion(
    code: &str,
    range: &TextRange,
    file_name: &str,
) {
    let files = SimpleFile::new(file_name.to_string(), code.into());
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

fn error_scope_end_assertion_points_to_the_wrong_scope_start(
    code: &str,
    range: &TextRange,
    same_name_range: &TextRange,
    file_name: &str,
) {
    let files = SimpleFile::new(file_name.to_string(), code.into());
    let d = Diagnostic::error(0, "", "Wrong scope start")
        .primary(
            range,
            "This scope end assertion points to the wrong scope start.",
        )
        .secondary(same_name_range, "This assertion has the same label");

    let mut console = EnvConsole::new(false);
    console.log(markup! {
        {d.display(&files)}
    });
    panic!("Wrong scope start");
}
