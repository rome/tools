use rome_js_syntax::{
    JsAnyBinding, JsAnyBindingPattern, JsComputedMemberExpression, JsIdentifierBinding, JsLanguage,
    JsLiteralMemberName, JsReferenceIdentifier, JsSyntaxKind, JsSyntaxNode, JsSyntaxToken,
    JsVariableDeclaration, JsVariableDeclarator, JsVariableDeclaratorList, TextRange, TextSize,
    TsGlobalDeclaration, TsThisParameter, WalkEvent,
};
use rome_rowan::{syntax::Preorder, AstNode, OptionAstNodeParent, SyntaxNodeCast};
use std::collections::{HashMap, VecDeque};

#[derive(Debug)]
pub enum ScopeResolutionEvent {
    DeclarationFound {
        name: String,
        range: TextRange,
        hoists: bool,
    },
    ReferenceFound {
        name: String,
        range: TextRange,
        declared_at: Option<TextRange>,
    },
    ReferenceMatched {
        range: TextRange,
        declared_at: TextRange,
    },
    ScopeStarted {
        start_at: TextSize,
        hoist: bool,
    },
    ScopeEnded {
        started_at: TextSize,
        end_at: TextSize,
    },
}

impl ScopeResolutionEvent {
    pub fn name(&self) -> &str {
        match self {
            ScopeResolutionEvent::DeclarationFound { name, .. } => name,
            ScopeResolutionEvent::ReferenceFound { name, .. } => name,
            _ => todo!(),
        }
    }

    pub fn range(&self) -> TextRange {
        match self {
            ScopeResolutionEvent::DeclarationFound { range, .. } => *range,
            ScopeResolutionEvent::ReferenceFound { range, .. } => *range,
            ScopeResolutionEvent::ReferenceMatched { range, .. } => *range,
            _ => todo!(),
        }
    }
}

pub struct SymbolIterator {
    iter: Preorder<JsLanguage>,
    current_scope: HashMap<String, TextRange>,
    hoisting_scope: Vec<HashMap<String, Vec<TextRange>>>,
    hoisting_scope_idx: Vec<usize>,
    items_entered_into_scope: Vec<Vec<String>>,
    items_shadowed: Vec<Vec<(String, TextRange)>>,
    scope_start: Vec<TextSize>,
    stash: VecDeque<ScopeResolutionEvent>,
}

fn is_identifier_declared_with_var(binding: JsSyntaxNode) -> bool {
    binding
        .cast::<JsIdentifierBinding>()
        .parent::<JsVariableDeclarator>()
        .parent::<JsVariableDeclaratorList>()
        .parent::<JsVariableDeclaration>()
        .map(|declaration| declaration.is_var())
        .unwrap_or(false)
}

