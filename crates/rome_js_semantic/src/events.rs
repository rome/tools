//! Events emitted by the [SemanticEventExtractor] which are then constructed into the Semantic Model

use std::collections::{HashMap, VecDeque};

use rome_js_syntax::{
    JsIdentifierBinding, JsLanguage, JsReferenceIdentifier, JsSyntaxNode, JsSyntaxToken, TextRange,
    TextSize,
};
use rome_rowan::{syntax::Preorder, SyntaxNodeCast, SyntaxTokenText};

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

struct Scope {
    started_at: TextSize,
    /// All bindings declared inside this scope
    bindings: Vec<Binding>,
    /// All bindings that where shadowed and will be
    /// restored after this scope ends.
    shadowed: Vec<(SyntaxTokenText, TextRange)>,
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
                if let Some(name_token) = node
                    .clone()
                    .cast::<JsIdentifierBinding>()
                    .and_then(|id| id.name_token().ok())
                {
                    self.push_binding_into_current_scope(&name_token);
                }
            }
            JS_REFERENCE_IDENTIFIER => {
                if let Some(name_token) = node
                    .clone()
                    .cast::<JsReferenceIdentifier>()
                    .and_then(|reference| reference.value_token().ok())
                {
                    self.stash.push_back(SemanticEvent::Read {
                        range: node.text_range(),
                        declaration_at: self.get_binding_range(&name_token).cloned(),
                    })
                }
            }

            JS_MODULE | JS_SCRIPT => self.push_scope(node.text_range()),
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
                self.push_scope(node.text_range());
            }
            _ => {}
        }
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

    fn push_scope(&mut self, range: TextRange) {
        self.stash.push_back(SemanticEvent::ScopeStarted { range });
        self.scopes.push(Scope {
            started_at: range.start(),
            bindings: vec![],
            shadowed: vec![],
        });
    }

    fn pop_scope(&mut self, range: TextRange) {
        if let Some(scope) = self.scopes.pop() {
            self.stash.push_back(SemanticEvent::ScopeEnded {
                range,
                started_at: scope.started_at,
            });

            for binding in scope.bindings {
                self.bindings.remove(&binding.name);
            }

            for (k, v) in scope.shadowed {
                self.bindings.insert(k, v);
            }
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

    fn push_binding_into_current_scope(&mut self, name_token: &JsSyntaxToken) {
        let name = name_token.token_text_trimmed();

        let declaration_range = name_token.text_range();

        // insert this name into the list of available names
        // and save shadowed names to be used later
        let shadowed = self
            .bindings
            .insert(name.clone(), declaration_range)
            .map(|shadowed_range| (name.clone(), shadowed_range));

        let current_scope = self.current_scope_mut();
        current_scope.bindings.push(Binding { name });
        current_scope.shadowed.extend(shadowed);
        let scope_started_at = current_scope.started_at;

        self.stash.push_back(SemanticEvent::DeclarationFound {
            range: declaration_range,
            scope_started_at,
        });
    }

    fn get_binding_range(&self, name_token: &JsSyntaxToken) -> Option<&TextRange> {
        let name = name_token.token_text_trimmed();
        self.bindings.get(&name)
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
