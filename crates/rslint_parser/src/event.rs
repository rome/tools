//! Events emitted by the Parser which are then constructed into a syntax tree

use std::{mem, ops::Range};

use crate::{
	JsSyntaxKind::{self, *},
	Parser, ParserError, TreeSink,
};

use crate::parser::Checkpoint;

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
		start: usize,
		forward_parent: Option<u32>,
	},

	/// Complete the previous `Start` event
	Finish {
		end: usize,
	},

	/// Produce a single leaf-element.
	/// `n_raw_tokens` is used to glue complex contextual tokens.
	/// For example, lexer tokenizes `>>` as `>`, `>`, and
	/// `n_raw_tokens = 2` is used to produced a single `>>`.
	Token {
		kind: JsSyntaxKind,
		range: Range<usize>,
	},

	MultipleTokens {
		amount: u8,
		kind: JsSyntaxKind,
	},
}

impl Event {
	pub fn tombstone(start: usize) -> Self {
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
		match mem::replace(&mut events[i], Event::tombstone(0)) {
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
					fp = match mem::replace(&mut events[idx], Event::tombstone(0)) {
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
			Event::Token { kind, .. } => {
				sink.token(kind);
			}
			Event::MultipleTokens { amount, kind } => sink.consume_multiple_tokens(amount, kind),
		}
	}
}

struct RewriteParseEventsTreeSink<'r, 'p, T> {
	reparse: &'r mut T,
	parser: &'r mut Parser<'p>,
}

impl<'r, 'p, T: RewriteParseEvents> TreeSink for RewriteParseEventsTreeSink<'r, 'p, T> {
	fn token(&mut self, kind: JsSyntaxKind) {
		self.reparse.token(kind, self.parser);
	}

	fn start_node(&mut self, kind: JsSyntaxKind) {
		self.reparse.start_node(kind, self.parser);
	}

	fn finish_node(&mut self) {
		self.reparse.finish_node(self.parser);
	}

	fn errors(&mut self, _errors: Vec<ParserError>) {}

	fn consume_multiple_tokens(&mut self, amount: u8, kind: JsSyntaxKind) {
		self.reparse.multiple_token(amount, kind, self.parser);
	}
}

/// Implement this trait if you want to change the tree structure
/// from already parsed events.
pub trait RewriteParseEvents {
	/// Called for a started node in the original tree
	fn start_node(&mut self, kind: JsSyntaxKind, p: &mut Parser);

	/// Called for a finished node in the original tree
	fn finish_node(&mut self, p: &mut Parser);

	/// Called for every token
	fn token(&mut self, kind: JsSyntaxKind, p: &mut Parser) {
		p.bump_remap(kind);
	}

	/// Called for tokens spawning multiple lexer tokens
	fn multiple_token(&mut self, amount: u8, kind: JsSyntaxKind, p: &mut Parser) {
		p.bump_multiple(amount, kind)
	}
}

/// Allows to rewrite the parse events by visiting each event emitted after the checkpoint.
/// Useful if a node turned out to be of a different kind its subtree must be re-shaped
/// (adding new nodes, dropping sub nodes, etc).
pub fn rewrite_events<T: RewriteParseEvents>(
	rewriter: &mut T,
	checkpoint: Checkpoint,
	p: &mut Parser,
) {
	let events: Vec<_> = p.events.split_off(checkpoint.event_pos + 1);
	p.rewind(checkpoint);

	let mut sink = RewriteParseEventsTreeSink {
		parser: p,
		reparse: rewriter,
	};
	process(&mut sink, events, Vec::default());
}