fn extract_symbol(node: JsSyntaxNode) -> Option<ScopeResolutionEvent> {
    match node.kind() {
        JsSyntaxKind::JS_IDENTIFIER_BINDING => Some(ScopeResolutionEvent::DeclarationFound {
            name: node.text_trimmed().to_string(),
            range: node.text_range(),
            hoists: is_identifier_declared_with_var(node),
        }),
        JsSyntaxKind::TS_IDENTIFIER_BINDING | JsSyntaxKind::JS_LITERAL_EXPORT_NAME => {
            Some(ScopeResolutionEvent::DeclarationFound {
                name: node.text_trimmed().to_string(),
                range: node.text_range(),
                hoists: false,
            })
        }
        JsSyntaxKind::JS_IDENTIFIER_ASSIGNMENT
        | JsSyntaxKind::JS_SUPER_EXPRESSION
        | JsSyntaxKind::JS_THIS_EXPRESSION
        | JsSyntaxKind::JS_MODULE_SOURCE => Some(ScopeResolutionEvent::ReferenceFound {
            name: node.text_trimmed().to_string(),
            range: node.text_range(),
            declared_at: None,
        }),
        // Some reference identifiers are not really references
        // - const on typescript const cast "10 as const"
        // - undefined
        JsSyntaxKind::JS_REFERENCE_IDENTIFIER => {
            let value_token = unsafe { JsReferenceIdentifier::new_unchecked(node) }
                .as_fields()
                .value_token
                .ok()?;

            match value_token.text_trimmed() {
                "const" | "undefined" => None,
                text_trimmed => Some(ScopeResolutionEvent::ReferenceFound {
                    name: text_trimmed.to_string(),
                    range: value_token.text_range(),
                    declared_at: None,
                }),
            }
        }
        // JS_LITERAL_MEMBER_NAME to be a symbol:
        // - it cannot be a constructor
        // - it cannot be a string literal
        JsSyntaxKind::JS_LITERAL_MEMBER_NAME => {
            let is_inside_constructor = matches!(
                node.parent()?.kind(),
                JsSyntaxKind::JS_CONSTRUCTOR_CLASS_MEMBER
                    | JsSyntaxKind::TS_CONSTRUCTOR_SIGNATURE_CLASS_MEMBER
            );

            let value = unsafe { JsLiteralMemberName::new_unchecked(node) }
                .as_fields()
                .value
                .ok()?;
            let is_string_literal = matches!(value.kind(), JsSyntaxKind::JS_STRING_LITERAL);

            (!is_inside_constructor && !is_string_literal).then(|| {
                ScopeResolutionEvent::DeclarationFound {
                    name: value.text_trimmed().to_string(),
                    range: value.text_range(),
                    hoists: false,
                }
            })
        }
        //
        // is JS_NAME under TS_NAMED_TUPLE_TYPE_ELEMENT a symbol?
        // example: type A = [ b: string ]; // <-- is b a symbol?
        JsSyntaxKind::JS_NAME => {
            let parent_kind = node.parent()?.kind();
            let parent_ok = matches!(
                parent_kind,
                JsSyntaxKind::JS_STATIC_MEMBER_ASSIGNMENT
                    | JsSyntaxKind::JS_STATIC_MEMBER_EXPRESSION
                    | JsSyntaxKind::TS_EXPORT_AS_NAMESPACE_CLAUSE
                    | JsSyntaxKind::TS_QUALIFIED_MODULE_NAME
                    | JsSyntaxKind::TS_QUALIFIED_NAME
            );
            parent_ok.then(|| ScopeResolutionEvent::ReferenceFound {
                name: node.text_trimmed().to_string(),
                range: node.text_range(),
                declared_at: None,
            })
        }
        JsSyntaxKind::TS_THIS_PARAMETER => {
            let this_token = unsafe { TsThisParameter::new_unchecked(node) }
                .as_fields()
                .this_token
                .ok()?;

            Some(ScopeResolutionEvent::DeclarationFound {
                name: this_token.text_trimmed().to_string(),
                range: this_token.text_range(),
                hoists: false,
            })
        }
        JsSyntaxKind::TS_GLOBAL_DECLARATION => {
            let global_token = unsafe { TsGlobalDeclaration::new_unchecked(node) }
                .as_fields()
                .global_token
                .ok()?;

            Some(ScopeResolutionEvent::DeclarationFound {
                name: global_token.text_trimmed().to_string(),
                range: global_token.text_range(),
                hoists: false,
            })
        }
        JsSyntaxKind::JS_COMPUTED_MEMBER_EXPRESSION => {
            let value_token = unsafe { JsComputedMemberExpression::new_unchecked(node) }
                .as_fields()
                .member
                .ok()?
                .as_js_any_literal_expression()?
                .as_js_string_literal_expression()?
                .as_fields()
                .value_token
                .ok()?;

            Some(ScopeResolutionEvent::ReferenceFound {
                name: value_token.text_trimmed().to_string(),
                range: value_token.text_range(),
                declared_at: None,
            })
        }
        JsSyntaxKind::TS_TYPE_PARAMETER_NAME => {
            let parent = node.parent()?;
            let great_parent = parent.parent()?;

            let is_in_type_parameter_list =
                matches!(great_parent.kind(), JsSyntaxKind::TS_TYPE_PARAMETER_LIST);
            let is_in_mapped_type = matches!(parent.kind(), JsSyntaxKind::TS_MAPPED_TYPE);

            (is_in_type_parameter_list || is_in_mapped_type).then(|| {
                ScopeResolutionEvent::DeclarationFound {
                    name: node.text_trimmed().to_string(),
                    range: node.text_range(),
                    hoists: false,
                }
            })
        }
        _ => None,
    }
}

impl SymbolIterator {
    fn push_new_scope(&mut self, start_at: TextSize, also_hoist_scope: bool) {
        self.stash.push_back(ScopeResolutionEvent::ScopeStarted {
            start_at,
            hoist: true,
        });

        self.items_entered_into_scope.push(vec![]);
        self.items_shadowed.push(vec![]);
        self.scope_start.push(start_at);

        if also_hoist_scope {
            self.hoisting_scope.push(HashMap::new());
            self.hoisting_scope_idx
                .push(self.items_entered_into_scope.len() - 1);
        }
    }

    fn pop_scope(&mut self, end_at: TextSize, also_hoist_scope: bool) {
        if let Some(symbols) = self.items_entered_into_scope.pop() {
            for symbol in symbols {
                self.current_scope.remove(&symbol);
            }
        }

        if let Some(symbols) = self.items_shadowed.pop() {
            for (symbol, range) in symbols {
                self.current_scope.insert(symbol, range);
            }
        }

        if also_hoist_scope {
            self.hoisting_scope_idx.pop();
            if let (Some(old), Some(current)) =
                (self.hoisting_scope.pop(), self.hoisting_scope.last_mut())
            {
                for (key, mut v) in old {
                    let pending_references = current.entry(key).or_default();
                    pending_references.append(&mut v);
                }
            }
        }

        if let Some(started_at) = self.scope_start.pop() {
            self.stash
                .push_back(ScopeResolutionEvent::ScopeEnded { end_at, started_at });
        }
    }

