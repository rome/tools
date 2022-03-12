//! Events emitted by the Parser which are then constructed into a syntax tree

use std::mem;

use crate::{Parser, ParserError, TreeSink};
use rome_js_syntax::JsSyntaxKind::{self, *};
use rome_rowan::TextRange;
use rslint_lexer::TextSize;

use crate::parser::Checkpoint;
use crate::token_source::Trivia;

/// Events emitted by the Parser, these events are later
/// made into a syntax tree with `process` into TreeSink.
#[derive(Debug, Clone)]
pub enum Event {
    /// This event signifies the start of the node.
    /// It should be either abandoned (in which case the
    /// `kind` is `TOMBSTONE`, and the event is ignored),
    /// or completed via a `Finish` event.
    ///
    /// All tokens between a `Start` and a `Finish` would
    /// become the children of the respective node.
    Start {
        kind: JsSyntaxKind,
        start: TextSize,
        forward_parent: Option<u32>,
    },

    /// Complete the previous `Start` event
    Finish { end: TextSize },

    /// Produce a single leaf-element.
    /// `n_raw_tokens` is used to glue complex contextual tokens.
    /// For example, lexer tokenizes `>>` as `>`, `>`, and
    /// `n_raw_tokens = 2` is used to produced a single `>>`.
    Token {
        kind: JsSyntaxKind,
        range: TextRange,
    },
}

impl Event {
    pub fn tombstone(start: TextSize) -> Self {
        Event::Start {
            kind: TOMBSTONE,
            forward_parent: None,
            start,
        }
    }
}

/// Generate the syntax tree with the control of events.
#[inline]
pub fn process(sink: &mut impl TreeSink, mut events: Vec<Event>, errors: Vec<ParserError>) {
    sink.errors(errors);
    let mut forward_parents = Vec::new();

    for i in 0..events.len() {
        match mem::replace(&mut events[i], Event::tombstone(TextSize::default())) {
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
                forward_parents.push(kind);
                let mut idx = i;
                let mut fp = forward_parent;
                while let Some(fwd) = fp {
                    idx += fwd as usize;
                    // append `A`'s forward_parent `B`
                    fp = match mem::replace(&mut events[idx], Event::tombstone(TextSize::default()))
                    {
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
            Event::Token { kind, range } => {
                sink.token(kind, range.len());
            }
        }
    }
}

struct RewriteParseEventsTreeSink<'r, 'p, T> {
    reparse: &'r mut T,
    parser: &'r mut Parser<'p>,
    offset: TextSize,
    trivia: &'r [Trivia],
}

impl<T> RewriteParseEventsTreeSink<'_, '_, T> {
    fn skip_trivia(&mut self, trailing: bool) {
        let mut processed = 0;
        for trivia in self.trivia {
            if trailing != trivia.trailing() || self.offset != trivia.offset() {
                break;
            }

            processed += 1;
            self.offset += trivia.len();
        }

        self.trivia = &self.trivia[processed..];
    }
}

impl<'r, 'p, T: RewriteParseEvents> TreeSink for RewriteParseEventsTreeSink<'r, 'p, T> {
    fn token(&mut self, kind: JsSyntaxKind, length: TextSize) {
        self.skip_trivia(false);

        let range = TextRange::at(self.offset, length);
        let new_kind = self.reparse.token(kind, self.parser);
        self.parser.push_token(new_kind, range);

        self.skip_trivia(true);
    }

    fn start_node(&mut self, kind: JsSyntaxKind) {
        // ISSUE: `complete` and `start()` use `cur_pos()` of the `tokens` source.
        self.reparse.start_node(kind, self.parser);
    }

    fn finish_node(&mut self) {
        self.reparse.finish_node(self.parser);
    }

    fn errors(&mut self, _errors: Vec<ParserError>) {}
}

/// Implement this trait if you want to change the tree structure
/// from already parsed events.
pub trait RewriteParseEvents {
    /// Called for a started node in the original tree
    fn start_node(&mut self, kind: JsSyntaxKind, p: &mut Parser);

    /// Called for a finished node in the original tree
    fn finish_node(&mut self, p: &mut Parser);

    /// Called for every token
    fn token(&mut self, kind: JsSyntaxKind, _p: &mut Parser) -> JsSyntaxKind {
        kind
    }

    /// Called for tokens spawning multiple lexer tokens
    fn multiple_token(&mut self, amount: u8, kind: JsSyntaxKind, p: &mut Parser) {
        p.bump_multiple(amount, kind)
    }
}

/// Allows rewriting a super grammar to a sub grammar by visiting each event emitted after the checkpoint.
/// Useful if a node turned out to be of a different kind its subtree must be re-shaped
/// (adding new nodes, dropping sub nodes, etc.).
pub fn rewrite_events<T: RewriteParseEvents>(
    rewriter: &mut T,
    checkpoint: Checkpoint,
    p: &mut Parser,
) {
    // Only rewind the events and token position but do not reset the parser errors nor parser state.
    // The current parsed grammar is a super-set of the grammar that gets re-parsed. Thus, any
    // error that applied to the old grammar also applies to the sub-grammar.
    let events: Vec<_> = p.events.split_off(checkpoint.event_pos + 1);

    // TODO: Ideally don't rewind. But then difficulty that `push_token` requires range
    let offset = checkpoint.token_source.offset();
    let trivia = checkpoint.token_source.trivia(&p.tokens).to_vec();

    let mut sink = RewriteParseEventsTreeSink {
        parser: p,
        reparse: rewriter,
        offset,
        trivia: &trivia,
    };
    process(&mut sink, events, Vec::default());
}
