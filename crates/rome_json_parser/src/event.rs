//! Events emitted by the Parser which are then constructed into a syntax tree

use std::mem;
use std::num::NonZeroU32;

// use crate::{ParseDiagnostic, Parser, TreeSink};
use rome_json_syntax::{JsonSyntaxKind::{self, *}, TextSize};
use rome_parse::{TreeSink, ParseDiagnostic};

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
        kind: JsonSyntaxKind,
        forward_parent: Option<NonZeroU32>,
    },

    /// Complete the previous `Start` event
    Finish,

    /// Produce a single leaf-element.
    Token {
        kind: JsonSyntaxKind,
        /// The end offset of this token.
        end: TextSize,
    },
}

impl Event {
    pub fn tombstone() -> Self {
        Event::Start {
            kind: TOMBSTONE,
            forward_parent: None,
        }
    }
}

/// Generate the syntax tree with the control of events.
#[inline]
pub fn process(sink: &mut impl TreeSink<Kind = JsonSyntaxKind>, mut events: Vec<Event>, errors: Vec<ParseDiagnostic>) {
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
                    fp = match mem::replace(&mut events[idx], Event::tombstone()) {
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
