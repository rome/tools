//! Events emitted by the [SemanticEventExtractor] which are then constructed into the Semantic Model

use std::collections::{HashMap, VecDeque};

use rome_js_syntax::{
    JsForVariableDeclaration, JsIdentifierBinding, JsLanguage, JsReferenceIdentifier, JsSyntaxKind,
    JsSyntaxNode, JsSyntaxToken, JsVariableDeclaration, JsVariableDeclarator,
    JsVariableDeclaratorList, TextRange, TextSize,
};
use rome_rowan::{syntax::Preorder, AstNode, SyntaxNodeCast, SyntaxTokenText};

/// Events emitted by the [SemanticEventExtractor]. These events are later
/// made into the Semantic Model.
#[derive(Debug)]
pub enum SemanticEvent {
    /// Tracks when a new symbol declaration is found.
    /// Generated for:
    /// - Variable Declarations
    /// - Import bindings
    /// - Functions parameters
    DeclarationFound {
        range: TextRange,
        scope_started_at: TextSize,
    },

    /// Tracks when a symbol is read.
    /// Generated for:
    /// - All reference identifiers
    Read {
        range: TextRange,
        declaration_at: Option<TextRange>,
    },

    HoistedRead {
        range: TextRange,
        declaration_at: TextRange,
    },

    /// Tracks when a reference do no have any matching
    /// binding
    /// Generated for:
    /// - Unmatched reference identifiers
    UnresolvedReference { range: TextRange },

    /// Tracks when a new scope starts
    /// Generated for:
    /// - Blocks
    /// - Function body
    ScopeStarted { range: TextRange },

    /// Tracks when a scope ends
    /// Generated for:
    /// - Blocks
    /// - Function body
    ScopeEnded {
        range: TextRange,
        started_at: TextSize,
    },
}

impl SemanticEvent {
    pub fn range(&self) -> &TextRange {
        match self {
            SemanticEvent::DeclarationFound { range, .. } => range,
            SemanticEvent::ScopeStarted { range } => range,
            SemanticEvent::ScopeEnded { range, .. } => range,
            SemanticEvent::Read { range, .. } => range,
            SemanticEvent::UnresolvedReference { range } => range,
            SemanticEvent::HoistedRead { range, .. } => range,
        }
    }

    pub fn str<'a>(&self, code: &'a str) -> &'a str {
        let range = self.range();
        let start: u32 = range.start().into();
        let end: u32 = range.end().into();
        &code[start as usize..end as usize]
    }
}

/// Extracts [SemanticEvent] from [SyntaxNode].
///
/// The extraction is not entirely pull based, nor entirely push based.
/// This happens because some nodes can generate multiple events.
/// A hoisted variable declaration like ```var a```, being the more obvious
/// example. As soon ```a``` is hoisted, all references of ```a``` are solved
/// on this node.
///
/// For a simpler way to extract [SemanticEvent] see [semantic_events] or [SemanticEventIterator].
///
/// To use the [SemanticEventExtractor] one must push the current node, following
/// the [PreOrder] of the tree, and must pull events until [Pop] returns [None].
///
/// ```rust
/// use rome_js_parser::*;
/// use rome_js_syntax::*;
/// use rome_js_semantic::*;
/// let tree = parse("let a = 1", 0, SourceType::js_script());
/// let mut extractor = SemanticEventExtractor::new();
/// for e in tree.syntax().preorder() {
///     match e {
///         WalkEvent::Enter(node) => extractor.enter(&node),
///         WalkEvent::Leave(node) => extractor.leave(&node),
///         _ => {}
///     }
///     
///     while let Some(e) = extractor.pop() {
///         dbg!(e);
///     }
/// }
/// ```
#[derive(Default)]
pub struct SemanticEventExtractor {
    stash: VecDeque<SemanticEvent>,
    scopes: Vec<Scope>,
    bindings: HashMap<SyntaxTokenText, TextRange>,
}

struct Binding {
    name: SyntaxTokenText,
}

#[derive(Debug)]
struct Reference {
    range: TextRange,
}

struct Scope {
    started_at: TextSize,
    /// All bindings declared inside this scope
    bindings: Vec<Binding>,
    /// Reference that do not have a matching declaration
    references: HashMap<SyntaxTokenText, Vec<Reference>>,
    /// All bindings that where shadowed and will be
    /// restored after this scope ends.
    shadowed: Vec<(SyntaxTokenText, TextRange)>,
    /// If this scope allows declarations to hoist to parent scope
    /// or not
    allows_decl_hoisting: bool,
}

impl SemanticEventExtractor {
    pub fn new() -> Self {
        Self {
            stash: VecDeque::new(),
            scopes: vec![],
            bindings: HashMap::new(),
        }
    }

