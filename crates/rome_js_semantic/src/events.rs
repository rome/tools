//! Events emitted by the [SemanticEventExtractor] which are then constructed into the Semantic Model

use std::collections::{HashMap, VecDeque};

use rome_js_syntax::{
    JsForVariableDeclaration, JsIdentifierAssignment, JsIdentifierBinding, JsLanguage,
    JsReferenceIdentifier, JsSyntaxKind, JsSyntaxNode, JsSyntaxToken, JsVariableDeclaration,
    JsVariableDeclarator, JsVariableDeclaratorList, TextRange, TextSize,
};
use rome_rowan::{syntax::Preorder, AstNode, SyntaxNodeCast, SyntaxTokenText};

/// Events emitted by the [SemanticEventExtractor]. These events are later
/// made into the Semantic Model.
#[derive(Debug)]
pub enum SemanticEvent {
    /// Tracks where a new symbol declaration is found.
    /// Generated for:
    /// - Variable Declarations
    /// - Import bindings
    /// - Functions parameters
    DeclarationFound {
        range: TextRange,
        scope_started_at: TextSize,
        scope_id: usize,
        hoisted_scope_id: Option<usize>,
        name: SyntaxTokenText,
    },

    /// Tracks where a symbol is read, but only if its declaration
    /// is before this refence.
    /// Generated for:
    /// - All reference identifiers
    Read {
        range: TextRange,
        declared_at: TextRange,
    },

    /// Tracks where a symbol is read, but only if its declaration
    /// was hoisted. This means that its declaration is after this reference.
    /// - All reference identifiers
    HoistedRead {
        range: TextRange,
        declared_at: TextRange,
    },

    /// Tracks where a symbol is written, but only if its declaration
    /// is before this refence.
    /// Generated for:
    /// - All identifier assignments
    Write {
        range: TextRange,
        declared_at: TextRange,
    },

    /// Tracks where a symbol is written, but only if its declaration
    /// was hoisted. This means that its declaration is after this reference.
    /// Generated for:
    /// - All identifier assignments
    HoistedWrite {
        range: TextRange,
        declared_at: TextRange,
    },

    /// Tracks references that do no have any matching binding
    /// Generated for:
    /// - Unmatched reference identifiers
    UnresolvedReference { range: TextRange },

    /// Tracks where a new scope starts
    /// Generated for:
    /// - Blocks
    /// - Function body
    ScopeStarted {
        range: TextRange,
        scope_id: usize,
        parent_scope_id: Option<usize>,
    },

    /// Tracks where a scope ends
    /// Generated for:
    /// - Blocks
    /// - Function body
    ScopeEnded {
        range: TextRange,
        started_at: TextSize,
        scope_id: usize,
    },
}