    fn push_symbol_to_scope(&mut self, name: &str, range: &TextRange) {
        let shadowed_value = self.current_scope.insert(name.to_string(), *range);

        if let Some(items_entered_into_scope) = self.items_entered_into_scope.last_mut() {
            items_entered_into_scope.push(name.to_string());
        }

        if let Some((shadowed_value, items_shadowed)) =
            shadowed_value.zip(self.items_shadowed.last_mut())
        {
            items_shadowed.push((name.to_string(), shadowed_value));
        }
    }

    fn push_hoisted_symbol_to_scope(&mut self, name: &str, range: &TextRange) {
        let scope_idx = self.hoisting_scope_idx.last_mut().unwrap();

        if self.current_scope.get(name).is_some() {
            //TODO signal error name already declared
        }

        self.current_scope.insert(name.to_string(), *range);
        self.items_entered_into_scope[*scope_idx].push(name.to_string());
    }

    fn solve_pending(&mut self, token: &JsSyntaxToken) {
        if let Some(hoisting_scope) = self.hoisting_scope.last_mut() {
            let txt = token.text_trimmed();
            if let Some(being_solved) = hoisting_scope.remove(txt) {
                for range in being_solved {
                    self.stash
                        .push_back(ScopeResolutionEvent::ReferenceMatched {
                            range,
                            declared_at: token.text_range(),
                        });
                }
            }
        }
    }

    fn solve_pending_with_binding(&mut self, binding: &JsAnyBinding) {
        match binding {
            JsAnyBinding::JsIdentifierBinding(ident) => {
                let name = ident.name_token().unwrap();
                self.solve_pending(&name);
            }
            JsAnyBinding::JsUnknownBinding(_) => {}
        }
    }

    fn enter_node(&mut self, node: &JsSyntaxNode) {
        match node.kind() {
            JsSyntaxKind::JS_BLOCK_STATEMENT
            | JsSyntaxKind::JS_FUNCTION_BODY
            | JsSyntaxKind::JS_FOR_OF_STATEMENT
            | JsSyntaxKind::JS_FOR_IN_STATEMENT
            | JsSyntaxKind::JS_ARROW_FUNCTION_EXPRESSION
            | JsSyntaxKind::JS_CONSTRUCTOR_CLASS_MEMBER
            | JsSyntaxKind::JS_GETTER_CLASS_MEMBER
            | JsSyntaxKind::JS_SETTER_CLASS_MEMBER
            | JsSyntaxKind::JS_CATCH_CLAUSE
            | JsSyntaxKind::JS_FINALLY_CLAUSE => {
                self.push_new_scope(node.text_range().start(), false)
            }
            JsSyntaxKind::JS_FUNCTION_DECLARATION => {
                // let declaration =
                //     unsafe { rome_js_syntax::JsFunctionDeclaration::new_unchecked(node.clone()) };

                // if let Ok(id) = declaration.id() {
                //     self.solve_pending_with_binding(&id);
                // }

                // self.push_new_scope(node.text_range().clone(), true);
            }
            JsSyntaxKind::JS_VARIABLE_DECLARATION => {
                let declaration =
                    unsafe { rome_js_syntax::JsVariableDeclaration::new_unchecked(node.clone()) };
                if declaration.is_var() {
                    for decl in declaration.declarators().into_iter().flatten() {
                        if let Ok(JsAnyBindingPattern::JsAnyBinding(binding)) = decl.id() {
                            self.solve_pending_with_binding(&binding);
                        }
                    }
                }
            }
            _ => {}
        }
    }

    fn leave_node(&mut self, node: &JsSyntaxNode) {
        match node.kind() {
            // JsSyntaxKind::JS_FUNCTION_DECLARATION => {
            //     self.pop_scope(node.text_range(), true);
            // }
            JsSyntaxKind::JS_BLOCK_STATEMENT
            | JsSyntaxKind::JS_FUNCTION_BODY
            | JsSyntaxKind::JS_FOR_OF_STATEMENT
            | JsSyntaxKind::JS_FOR_IN_STATEMENT
            | JsSyntaxKind::JS_ARROW_FUNCTION_EXPRESSION
            | JsSyntaxKind::JS_CONSTRUCTOR_CLASS_MEMBER
            | JsSyntaxKind::JS_GETTER_CLASS_MEMBER
            | JsSyntaxKind::JS_SETTER_CLASS_MEMBER
            | JsSyntaxKind::JS_CATCH_CLAUSE
            | JsSyntaxKind::JS_FINALLY_CLAUSE => {
                let end_at = node.last_token().unwrap().text_range().start();
                self.pop_scope(end_at, false);
            }
            _ => {}
        }
    }
}

impl Iterator for SymbolIterator {
    type Item = ScopeResolutionEvent;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(from_stash) = self.stash.pop_front() {
                return Some(from_stash);
            }

