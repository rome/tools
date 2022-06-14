//! Events emitted by the [SemanticEventExtractor] which are then constructed into the Semantic Model

use std::collections::VecDeque;

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
    DeclarationFound { range: TextRange },

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
            SemanticEvent::DeclarationFound { range } => range,
            SemanticEvent::ScopeStarted { range } => range,
            SemanticEvent::ScopeEnded { range, .. } => range,
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
}

struct Scope {
    started_at: TextSize,
}

impl SemanticEventExtractor {
    pub fn new() -> Self {
        Self {
            stash: VecDeque::new(),
            scopes: vec![],
        }
    }

    /// See [SemanticEvent] for a more detailed description
    /// of which ```SyntaxNode``` generates which events.
    pub fn enter(&mut self, node: &JsSyntaxNode) {
        use rome_js_syntax::JsSyntaxKind::*;
        use SemanticEvent::*;

        match node.kind() {
            JS_IDENTIFIER_BINDING => self.stash.push_back(DeclarationFound {
                range: node.text_range(),
            }),
            JS_BLOCK_STATEMENT | JS_FUNCTION_BODY => self.push_scope(node.text_range()),
            _ => {}
        }
    }

    /// See [SemanticEvent] for a more detailed description
    /// of which ```SyntaxNode``` generates which events.
    pub fn leave(&mut self, node: &JsSyntaxNode) {
        use rome_js_syntax::JsSyntaxKind::*;

        match node.kind() {
            JS_BLOCK_STATEMENT | JS_FUNCTION_BODY => self.pop_scope(node.text_range()),
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
        });
    }

    fn pop_scope(&mut self, range: TextRange) {
        if let Some(scope) = self.scopes.pop() {
            self.stash.push_back(SemanticEvent::ScopeEnded {
                range,
                started_at: scope.started_at,
            });
        }
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