impl SemanticEvent {
    pub fn range(&self) -> &TextRange {
        match self {
            SemanticEvent::DeclarationFound { range, .. } => range,
            SemanticEvent::ScopeStarted { range, .. } => range,
            SemanticEvent::ScopeEnded { range, .. } => range,
            SemanticEvent::Read { range, .. } => range,
            SemanticEvent::HoistedRead { range, .. } => range,
            SemanticEvent::Write { range, .. } => range,
            SemanticEvent::HoistedWrite { range, .. } => range,
            SemanticEvent::UnresolvedReference { range } => range,
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
    next_scope_id: usize,
    bindings: HashMap<SyntaxTokenText, TextRange>,
}

struct Binding {
    name: SyntaxTokenText,
}

#[derive(Debug)]
enum Reference {
    Read { range: TextRange },
    Write { range: TextRange },
}

impl Reference {
    pub fn range(&self) -> &TextRange {
        match self {
            Reference::Read { range } => range,
            Reference::Write { range } => range,
        }
    }
}

pub enum ScopeHoisting {
    DontHoistDeclarationsToParent,
    HoistDeclarationsToParent,
}

struct Scope {
    scope_id: usize,
    started_at: TextSize,
    /// All bindings declared inside this scope
    bindings: Vec<Binding>,
    /// References that still needs to be bound
    references: HashMap<SyntaxTokenText, Vec<Reference>>,
    /// All bindings that where shadowed and will be
    /// restored after this scope ends.
    shadowed: Vec<(SyntaxTokenText, TextRange)>,
    /// If this scope allows declarations to be hoisted
    /// to parent scope or not
    hoisting: ScopeHoisting,
}

impl SemanticEventExtractor {
    pub fn new() -> Self {
        Self {
            stash: VecDeque::new(),
            scopes: vec![],
            next_scope_id: 0,
            bindings: HashMap::new(),
        }
    }

    /// See [SemanticEvent] for a more detailed description
    /// of which ```SyntaxNode``` generates which events.
    #[inline]
    pub fn enter(&mut self, node: &JsSyntaxNode) {
        use rome_js_syntax::JsSyntaxKind::*;

        match node.kind() {
            JS_IDENTIFIER_BINDING => {
                self.enter_js_identifier_binding(node);
            }
            JS_REFERENCE_IDENTIFIER => {
                self.enter_js_reference_identifier(node);
            }
            JS_IDENTIFIER_ASSIGNMENT => {
                self.enter_js_identifier_assignment(node);
            }

            JS_MODULE | JS_SCRIPT => self.push_scope(
                node.text_range(),
                ScopeHoisting::DontHoistDeclarationsToParent,
            ),
            JS_FUNCTION_DECLARATION
            | JS_ARROW_FUNCTION_EXPRESSION
            | JS_CONSTRUCTOR_CLASS_MEMBER
            | JS_GETTER_CLASS_MEMBER
            | JS_SETTER_CLASS_MEMBER
            | JS_FUNCTION_BODY => {
                self.push_scope(
                    node.text_range(),
                    ScopeHoisting::DontHoistDeclarationsToParent,
                );
            }

            JS_BLOCK_STATEMENT | JS_FOR_STATEMENT | JS_FOR_OF_STATEMENT | JS_FOR_IN_STATEMENT
            | JS_CATCH_CLAUSE => {
                self.push_scope(node.text_range(), ScopeHoisting::HoistDeclarationsToParent);
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

    fn enter_js_identifier_binding(&mut self, node: &JsSyntaxNode) -> Option<()> {
        let binding = node.clone().cast::<JsIdentifierBinding>()?;
        let name_token = binding.name_token().ok()?;

        use JsSyntaxKind::*;
        match node.parent().map(|parent| parent.kind()) {
            Some(JS_VARIABLE_DECLARATOR) => {
                if let Some(true) = Self::is_var(&binding) {
                    let scope_idx = self.scope_index_to_hoist_declarations();
                    self.push_binding_into_scope(scope_idx, &name_token);
                } else {
                    self.push_binding_into_scope(None, &name_token);
                };
            }
            Some(JS_FUNCTION_DECLARATION) => {
                let scope_idx = self.scope_index_to_hoist_declarations();
                self.push_binding_into_scope(scope_idx, &name_token);
            }
            Some(_) => {
                self.push_binding_into_scope(None, &name_token);
            }
            _ => {}
        }

        Some(())
    }

    fn enter_js_reference_identifier(&mut self, node: &JsSyntaxNode) -> Option<()> {
        let reference = node.clone().cast::<JsReferenceIdentifier>()?;
        let name_token = reference.value_token().ok()?;
        let name = name_token.token_text_trimmed();

        let current_scope = self.current_scope_mut();
        let references = current_scope.references.entry(name).or_default();
        references.push(Reference::Read {
            range: node.text_range(),
        });

        Some(())
    }

    fn enter_js_identifier_assignment(&mut self, node: &JsSyntaxNode) -> Option<()> {
        let reference = node.clone().cast::<JsIdentifierAssignment>()?;
        let name_token = reference.name_token().ok()?;
        let name = name_token.token_text_trimmed();

        let current_scope = self.current_scope_mut();
        let references = current_scope.references.entry(name).or_default();
        references.push(Reference::Write {
            range: node.text_range(),
        });

        Some(())
    }

    /// See [SemanticEvent] for a more detailed description
    /// of which ```SyntaxNode``` generates which events.
    #[inline]
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
    #[inline]
    pub fn pop(&mut self) -> Option<SemanticEvent> {
        self.stash.pop_front()
    }

    fn push_scope(&mut self, range: TextRange, hoisting: ScopeHoisting) {
        let scope_id = self.next_scope_id;
        self.next_scope_id += 1;

        let parent_scope_id = self.scopes.iter().last().map(|x| x.scope_id);

        self.stash.push_back(SemanticEvent::ScopeStarted {
            range,
            scope_id,
            parent_scope_id,
        });

        self.scopes.push(Scope {
            scope_id,
            started_at: range.start(),
            bindings: vec![],
            references: HashMap::new(),
            shadowed: vec![],
            hoisting,
        });
    }

    /// When a scope dies we do the following:
    /// 1 - Match all references and declarations;
    /// 2 - Unmatched references are promoted to its parent scope or become [UnresolvedReference] events;
    /// 3 - All declarations of this scope are removed;
    /// 4 - All shawed declarations are restored.
    fn pop_scope(&mut self, range: TextRange) {
        debug_assert!(!self.scopes.is_empty());

        if let Some(scope) = self.scopes.pop() {
            // Match references and declarations
            for (name, references) in scope.references {
                // If we know the declaration of these reference push the correct events...
                if let Some(declaration_at) = self.bindings.get(&name) {
                    for reference in references {
                        let declaration_before_reference =
                            declaration_at.start() < reference.range().start();
                        let e = match (declaration_before_reference, reference) {
                            (true, Reference::Read { range }) => SemanticEvent::Read {
                                range,
                                declared_at: *declaration_at,
                            },
                            (false, Reference::Read { range }) => SemanticEvent::HoistedRead {
                                range,
                                declared_at: *declaration_at,
                            },
                            (true, Reference::Write { range }) => SemanticEvent::Write {
                                range,
                                declared_at: *declaration_at,
                            },
                            (false, Reference::Write { range }) => SemanticEvent::HoistedWrite {
                                range,
                                declared_at: *declaration_at,
                            },
                        };
                        self.stash.push_back(e);
                    }
                } else if let Some(parent) = self.scopes.last_mut() {
                    // ... if not, promote these references to the parent scope ...
                    parent.references.insert(name, references);
                } else {
                    // ... or raise UnresolvedReference if this is the global scope.
                    for reference in references {
                        self.stash.push_back(SemanticEvent::UnresolvedReference {
                            range: *reference.range(),
                        });
                    }
                }
            }

            // Remove all bindings declared in this scope
            for binding in scope.bindings {
                self.bindings.remove(&binding.name);
            }

            // Restore shadowed bindings
            self.bindings.extend(scope.shadowed);

            self.stash.push_back(SemanticEvent::ScopeEnded {
                range,
                started_at: scope.started_at,
                scope_id: scope.scope_id,
            });
        }
    }

    fn current_scope_mut(&mut self) -> &mut Scope {
        // We should at least have the global scope
        debug_assert!(!self.scopes.is_empty());

        match self.scopes.last_mut() {
            Some(scope) => scope,
            None => unreachable!(),
        }
    }

    /// Finds the scope where declarations that are hoisted
    /// will be declared at. For example:
    ///
    /// ```js
    /// function f() {
    ///     if (true) {
    ///         var a;
    ///     }
    /// }
    /// ```
    ///
    /// `a` declaration will be hoisted to the scope of
    /// function `f`.
    ///
    /// This method when called inside the `f` scope will return
    /// the `f` scope index.
    fn scope_index_to_hoist_declarations(&mut self) -> Option<usize> {
        // We should at least have the global scope
        // that do not hoist
        debug_assert!(matches!(
            self.scopes[0].hoisting,
            ScopeHoisting::DontHoistDeclarationsToParent
        ));
        debug_assert!(!self.scopes.is_empty());

        let idx = self.scopes.iter().rev().position(|scope| {
            matches!(scope.hoisting, ScopeHoisting::DontHoistDeclarationsToParent)
        });

        let current_scope_id = self.current_scope_mut().scope_id;
        match idx {
            Some(idx) => {
                if idx == current_scope_id {
                    None
                } else {
                    Some(idx)
                }
            }
            // Worst case this will fallback to the global scope
            // which will be idx = 0
            None => unreachable!("We must have a least of scope."),
        }
    }

    fn push_binding_into_scope(
        &mut self,
        hoisted_scope_id: Option<usize>,
        name_token: &JsSyntaxToken,
    ) {
        let name = name_token.token_text_trimmed();

        let declaration_range = name_token.text_range();

        // insert this name into the list of available names
        // and save shadowed names to be used later
        let shadowed = self
            .bindings
            .insert(name.clone(), declaration_range)
            .map(|shadowed_range| (name.clone(), shadowed_range));

        let scope_id = self.scopes.len() - 1;
        let binding_scope_id = hoisted_scope_id.unwrap_or(scope_id);

        debug_assert!(binding_scope_id < self.scopes.len());
        let scope = &mut self.scopes[binding_scope_id];
        scope.bindings.push(Binding { name: name.clone() });
        scope.shadowed.extend(shadowed);
        let scope_started_at = scope.started_at;

        self.stash.push_back(SemanticEvent::DeclarationFound {
            range: declaration_range,
            scope_started_at,
            scope_id,
            hoisted_scope_id,
            name,
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
