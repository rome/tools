use crate::event::Event;
use crate::event::Event::Token;
use crate::token_source::TokenSource;
use crate::{LanguageParser, Parser};

use drop_bomb::DebugDropBomb;
use rome_rowan::{SyntaxKind, TextRange, TextSize};
use std::num::NonZeroU32;

/// A structure signifying the start of parsing of a syntax tree node
#[derive(Debug)]
#[must_use = "Marker must either be `completed` or `abandoned`"]
pub struct Marker {
    /// The index in the events list
    pos: u32,
    /// The byte index where the node starts
    start: TextSize,
    pub(crate) old_start: u32,
    child_idx: Option<usize>,
    bomb: DebugDropBomb,
}

impl Marker {
    pub fn new(pos: u32, start: TextSize) -> Marker {
        Marker {
            pos,
            start,
            old_start: pos,
            child_idx: None,
            bomb: DebugDropBomb::new("Marker must either be `completed` or `abandoned` to avoid that children are implicitly attached to a marker's parent."),
        }
    }

    fn old_start(mut self, old: u32) -> Self {
        if self.old_start >= old {
            self.old_start = old;
        };
        self
    }

    /// Finishes the syntax tree node and assigns `kind` to it,
    /// and mark the create a `CompletedMarker` for possible future
    /// operation like `.precede()` to deal with forward_parent.
    pub fn complete<L>(mut self, p: &mut Parser<L>, kind: L::Kind) -> CompletedMarker
    where
        L: LanguageParser,
    {
        self.bomb.defuse();
        let idx = self.pos as usize;
        match p.events[idx] {
            Event::Start {
                kind: ref mut slot, ..
            } => {
                *slot = kind;
            }
            _ => unreachable!(),
        }
        let finish_pos = p.events.len() as u32;
        p.push_event(Event::Finish);

        let new = CompletedMarker::new(self.pos, finish_pos, self.start);
        new.old_start(self.old_start)
    }

    /// Abandons the syntax tree node. All its children
    /// are attached to its parent instead.
    pub fn abandon<L>(mut self, p: &mut Parser<L>)
    where
        L: LanguageParser,
    {
        self.bomb.defuse();
        let idx = self.pos as usize;
        if idx == p.events.len() - 1 {
            if let Some(Event::Start {
                forward_parent: None,
                kind,
            }) = p.events.pop()
            {
                assert_eq!(kind, L::Kind::TOMBSTONE);
            }
        }
        if let Some(idx) = self.child_idx {
            match p.events[idx] {
                Event::Start {
                    ref mut forward_parent,
                    ..
                } => {
                    *forward_parent = None;
                }
                _ => unreachable!(),
            }
        }
    }

    pub fn start(&self) -> TextSize {
        self.start
    }
}

/// A structure signifying a completed node
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CompletedMarker {
    start_pos: u32,
    offset: TextSize,
    // Hack for parsing completed markers which have been preceded
    // This should be redone completely in the future
    old_start: u32,
    finish_pos: u32,
}

impl CompletedMarker {
    pub fn new(start_pos: u32, finish_pos: u32, offset: TextSize) -> Self {
        CompletedMarker {
            start_pos,
            offset,
            old_start: start_pos,
            finish_pos,
        }
    }

    pub(crate) fn old_start(mut self, old: u32) -> Self {
        // For multiple precedes we should not update the old start
        if self.old_start >= old {
            self.old_start = old;
        };
        self
    }

    /// Change the kind of node this marker represents
    pub fn change_kind<L>(&mut self, p: &mut Parser<L>, new_kind: L::Kind)
    where
        L: LanguageParser,
    {
        match p
            .events
            .get_mut(self.start_pos as usize)
            .expect("Finish position of marker is OOB")
        {
            Event::Start { kind, .. } => {
                *kind = new_kind;
            }
            _ => unreachable!(),
        }
    }

    pub fn change_to_unknown<L: LanguageParser>(&mut self, p: &mut Parser<L>)
    where
        L: LanguageParser,
    {
        self.change_kind(p, self.kind(p).to_unknown());
    }

    /// Get the range of the marker
    pub fn range<L>(&self, p: &Parser<L>) -> TextRange
    where
        L: LanguageParser,
    {
        let end = p.events[self.old_start as usize..self.finish_pos as usize]
            .iter()
            .rev()
            .find_map(|event| match event {
                Token { end, .. } => Some(*end),
                _ => None,
            })
            .unwrap_or(self.offset);

        TextRange::new(self.offset, end)
    }

    /// Get the underlying text of a marker
    pub fn text<'a, L>(&self, p: &'a Parser<L>) -> &'a str
    where
        L: LanguageParser,
    {
        &p.source.text()[self.range(p)]
    }

    /// This method allows to create a new node which starts
    /// *before* the current one. That is, parser could start
    /// node `A`, then complete it, and then after parsing the
    /// whole `A`, decide that it should have started some node
    /// `B` before starting `A`. `precede` allows to do exactly
    /// that. See also docs about `forward_parent` in `Event::Start`.
    ///
    /// Given completed events `[START, FINISH]` and its corresponding
    /// `CompletedMarker(pos: 0, _)`.
    /// Append a new `START` events as `[START, FINISH, NEWSTART]`,
    /// then mark `NEWSTART` as `START`'s parent with saving its relative
    /// distance to `NEWSTART` into forward_parent(=2 in this case);
    pub fn precede<L>(self, p: &mut Parser<L>) -> Marker
    where
        L: LanguageParser,
    {
        let mut new_pos = p.start();
        let idx = self.start_pos as usize;
        match p.events[idx] {
            Event::Start {
                ref mut forward_parent,
                ..
            } => {
                // Safety: The new marker is always inserted after the start marker of this node, thus
                // subtracting the two positions can never be 0.
                *forward_parent = Some(NonZeroU32::try_from(new_pos.pos - self.start_pos).unwrap());
            }
            _ => unreachable!(),
        }
        new_pos.child_idx = Some(self.start_pos as usize);
        new_pos.start = self.offset;
        new_pos.old_start(self.old_start as u32)
    }

    /// Undo this completion and turns into a `Marker`
    pub fn undo_completion<L>(self, p: &mut Parser<L>) -> Marker
    where
        L: LanguageParser,
    {
        let start_idx = self.start_pos as usize;
        let finish_idx = self.finish_pos as usize;

        match p.events[start_idx] {
            Event::Start {
                ref mut kind,
                forward_parent: None,
            } => *kind = L::Kind::TOMBSTONE,
            _ => unreachable!(),
        }
        match p.events[finish_idx] {
            ref mut slot @ Event::Finish { .. } => *slot = Event::tombstone(),
            _ => unreachable!(),
        }
        Marker::new(self.start_pos, self.offset)
    }

    pub fn kind<L>(&self, p: &Parser<L>) -> L::Kind
    where
        L: LanguageParser,
    {
        match p.events[self.start_pos as usize] {
            Event::Start { kind, .. } => kind,
            _ => unreachable!(),
        }
    }
}
