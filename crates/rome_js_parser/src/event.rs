//! Events emitted by the Parser which are then constructed into a syntax tree

use std::mem;
use std::num::NonZeroU32;

use crate::lexer::TextSize;
use crate::parser::rewrite_parser::{RewriteParser, RewriteToken};
use crate::parser::Checkpoint;
use crate::{JsParser, ParseDiagnostic, TreeSink};
use rome_js_syntax::JsSyntaxKind::{self, *};
use rome_rowan::SyntaxKind;

/// Events emitted by the Parser, these events are later
/// made into a syntax tree with `process` into TreeSink.
#[derive(Debug, Clone)]
pub enum Event<K: SyntaxKind> {
    /// This event signifies the start of the node.
    /// It should be either abandoned (in which case the
    /// `kind` is `TOMBSTONE`, and the event is ignored),
    /// or completed via a `Finish` event.
    ///
    /// All tokens between a `Start` and a `Finish` would
    /// become the children of the respective node.
    Start {
        kind: K,
        forward_parent: Option<NonZeroU32>,
    },

    /// Complete the previous `Start` event
    Finish,

    /// Produce a single leaf-element.
    Token {
        kind: K,
        /// The end offset of this token.
        end: TextSize,
    },
}

impl<K: SyntaxKind> Event<K> {
    pub fn tombstone(kind: K) -> Self {
        Event::Start {
            kind,
            forward_parent: None,
        }
    }
}

/// Generate the syntax tree with the control of events.
#[inline]
pub fn process(
    sink: &mut impl TreeSink,
    mut events: Vec<Event<JsSyntaxKind>>,
    errors: Vec<ParseDiagnostic>,
) {
    sink.errors(errors);
    let mut forward_parents = Vec::new();

    for i in 0..events.len() {
        match &mut events[i] {
            Event::Start {
                kind: TOMBSTONE, ..
            } => (),

            Event::Start {
                kind,
                forward_parent,
                ..
            } => {
                // For events[A, B, C], B is A's forward_parent, C is B's forward_parent,
                // in the normal control flow, the parent-child relation: `A -> B -> C`,
                // while with the magic forward_parent, it writes: `C <- B <- A`.

                // append `A` into parents.
                forward_parents.push(*kind);
                let mut idx = i;
                let mut fp = *forward_parent;
                while let Some(fwd) = fp {
                    idx += u32::from(fwd) as usize;
                    // append `A`'s forward_parent `B`
                    fp = match mem::replace(&mut events[idx], Event::tombstone(TOMBSTONE)) {
                        Event::Start {
                            kind,
                            forward_parent,
                            ..
                        } => {
                            if kind != TOMBSTONE {
                                forward_parents.push(kind);
                            }
                            forward_parent
                        }
                        _ => unreachable!(),
                    };
                    // append `B`'s forward_parent `C` in the next stage.
                }

                for kind in forward_parents.drain(..).rev() {
                    sink.start_node(kind);
                }
            }
            Event::Finish { .. } => sink.finish_node(),
            Event::Token { kind, end } => {
                sink.token(*kind, *end);
            }
        }
    }
}

struct RewriteParseEventsTreeSink<'r, 'p, T> {
    reparse: &'r mut T,
    parser: RewriteParser<'r, 'p>,
}

impl<'r, 'p, T: RewriteParseEvents> TreeSink for RewriteParseEventsTreeSink<'r, 'p, T> {
    fn token(&mut self, kind: JsSyntaxKind, end: TextSize) {
        self.reparse
            .token(RewriteToken::new(kind, end), &mut self.parser);
    }

    fn start_node(&mut self, kind: JsSyntaxKind) {
        self.reparse.start_node(kind, &mut self.parser);
    }

    fn finish_node(&mut self) {
        self.reparse.finish_node(&mut self.parser);
    }

    fn errors(&mut self, _errors: Vec<ParseDiagnostic>) {}
}

/// Implement this trait if you want to change the tree structure
/// from already parsed events.
pub(crate) trait RewriteParseEvents {
    /// Called for a started node in the original tree
    fn start_node(&mut self, kind: JsSyntaxKind, p: &mut RewriteParser);

    /// Called for a finished node in the original tree
    fn finish_node(&mut self, p: &mut RewriteParser);

    /// Called for every token
    fn token(&mut self, token: RewriteToken, p: &mut RewriteParser) {
        p.bump(token)
    }
}

/// Allows rewriting a super grammar to a sub grammar by visiting each event emitted after the checkpoint.
/// Useful if a node turned out to be of a different kind its subtree must be re-shaped
/// (adding new nodes, dropping sub nodes, etc.).
pub(crate) fn rewrite_events<T: RewriteParseEvents>(
    rewriter: &mut T,
    checkpoint: Checkpoint,
    p: &mut JsParser,
) {
    // Only rewind the events but do not reset the parser errors nor parser state.
    // The current parsed grammar is a super-set of the grammar that gets re-parsed. Thus, any
    // error that applied to the old grammar also applies to the sub-grammar.
    let events: Vec<_> = unsafe { p.split_off_events(checkpoint.event_pos + 1) };

    let mut sink = RewriteParseEventsTreeSink {
        parser: RewriteParser::new(p, checkpoint.token_source),
        reparse: rewriter,
    };
    process(&mut sink, events, Vec::default());
    sink.parser.finish();
}