            if let Some(e) = self.iter.next() {
                match e {
                    WalkEvent::Enter(node) => {
                        self.enter_node(&node);

                        if let Some(mut s) = extract_symbol(node) {
                            match &mut s {
                                ScopeResolutionEvent::DeclarationFound {
                                    name,
                                    range,
                                    hoists,
                                } => {
                                    if *hoists {
                                        self.push_hoisted_symbol_to_scope(name, range);
                                    } else {
                                        self.push_symbol_to_scope(name, range);
                                    }
                                }
                                ScopeResolutionEvent::ReferenceFound {
                                    name,
                                    range,
                                    declared_at,
                                } => match self.current_scope.get(name) {
                                    Some(target) => *declared_at = Some(*target),
                                    None => {
                                        if let Some(hoisting_scope) = self.hoisting_scope.last_mut()
                                        {
                                            let pending =
                                                hoisting_scope.entry(name.clone()).or_default();
                                            pending.push(*range);
                                        }
                                    }
                                },
                                _ => {}
                            }

                            self.stash.push_back(s)
                        }
                    }
                    WalkEvent::Leave(node) => {
                        self.leave_node(&node);
                    }
                }
            } else {
                if !self.items_entered_into_scope.is_empty() {
                    self.pop_scope(0.into(), true);
                } else {
                    break;
                }
            }
        }

        None
    }
}

pub fn symbols(root: JsSyntaxNode) -> SymbolIterator {
    let mut i = SymbolIterator {
        iter: root.preorder(),
        current_scope: HashMap::new(),
        hoisting_scope: vec![],
        hoisting_scope_idx: vec![],
        items_entered_into_scope: vec![],
        items_shadowed: vec![],
        scope_start: vec![],
        stash: VecDeque::new(),
    };

    i.push_new_scope(0.into(), true); // global scope
    i
}

#[cfg(test)]
mod tests {
    use std::collections::{BTreeMap, HashMap};

    use super::*;
    use rome_console::{markup, ConsoleExt, EnvConsole};
    use rome_diagnostics::{file::SimpleFile, Applicability, Diagnostic, Severity};
    use rome_js_syntax::TextRange;
    use rome_rowan::NodeOrToken;
    use suggest::Suggest;

    #[derive(Eq, PartialEq, Debug)]
    pub struct TextRangeByStart(TextRange);