    /// See [SemanticEvent] for a more detailed description
    /// of which ```SyntaxNode``` generates which events.
    pub fn enter(&mut self, node: &JsSyntaxNode) {
        use rome_js_syntax::JsSyntaxKind::*;

        match node.kind() {
            JS_IDENTIFIER_BINDING => {
                self.enter_identifier_binding(node);
            }
            JS_REFERENCE_IDENTIFIER => {
                self.enter_reference_identifier(node);
            }

            JS_MODULE | JS_SCRIPT => self.push_scope(node.text_range(), false),
            JS_FUNCTION_DECLARATION
            | JS_ARROW_FUNCTION_EXPRESSION
            | JS_CONSTRUCTOR_CLASS_MEMBER
            | JS_GETTER_CLASS_MEMBER
            | JS_SETTER_CLASS_MEMBER
            | JS_FUNCTION_BODY => {
                self.push_scope(node.text_range(), false);
            }

            JS_BLOCK_STATEMENT | JS_FOR_STATEMENT | JS_FOR_OF_STATEMENT | JS_FOR_IN_STATEMENT
            | JS_CATCH_CLAUSE => {
                self.push_scope(node.text_range(), true);
            }
            _ => {}
        }
    }

    fn is_var(binding: &JsIdentifierBinding) -> Option<bool> {
        let declarator = binding.parent::<JsVariableDeclarator>()?;

        use JsSyntaxKind::*;
        let is_var = match declarator.syntax().parent().map(|parent| parent.kind()) {
            Some(JS_VARIABLE_DECLARATOR_LIST) => declarator
                .parent::<JsVariableDeclaratorList>()?
                .parent::<JsVariableDeclaration>()?
                .is_var(),
            Some(JS_FOR_VARIABLE_DECLARATION) => {
                declarator
                    .parent::<JsForVariableDeclaration>()?
                    .kind_token()
                    .ok()?
                    .kind()
                    == VAR_KW
            }
            _ => false,
        };
        Some(is_var)
    }

    fn enter_identifier_binding(&mut self, node: &JsSyntaxNode) -> Option<()> {
        let binding = node.clone().cast::<JsIdentifierBinding>()?;
        let name_token = binding.name_token().ok()?;

        use JsSyntaxKind::*;
        match node.parent().map(|parent| parent.kind()) {
            Some(JS_VARIABLE_DECLARATOR) => {
                if let Some(true) = Self::is_var(&binding) {
                    let scope_idx = self.current_not_hoisting_scope_index();
                    self.push_binding_into_scope(scope_idx, &name_token);
                    self.solve_pending_references(node, &binding, &name_token);
                } else {
                    let scope_idx = self.scopes.len() - 1;
                    self.push_binding_into_scope(scope_idx, &name_token);
                };
            }
            Some(_) => {
                let scope_idx = self.scopes.len() - 1;
                self.push_binding_into_scope(scope_idx, &name_token);
            }
            _ => {}
        }

        Some(())
    }

    fn solve_pending_references(
        &mut self,
        node: &JsSyntaxNode,
        binding: &JsIdentifierBinding,
        name_token: &JsSyntaxToken,
    ) -> Option<()> {
        let is_var = binding
            .parent::<JsVariableDeclarator>()?
            .parent::<JsVariableDeclaratorList>()?
            .parent::<JsVariableDeclaration>()?
            .is_var();

        if is_var {
            let name = name_token.token_text_trimmed();

            // Solve pending references in parent scopes if the
            // current scope is flagged as hoists = false.
            let scopes = self.scopes.iter_mut().rev();
            for scope in scopes {
                if let Some(references) = scope.references.remove(&name) {
                    for reference in references {
                        self.stash.push_back(SemanticEvent::HoistedRead {
                            range: reference.range,
                            declaration_at: node.text_range(),
                        })
                    }
                }

                if !scope.allows_decl_hoisting {
                    break;
                }
            }
        }

        Some(())
    }

    fn enter_reference_identifier(&mut self, node: &JsSyntaxNode) -> Option<()> {
        let reference = node.clone().cast::<JsReferenceIdentifier>()?;
        let name_token = reference.value_token().ok()?;
        let name = name_token.token_text_trimmed();

        let current_scope = self.current_scope_mut();
        let references = current_scope.references.entry(name).or_default();
        references.push(Reference {
            range: node.text_range(),
        });

        Some(())
    }

    /// See [SemanticEvent] for a more detailed description
    /// of which ```SyntaxNode``` generates which events.
    pub fn leave(&mut self, node: &JsSyntaxNode) {
        use rome_js_syntax::JsSyntaxKind::*;

        match node.kind() {
            JS_MODULE | JS_SCRIPT => self.pop_scope(node.text_range()),
            JS_FUNCTION_DECLARATION
            | JS_ARROW_FUNCTION_EXPRESSION
            | JS_CONSTRUCTOR_CLASS_MEMBER
            | JS_GETTER_CLASS_MEMBER
            | JS_SETTER_CLASS_MEMBER
            | JS_BLOCK_STATEMENT
            | JS_FOR_STATEMENT
            | JS_FOR_OF_STATEMENT
            | JS_FOR_IN_STATEMENT
            | JS_CATCH_CLAUSE
            | JS_FUNCTION_BODY => {
                self.pop_scope(node.text_range());
            }
            _ => {}
        }
    }

