//! Events emitted by the [SemanticEventExtractor] which are then constructed into the Semantic Model

use std::collections::{HashMap, VecDeque};

use rome_js_syntax::{JsLanguage, JsSyntaxNode, TextRange, TextSize};
use rome_rowan::syntax::Preorder;

/// Events emitted by the [SemanticEventExtractor]. These events are later
/// made into the Semantic Model.
#[derive(Debug)]
pub enum SemanticEvent {
    /// Signifies that a new symbol declaration was found.
    /// Currently is generated for:
    /// - Variable Declarations
    /// - Import bindings
    /// - Functions parameters
    DeclarationFound {
        range: TextRange,
        scope_started_at: TextSize,
    },

    /// Signifies that a symbol value is being read.
    Read {
        range: TextRange,
        declaration_at: Option<TextRange>,
    },

    /// Signifies that a new scope was started
    /// Currently generated for:
    /// - Blocks
    /// - Function body
    ScopeStarted { range: TextRange },

    /// Signifies that a new scope was ended
    /// Currently generated for:
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
/// The extraction is not enterily pull based, nor entirely push based.
/// This happens because some nodes can generate multiple events.
/// A hoisted variable declaration like ```var a```, being the more obvious
/// example. As soon ```a``` is hoisted, all references of ```a``` are solved
/// on this node.
///
/// For a simpler way to extract [SemanticEvent] see [semantic_events] or [SemanticEventIterator].
///
/// To use the [SemanticEventExtractor] one must push the current node, following
/// the [PreOrder] of the tree, and must pull events unti [Pop] returuns [None].
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
    declared_names: HashMap<String, TextRange>,
}

struct ScopeDeclaration {
    name: String,
}

struct Scope {
    started_at: TextSize,
    declared: Vec<ScopeDeclaration>,
    shadowed: Vec<(String, TextRange)>,
}

impl SemanticEventExtractor {
    pub fn new() -> Self {
        Self {
            stash: VecDeque::new(),
            scopes: vec![],
            declared_names: HashMap::new(),
        }
    }

    /// See [SemanticEvent] for a more detailed description
    /// of which ```SyntaxNode``` generates which events.
    pub fn enter(&mut self, node: &JsSyntaxNode) {
        use rome_js_syntax::JsSyntaxKind::*;

        match node.kind() {
            JS_IDENTIFIER_BINDING => self.declare_name(node),
            JS_REFERENCE_IDENTIFIER => self.stash.push_back(SemanticEvent::Read {
                range: node.text_range(),
                declaration_at: self.get_name(node).cloned(),
            }),

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
            declared: vec![],
            shadowed: vec![],
        });
    }

    fn pop_scope(&mut self, range: TextRange) {
        if let Some(scope) = self.scopes.pop() {
            self.stash.push_back(SemanticEvent::ScopeEnded {
                range,
                started_at: scope.started_at,
            });

            // remove all declarations
            for decl in scope.declared {
                self.declared_names.remove(&decl.name);
            }

            // return all shadowed names
            for (name, range) in scope.shadowed {
                self.declared_names.insert(name, range);
            }
        }
    }

    fn declare_name(&mut self, node: &JsSyntaxNode) {
        let name = node.text_trimmed().to_string();
        let declaration_range = node.text_range();

        // insert this name into the list of available names
        // and get if any previous name is being shadowed
        let shadowed = self
            .declared_names
            .insert(name.clone(), declaration_range.clone())
            .map(|shadowed_range| (name.clone(), shadowed_range));

        let current_scope = self.scopes.last_mut().unwrap();
        current_scope.declared.push(ScopeDeclaration { name });
        current_scope.shadowed.extend(shadowed);

        self.stash.push_back(SemanticEvent::DeclarationFound {
            range: node.text_range(),
            scope_started_at: current_scope.started_at,
        });
    }

    fn get_name(&self, node: &JsSyntaxNode) -> Option<&TextRange> {
        let name = dbg!(node.text_trimmed().to_string());
        dbg!(&self.declared_names).get(&name)
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