    impl PartialOrd for TextRangeByStart {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            self.0.start().partial_cmp(&other.0.start())
        }
    }

    impl Ord for TextRangeByStart {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.0.start().cmp(&other.0.start())
        }
    }

    fn assert_reference_points_to_nothing(
        file_name: &str,
        code: &str,
        range: &TextRange,
        declared_at: &Option<TextRange>,
    ) {
        if declared_at.is_some() {
            let files = SimpleFile::new(file_name.to_string(), code.into());
            let d = Diagnostic::error(0, "", "Unreferenced symbol is pointing to a declaration")
                .primary(
                    range,
                    "This reference should not point to a declaration ...",
                );

            let d = if let Some(declared_at) = declared_at {
                d.label(
                    Severity::Error,
                    declared_at,
                    "... but is pointing to this declaration.",
                )
            } else {
                d
            };

            let mut console = EnvConsole::default();
            console.log(markup! {
                {d.display(&files)}
            });
        }
        assert!(declared_at.is_none());
    }

    fn asserts_references(file_name: &str, _line: u32, code: &str) {
        let r = crate::parse(code, 0, crate::SourceType::js_script());

        // Extract symbols and index by range

        let mut event_by_range = HashMap::new();
        for symbol in symbols(r.syntax()) {
            match &symbol {
                ScopeResolutionEvent::DeclarationFound { range, .. } => {
                    event_by_range.insert(*range, symbol);
                }
                ScopeResolutionEvent::ReferenceFound { range, .. } => {
                    event_by_range.insert(*range, symbol);
                }
                ScopeResolutionEvent::ReferenceMatched { range, declared_at } => {
                    if let Some(ScopeResolutionEvent::ReferenceFound {
                        declared_at: ref_declared_at,
                        ..
                    }) = event_by_range.get_mut(range)
                    {
                        *ref_declared_at = Some(*declared_at);
                    }
                }
                ScopeResolutionEvent::ScopeStarted { start_at, hoist } => {
                    let range = TextRange::new(start_at.clone(), start_at.clone());
                    event_by_range.insert(range, symbol);
                }
                ScopeResolutionEvent::ScopeEnded { end_at, .. } => {
                    let range = TextRange::new(end_at.clone(), end_at.clone());
                    event_by_range.insert(range, symbol);
                }
            }
        }

        // Extract assertions inside comments

        let mut reference_assertions = BTreeMap::new();
        let mut declarations_assertions = BTreeMap::new();
        let mut scope_start_assertions = BTreeMap::new();
        let mut scope_end_assertions = BTreeMap::new();
        for node in r.syntax().preorder_with_tokens() {
            if let WalkEvent::Enter(NodeOrToken::Token(token)) = node {
                let trivia = token.trailing_trivia();
                let text = trivia.text();

                if text.starts_with("/*START") {
                    let old = scope_start_assertions.insert(
                        text.trim()
                            .trim_start_matches("/*START")
                            .trim_end_matches("*/")
                            .trim()
                            .to_string(),
                        (token.text_range().start(), trivia.text_range()),
                    );

                    // If there is already an assertion with the same name. Suggest a rename
                    if let Some((_, old)) = old {
                        let files = SimpleFile::new(file_name.to_string(), code.into());
                        let d = Diagnostic::error(
                            0,
                            "",
                            "Assertion label conflict.",
                        )
                        .primary(token.text_range(), "There is already a assertion with the same name. Consider renaming this one.")
                        .secondary(
                            old,
                            "Previous assertion",
                        );

                        let mut console = EnvConsole::default();
                        console.log(markup! {
                            {d.display(&files)}
                        });

                        panic!("Assertion label conflict")
                    }
                } else if text.starts_with("/*END") {
                    scope_end_assertions.insert(
                        token.text_range().start(),
                        (
                            text.trim()
                                .trim_start_matches("/*END")
                                .trim_end_matches("*/")
                                .trim()
                                .to_string(),
                            trivia.text_range(),
                        ),
                    );
                } else if text.contains('@') {
                    reference_assertions.insert(
                        TextRangeByStart(token.text_range()),
                        text.trim()
                            .trim_start_matches("/*@")
                            .trim_end_matches("*/")
                            .trim()
                            .to_string(),
                    );
                } else if text.contains('#') {
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
                        let files = SimpleFile::new(file_name.to_string(), code.into());
                        let d = Diagnostic::error(
                            0,
                            "",
                            "Assertion label conflict.",
                        )
                        .primary(token.text_range(), "There is already a assertion with the same name. Consider renaming this one.")
                        .secondary(
                            old,
                            "Previous assertion",
                        );

                        let mut console = EnvConsole::default();
                        console.log(markup! {
                            {d.display(&files)}
                        });

                        panic!("Assertion label conflict")
                    }
                }
            }
        }

        // Check every reference assertion is ok

        for (assertion_range, assertion_label) in reference_assertions {
            // Check if the assertion is attached to a symbol
            if let Some(symbol) = &event_by_range.get(&assertion_range.0) {
                match symbol {
                    // ... if it is attached to a declaration symbol, show an error
                    ScopeResolutionEvent::DeclarationFound { range, .. } => {
                        error_reference_assertion_attached_to_declaration(
                            code,
                            &assertion_range,
                            file_name,
                            range,
                        );
                    }
                    ScopeResolutionEvent::ReferenceFound {
                        range, declared_at, ..
                    } => {
                        // ... if it is attached to a reference symbol, we have fours possibilities:
                        // 1 - is labeled '?' and it must points to nothing
                        // 2 - this reference assertion points to an non existing declaration assertion;
                        // 3 - this reference assertion points to the wrong declaration assertion;
                        // 4 - everything is fine.
                        if assertion_label == "?" {
                            // case 1
                            assert_reference_points_to_nothing(file_name, code, range, declared_at);
                        } else {
                            let expected_declaration =
                                declarations_assertions.get(&assertion_label);
                            if let Some(expected_declaration_range) = expected_declaration {
                                // case 3 and 4
                                assert_reference_points_to_correct_declaration(
                                    declared_at,
                                    expected_declaration_range,
                                    file_name,
                                    code,
                                    range,
                                );
                            } else {
                                // case 2
                                error_assertion_points_to_non_existing_declaration(
                                    file_name,
                                    code,
                                    range,
                                    &declarations_assertions,
                                    assertion_label,
                                );
                            }
                        }
                    }
                    _ => todo!(),
                }
            } else {
                error_reference_attached_to_non_symbol(code, assertion_range, file_name);
            }
        }

        // Check every declaration assertion is ok

        for (_, assertion_range) in declarations_assertions {
            if let Some(symbol) = event_by_range.get(&assertion_range) {
                match symbol {
                    ScopeResolutionEvent::DeclarationFound { .. } => {
                        // No need to check anything on declarations
                    }
                    ScopeResolutionEvent::ReferenceFound { range, .. } => {
                        error_declaration_assertion_attached_to_reference(
                            code,
                            assertion_range,
                            file_name,
                            range,
                        );
                    }
                    _ => todo!(),
                }
            } else {
                error_declaration_attached_to_non_symbol(code, assertion_range, file_name);
            }
        }

        // Check every scope end assertion is ok
        for (expected_scope_end, (assertion_label, scope_end_assertion_range)) in
            scope_end_assertions
        {
            let scope_end_start_range =
                TextRange::new(expected_scope_end.clone(), expected_scope_end.clone());
            if let Some(e) = event_by_range.get(&scope_end_start_range) {
                match e {
                    ScopeResolutionEvent::ScopeEnded {
                        started_at: actual_started_at,
                        ..
                    } => {
                        if let Some((expected_start_at, scope_start_assertion_range)) =
                            scope_start_assertions.get(&assertion_label)
                        {
                            assert_scope_end_points_to_correct_scope_start(
                                expected_start_at,
                                &expected_scope_end,
                                actual_started_at,
                                file_name,
                                code,
                                &TextRange::new(
                                    actual_started_at.clone(),
                                    actual_started_at.clone(),
                                ),
                                &scope_start_assertion_range,
                                &scope_end_assertion_range,
                            );

                            if expected_start_at == actual_started_at {
                                // OK
                            } else {
                                todo!()
                            }
                        } else {
                            error_scope_end_assertion_points_to_non_existing_scope_start_assertion(
                                file_name,
                                code,
                                &scope_end_assertion_range,
                                &scope_start_assertions,
                                assertion_label,
                            )
                        }
                    }
                    _ => error_no_scope_ends_here(code, scope_end_start_range, file_name),
                }
            } else {
                error_no_scope_ends_here(code, scope_end_start_range, file_name);
            }
        }

        // Check every scope start assertion is ok

        for (_, (scope_start, _)) in scope_start_assertions {
            let range = TextRange::new(scope_start.clone(), scope_start.clone());
            if let Some(e) = event_by_range.get(&range) {
                match e {
                    ScopeResolutionEvent::ScopeStarted { .. } => {
                        // OK
                    }
                    _ => error_no_scope_starts_here(code, range, file_name),
                }
            } else {
                error_no_scope_starts_here(code, range, file_name);
            }
        }
    }

    fn assert_scope_end_points_to_correct_scope_start(
        expected_start_at: &TextSize,
        expected_scope_end: &TextSize,
        actual_started_at: &TextSize,

        file_name: &str,
        code: &str,
        actual_scope_start_range: &TextRange,
        scope_start_assertion_range: &TextRange,
        scope_end_assertion_range: &TextRange,
    ) {
        if *expected_start_at != *actual_started_at {
            let files = SimpleFile::new(file_name.to_string(), code.into());
            let d = Diagnostic::error(0, "", "Scope end assertion pointing to wrong scope start.")
                .primary(scope_end_assertion_range, "This scope end...");

            let d = d.label(
                Severity::Error,
                actual_scope_start_range,
                "... is ending this scope start ...",
            );

            let d = d.secondary(
                scope_start_assertion_range,
                "... but this was the expected scope start.",
            );

            let mut console = EnvConsole::default();
            console.log(markup! {
                {d.display(&files)}
            });
        }
        assert_eq!(*expected_start_at, *actual_started_at);
    }

    fn error_scope_end_assertion_points_to_non_existing_scope_start_assertion(
        file_name: &str,
        code: &str,
        range: &TextRange,
        valid_assertions: &BTreeMap<String, (TextSize, TextRange)>,
        assertion_label: String,
    ) {
        let files = SimpleFile::new(file_name.to_string(), code.into());
        let d = Diagnostic::error(0, "", "Scope start assertion not found.").primary(
            range,
            "This scope end assertion points to a non-existing scope start assertion.",
        );

        let labels: Vec<_> = valid_assertions.keys().collect();
        let d = if let Some(suggestion) = labels.suggest(&assertion_label) {
            let suggestion = format!("Did you mean \"{suggestion}\"?");
            d.suggestion_no_code(range, &suggestion, Applicability::Unspecified)
        } else {
            d
        };
        let mut console = EnvConsole::default();
        console.log(markup! {
            {d.display(&files)}
        });
        panic!("Scope start assertion not found.");
    }

    fn error_no_scope_ends_here(code: &str, assertion_range: TextRange, file_name: &str) {
        let files = SimpleFile::new(file_name.to_string(), code.into());
        let d = Diagnostic::error(
            0,
            "",
            "Scope end assertions must be attached to scope ends.",
        )
        .primary(assertion_range, "A scope does not end here");

        let mut console = EnvConsole::default();
        console.log(markup! {
            {d.display(&files)}
        });
    }

    fn error_no_scope_starts_here(code: &str, assertion_range: TextRange, file_name: &str) {
        let files = SimpleFile::new(file_name.to_string(), code.into());
        let d = Diagnostic::error(
            0,
            "",
            "Scope start assertions must be attached to scope starts.",
        )
        .primary(assertion_range, "A scope does not start here");

        let mut console = EnvConsole::default();
        console.log(markup! {
            {d.display(&files)}
        });
    }

    fn error_declaration_attached_to_non_symbol(
        code: &str,
        assertion_range: TextRange,
        file_name: &str,
    ) {
        let mut fix = code[assertion_range]
            .split("/*#")
            .next()
            .unwrap()
            .to_string();
        fix.push(' ');
        let files = SimpleFile::new(file_name.to_string(), code.into());
        let d = Diagnostic::error(
            0,
            "",
            "Declaration assertions must be attached to symbols declarations.",
        )
        .suggestion(
            assertion_range,
            "Remove the assertion",
            fix,
            Applicability::Always,
        );
        let mut console = EnvConsole::default();
        console.log(markup! {
            {d.display(&files)}
        });
    }

    fn error_declaration_assertion_attached_to_reference(
        code: &str,
        assertion_range: TextRange,
        file_name: &str,
        range: &TextRange,
    ) {
        let fix = code[assertion_range].replace("/*#", "/*@");
        let files = SimpleFile::new(file_name.to_string(), code.into());
        let d = Diagnostic::error(0, "", "Declaration assertion attached to symbol reference.")
            .suggestion(range, "Change '/*#' to '/*@'", fix, Applicability::Always);
        let mut console = EnvConsole::default();
        console.log(markup! {
            {d.display(&files)}
        });
    }

    fn error_reference_attached_to_non_symbol(
        code: &str,
        assertion_range: TextRangeByStart,
        file_name: &str,
    ) {
        let mut fix = code[assertion_range.0]
            .split("/*@")
            .next()
            .unwrap()
            .to_string();
        fix.push(' ');
        let files = SimpleFile::new(file_name.to_string(), code.into());
        let d = Diagnostic::error(
            0,
            "",
            "Reference assertions must be attached to symbols references.",
        )
        .suggestion(
            assertion_range.0,
            "Remove the assertion",
            fix,
            Applicability::Always,
        );
        let mut console = EnvConsole::default();
        console.log(markup! {
            {d.display(&files)}
        });
    }

    fn error_reference_assertion_attached_to_declaration(
        code: &str,
        assertion_range: &TextRangeByStart,
        file_name: &str,
        range: &TextRange,
    ) {
        let fix = code[assertion_range.0].replace("/*@", "/*#");
        let files = SimpleFile::new(file_name.to_string(), code.into());
        let d = Diagnostic::error(0, "", "Reference assertion attached to symbol declaration.")
            .suggestion(range, "Change '/*@' to '/*#'", fix, Applicability::Always);
        let mut console = EnvConsole::default();
        console.log(markup! {
            {d.display(&files)}
        });
    }

    fn error_assertion_points_to_non_existing_declaration(
        file_name: &str,
        code: &str,
        range: &TextRange,
        declarations_assertions: &BTreeMap<String, TextRange>,
        assertion_label: String,
    ) {
        let files = SimpleFile::new(file_name.to_string(), code.into());
        let d = Diagnostic::error(0, "", "Wrong reference.").primary(
            range,
            "This reference assertion points to a non-existing declaration assertion.",
        );

        let labels: Vec<_> = declarations_assertions.keys().collect();
        let d = if let Some(suggestion) = labels.suggest(&assertion_label) {
            let suggestion = format!("Did you mean \"{suggestion}\"?");
            d.suggestion_no_code(range, &suggestion, Applicability::Unspecified)
        } else {
            d
        };
        let mut console = EnvConsole::default();
        console.log(markup! {
            {d.display(&files)}
        });
        panic!("Wrong reference.");
    }

    fn assert_reference_points_to_correct_declaration(
        declared_at: &Option<TextRange>,
        expected_declaration_range: &TextRange,
        file_name: &str,
        code: &str,
        range: &TextRange,
    ) {
        if *declared_at != Some(*expected_declaration_range) {
            let files = SimpleFile::new(file_name.to_string(), code.into());
            let d =
                Diagnostic::error(0, "", "Wrong reference.").primary(range, "This reference...");

            let d = if let Some(declared_at) = declared_at {
                d.primary(range, "This reference...").label(
                    Severity::Error,
                    declared_at,
                    "... is pointing to this declaration ...",
                )
            } else {
                d.primary(
                    range,
                    "This reference is not pointing to any declaration ...",
                )
            };

            let d = d.secondary(
                expected_declaration_range,
                "... but this was the expected declaration",
            );

            let mut console = EnvConsole::default();
            console.log(markup! {
                {d.display(&files)}
            });
        }
        assert_eq!(*declared_at, Some(*expected_declaration_range));
    }

    #[test]
    pub fn ok_symbol_resolution() {
        asserts_references(
            std::file!(),
            std::line!(),
            r#"
let global/*#GLOBAL*/ = 1;
console.log(global/*@GLOBAL*/);

function f(a/*#A1*/) {/*START SCOPE1*/
    console.log(global/*@GLOBAL*/);

    let b/*#B*/ = 1;
    let c/*#C1*/ = b/*@B*/ + 1;

    if (c/*@C1*/ == 1) {
        console.log(global/*@GLOBAL*/);

        let c/*#C2*/ = 2;
        console.log(b/*@B*/, c/*@C2*/);
    }

    for(const c/*#C3*/ of [1,2,3]) {
        console.log(global/*@GLOBAL*/);
        console.log(b/*@B*/, c/*@C3*/);
    }

    for(const c/*#C4*/ in [1,2,3]) {
        console.log(global/*@GLOBAL*/);
        console.log(b/*@B*/, c/*@C4*/);
    }

    while (c/*@C1*/ == 1) {
        console.log(global/*@GLOBAL*/);

        let c/*#C5*/ = 2;
        console.log(b/*@B*/, c/*@C5*/);
    }

    {
        console.log(global/*@GLOBAL*/);

        let c/*#C6*/ = 2;
        console.log(b/*@B*/, c/*@C6*/);
    }

    function inner(a/*#A2*/) {
        console.log(global/*@GLOBAL*/);
        console.log(a/*@A2*/, b/*@B*/, c/*@C1*/);

        let global/*#GLOBAL2*/ = 1;
        console.log(global/*@GLOBAL2*/);
    }

    console.log(global/*@GLOBAL*/);

    ((c/*#C7*/) => console.log(a/*@A1*/, b/*@B*/, c/*@C7*/))();

    console.log(x/*@HOISTED-X*/, w, u);
    {
        let b/*#B2*/ = 2;
        console.log(x/*@HOISTED-X*/, w, u);
        try
        {
            let a/*#A4*/ = 1;
            console.log(a/*@A4*/, b/*@B2*/, y/*@Y*/);
            console.log('try', x/*@HOISTED-X*/, w, u);
            var x/*#HOISTED-X*/;
        } catch (a/*#A5*/) {
            console.log(a/*@A5*/, b/*@B2*/, y/*@Y*/);
            let a/*#A6*/ = 1;
            console.log(a/*@A6*/, b/*@B2*/, y/*@Y*/);
            console.log('catch', x/*@HOISTED-X*/, w, u);
            var w;
        } finally {
            console.log(a/*@A1*/, b/*@B2*/, y/*@Y*/);
            console.log('finally', x/*@HOISTED-X*/, w, u);
            var u;
        }
        console.log(a/*@A1*/, b/*@B2*/, y);
        console.log(x/*@HOISTED-X*/, w, u);
        var y/*#Y*/;
    }

    return a/*@A1*/ + b/*@B*/ + c/*@C1*/;
}/*END SCOPE1*/

console.log(global/*@GLOBAL*/);

class Car/*#CAR*/ {
    constructor(a/*#A3*/, b) {
        let b/*#B1*/ = 2;
        console.log(a/*@A3*/, b/*@B1*/);
    }

    get name() {
        console.log(b/*@?*/);
        return "hi";
    }

    set name(v/*#V1*/) {
        console.log(b/*@?*/);
        this.name = v/*@V1*/;
    }
}

let car = new Car/*@CAR*/();

console.log(f1/*@HOISTED-F1*/);
console.log(f2/*@HOISTED-F2*/);

function f1/*#HOISTED-F1*/ () {
    console.log(a/*@HOISTED-A1*/, z/*@HOISTED-Z*/);
    var a/*#HOISTED-A1*/ = 1;

    console.log(b/*@HOISTED-B*/);
    if (b == 1) {
        var b/*#HOISTED-B*/;
    }
}

function f2/*#HOISTED-F2*/ () {
    console.log(a/*@HOISTED-A2*/, z/*@HOISTED-Z2*/);
    var a/*#HOISTED-A2*/ = 1;
    var z/*#HOISTED-Z2*/;
}

var z/*#HOISTED-Z*/ ;
"#,
        );
    }

    #[test]
    pub fn ok_scope_function() {
        asserts_references(
            std::file!(),
            std::line!(),
            r#"
function f() {/*START SCOPE1*/
}/*END SCOPE1*/"#,
        )
    }

    #[test]
    pub fn ok_scope_inline_function() {
        asserts_references(
            std::file!(),
            std::line!(),
            r#"
function f1() {/*START SCOPE1*/
    function f2() {/*START SCOPE2*/
    }/*END SCOPE2*/"
}/*END SCOPE1*/"#,
        )
    }

    #[test]
    pub fn ok_scope_function_with_statements() {
        asserts_references(
            std::file!(),
            std::line!(),
            r#"
function f1() {/*START SCOPE1*/
    if (1==1) {/*START SCOPE2*/
    }/*END SCOPE2*/

    for (;;) {/*START SCOPE3*/
    }/*END SCOPE3*/

    while (1==1) {/*START SCOPE4*/
    }/*END SCOPE4*/

    do {/*START SCOPE5*/
    }/*END SCOPE5*/
    while (1==1);

    let x = (/*START SCOPE6*/) => {}/*END SCOPE6*/;
    let y = (/*START SCOPE7*/y) => y/*END SCOPE7*/;
}/*END SCOPE1*/"#,
        )
    }
}