    /// Return any previous extracted [SemanticEvent].
    pub fn pop(&mut self) -> Option<SemanticEvent> {
        self.stash.pop_front()
    }

    fn push_scope(&mut self, range: TextRange, allows_decl_hoisting: bool) {
        self.stash.push_back(SemanticEvent::ScopeStarted { range });
        self.scopes.push(Scope {
            started_at: range.start(),
            bindings: vec![],
            references: HashMap::new(),
            shadowed: vec![],
            allows_decl_hoisting,
        });
    }

    fn pop_scope(&mut self, range: TextRange) {
        debug_assert!(!self.scopes.is_empty());

        if let Some(scope) = self.scopes.pop() {
            // Solve all references ..
            for (name, references) in scope.references {
                if let Some(declaration_at) = self.bindings.get(&name) {
                    for reference in references {
                        let e = if declaration_at.start() < reference.range.start() {
                            SemanticEvent::Read {
                                range: reference.range,
                                declaration_at: Some(*declaration_at),
                            }
                        } else {
                            SemanticEvent::HoistedRead {
                                range: reference.range,
                                declaration_at: *declaration_at,
                            }
                        };
                        self.stash.push_back(e);
                    }
                } else if let Some(parent) = self.scopes.last_mut() {
                    // .. and promote pending references to the parent scope
                    parent.references.insert(name, references);
                } else {
                    // ... or raise UnresolvedReference events
                    // when popping the global scope
                    for reference in references {
                        self.stash.push_back(SemanticEvent::UnresolvedReference {
                            range: reference.range,
                        });
                    }
                }
            }

            // Remove all bindings declared in this scope
            for binding in scope.bindings {
                self.bindings.remove(&binding.name);
            }

            // Return shadowed bindings
            self.bindings.extend(scope.shadowed);

            self.stash.push_back(SemanticEvent::ScopeEnded {
                range,
                started_at: scope.started_at,
            });
        }
    }

    fn current_scope_mut(&mut self) -> &mut Scope {
        // We should at least have the global scope
        debug_assert!(!self.scopes.is_empty());

        match self.scopes.last_mut() {
            None => unreachable!(),
            Some(scope) => scope,
        }
    }

    fn current_not_hoisting_scope_index(&mut self) -> usize {
        // We should at least have the global scope
        // that do not hoist
        debug_assert!(!self.scopes[0].allows_decl_hoisting);
        debug_assert!(!self.scopes.is_empty());

        let idx = self
            .scopes
            .iter()
            .rev()
            .position(|scope| !scope.allows_decl_hoisting);

        match idx {
            None => unreachable!(),
            Some(idx) => idx,
        }
    }

    fn push_binding_into_scope(&mut self, scope_idx: usize, name_token: &JsSyntaxToken) {
        let name = name_token.token_text_trimmed();

        let declaration_range = name_token.text_range();

        // insert this name into the list of available names
        // and save shadowed names to be used later
        let shadowed = self
            .bindings
            .insert(name.clone(), declaration_range)
            .map(|shadowed_range| (name.clone(), shadowed_range));

        debug_assert!(scope_idx < self.scopes.len());
        let scope = &mut self.scopes[scope_idx];
        scope.bindings.push(Binding { name });
        scope.shadowed.extend(shadowed);
        let scope_started_at = scope.started_at;

        self.stash.push_back(SemanticEvent::DeclarationFound {
            range: declaration_range,
            scope_started_at,
        });
    }
}

/// Extracts [SemanticEvent] from [SyntaxNode].
/// See [semantic_events] how to create this iterator.
struct SemanticEventIterator {
    iter: Preorder<JsLanguage>,
    extractor: SemanticEventExtractor,
}

impl Iterator for SemanticEventIterator {
    type Item = SemanticEvent;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            if let Some(e) = self.extractor.pop() {
                break Some(e);
            } else {
                use rome_js_syntax::WalkEvent::*;
                match self.iter.next() {
                    Some(Enter(node)) => {
                        self.extractor.enter(&node);
                    }
                    Some(Leave(node)) => {
                        self.extractor.leave(&node);
                    }
                    None => {
                        if let Some(e) = self.extractor.pop() {
                            break Some(e);
                        } else {
                            break None;
                        }
                    }
                }
            }
        }
    }
}

/// Extracts [SemanticEvent] from [SyntaxNode].
///
/// For a way to extract [SemanticEvent] which gives more control see [SemanticEventExtractor].
///
/// ```rust
/// use rome_js_parser::*;
/// use rome_js_syntax::*;
/// use rome_js_semantic::*;
/// let tree = parse("let a = 1", 0, SourceType::js_script());
/// for e in semantic_events(tree.syntax()) {
///     dbg!(e);
/// }
/// ```
pub fn semantic_events(root: JsSyntaxNode) -> impl IntoIterator<Item = SemanticEvent> {
    SemanticEventIterator {
        iter: root.preorder(),
        extractor: SemanticEventExtractor::default(),
    }
}
