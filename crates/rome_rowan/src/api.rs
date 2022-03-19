use std::fmt::{Debug, Formatter};

use std::iter::FusedIterator;
use std::{fmt, iter, marker::PhantomData, ops::Range};

use crate::{
    cursor::{self},
    Direction, GreenNode, NodeOrToken, RawSyntaxKind, SyntaxText, TextRange, TextSize,
    TokenAtOffset, WalkEvent,
};

/// Type tag for each node or token of a language
pub trait SyntaxKind: fmt::Debug + PartialEq + Copy {
    /// Returns `true` if this is an unknown node kind.
    fn is_unknown(&self) -> bool;

    /// Converts this into to the best matching unknown node kind.
    fn to_unknown(&self) -> Self;

    /// Converts this kind to a raw syntax kind.
    fn to_raw(&self) -> RawSyntaxKind;

    /// Creates a syntax kind from a raw kind.
    fn from_raw(raw: RawSyntaxKind) -> Self;
}

pub trait Language: Sized + Clone + Copy + fmt::Debug + Eq + Ord + std::hash::Hash {
    type Kind: SyntaxKind;
}

#[derive(Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub enum TriviaPieceKind {
    /// A line break (`\n`, `\r`, `\r\n`, ...)
    Newline,
    /// Any whitespace character
    Whitespace,
    /// Comment that does not contain any line breaks
    SingleLineComment,
    /// Comment that contains at least one line break
    MultiLineComment,
}

impl TriviaPieceKind {
    pub fn is_newline(&self) -> bool {
        matches!(self, TriviaPieceKind::Newline)
    }

    pub fn is_whitespace(&self) -> bool {
        matches!(self, TriviaPieceKind::Whitespace)
    }

    pub fn is_single_line_comment(&self) -> bool {
        matches!(self, TriviaPieceKind::SingleLineComment)
    }

    pub fn is_multiline_comment(&self) -> bool {
        matches!(self, TriviaPieceKind::MultiLineComment)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct TriviaPiece {
    pub(crate) kind: TriviaPieceKind,
    pub(crate) length: TextSize,
}

impl TriviaPiece {
    /// Creates a new whitespace trivia piece with the given length
    pub fn whitespace<L: Into<TextSize>>(len: L) -> TriviaPiece {
        Self::new(TriviaPieceKind::Whitespace, len)
    }

    /// Creates a new newline trivia piece with the given text length
    pub fn newline<L: Into<TextSize>>(len: L) -> TriviaPiece {
        Self::new(TriviaPieceKind::Newline, len)
    }

    /// Creates a new comment trivia piece that does not contain any line breaks
    pub fn single_line_comment<L: Into<TextSize>>(len: L) -> TriviaPiece {
        Self::new(TriviaPieceKind::SingleLineComment, len)
    }

    /// Creates a new comment trivia piece that contains at least one line breaks
    pub fn multi_line_comment<L: Into<TextSize>>(len: L) -> TriviaPiece {
        Self::new(TriviaPieceKind::MultiLineComment, len)
    }

    pub fn new<L: Into<TextSize>>(kind: TriviaPieceKind, length: L) -> Self {
        Self {
            kind,
            length: length.into(),
        }
    }

    /// Returns the trivia's length
    pub fn text_len(&self) -> TextSize {
        self.length
    }

    /// Returns the trivia's kind
    pub fn kind(&self) -> TriviaPieceKind {
        self.kind
    }
}

pub struct SyntaxTriviaPieceNewline<L: Language>(SyntaxTriviaPiece<L>);
pub struct SyntaxTriviaPieceWhitespace<L: Language>(SyntaxTriviaPiece<L>);
pub struct SyntaxTriviaPieceComments<L: Language>(SyntaxTriviaPiece<L>);

impl<L: Language> SyntaxTriviaPieceNewline<L> {
    pub fn text(&self) -> &str {
        self.0.text()
    }

    pub fn text_len(&self) -> TextSize {
        self.0.text_len()
    }

    pub fn text_range(&self) -> TextRange {
        self.0.text_range()
    }
}

impl<L: Language> SyntaxTriviaPieceWhitespace<L> {
    pub fn text(&self) -> &str {
        self.0.text()
    }

    pub fn text_len(&self) -> TextSize {
        self.0.text_len()
    }

    pub fn text_range(&self) -> TextRange {
        self.0.text_range()
    }
}

impl<L: Language> SyntaxTriviaPieceComments<L> {
    pub fn text(&self) -> &str {
        self.0.text()
    }

    pub fn text_len(&self) -> TextSize {
        self.0.text_len()
    }

    pub fn text_range(&self) -> TextRange {
        self.0.text_range()
    }

    pub fn has_newline(&self) -> bool {
        self.0.trivia.kind.is_multiline_comment()
    }
}

/// [SyntaxTriviaPiece] gives access to the most granular information about the trivia
/// that was specified by the lexer at the token creation time.
///
/// For example:
///
/// ```ignore
/// builder.token_with_trivia(
///     RawSyntaxKind(1),
///     "\n\t /**/let \t\t",
///     &[TriviaPiece::whitespace(3), TriviaPiece::single_line_comment(4)],
///     &[TriviaPiece::whitespace(3)],
/// );
/// });
///
/// This token has two pieces in the leading trivia, and one piece at the trailing trivia. Each
/// piece is defined by the [TriviaPiece]; its content is irrelevant.
/// ```
#[derive(Clone)]
pub struct SyntaxTriviaPiece<L: Language> {
    raw: cursor::SyntaxTrivia,
    offset: TextSize,
    trivia: TriviaPiece,
    _p: PhantomData<L>,
}

impl<L: Language> SyntaxTriviaPiece<L> {
    /// Returns the associated text just for this trivia piece. This is different from [SyntaxTrivia::text()],
    /// which returns the text of the whole trivia.
    ///
    /// ```
    /// use rome_rowan::*;
    /// use rome_rowan::raw_language::{RawLanguage, RawLanguageKind, RawSyntaxTreeBuilder};
    /// use std::iter::Iterator;
    /// let mut node = RawSyntaxTreeBuilder::wrap_with_node(RawLanguageKind::ROOT,|builder| {
    ///     builder.token_with_trivia(
    ///         RawLanguageKind::LET_TOKEN,
    ///         "\n\t /**/let \t\t",
    ///         &[TriviaPiece::whitespace(3), TriviaPiece::single_line_comment(4)],
    ///         &[TriviaPiece::whitespace(3)],
    ///     );
    /// });
    /// let pieces: Vec<_> = node.first_leading_trivia().unwrap().pieces().collect();
    /// assert_eq!("\n\t ", pieces[0].text());
    /// ```
    pub fn text(&self) -> &str {
        let txt = self.raw.text();
        let start = self.offset - self.raw.offset();
        let end = start + self.text_len();

        &txt[start.into()..end.into()]
    }

    /// Returns the associated text length just for this trivia piece. This is different from [SyntaxTrivia::text_len()],
    /// which returns the text length of the whole trivia.
    ///
    /// ```
    /// use rome_rowan::*;
    /// use rome_rowan::raw_language::{RawLanguage, RawLanguageKind, RawSyntaxTreeBuilder};
    /// use std::iter::Iterator;
    /// let mut node = RawSyntaxTreeBuilder::wrap_with_node(RawLanguageKind::ROOT,|builder| {
    ///     builder.token_with_trivia(
    ///         RawLanguageKind::LET_TOKEN,
    ///         "\n\t /**/let \t\t",
    ///         &[TriviaPiece::whitespace(3), TriviaPiece::single_line_comment(4)],
    ///         &[TriviaPiece::whitespace(3)],
    ///     );
    /// });
    /// let pieces: Vec<_> = node.first_leading_trivia().unwrap().pieces().collect();
    /// assert_eq!(TextSize::from(3), pieces[0].text_len());
    /// ```
    pub fn text_len(&self) -> TextSize {
        self.trivia.text_len()
    }

    /// Returns the associated text range just for this trivia piece. This is different from [SyntaxTrivia::text_range()],
    /// which returns the text range of the whole trivia.
    ///
    /// ```
    /// use rome_rowan::*;
    /// use rome_rowan::raw_language::{RawLanguage, RawLanguageKind, RawSyntaxTreeBuilder};
    /// use std::iter::Iterator;
    /// let mut node = RawSyntaxTreeBuilder::wrap_with_node(RawLanguageKind::ROOT,|builder| {
    ///     builder.token_with_trivia(
    ///         RawLanguageKind::LET_TOKEN,
    ///         "\n\t /**/let \t\t",
    ///         &[TriviaPiece::whitespace(3), TriviaPiece::single_line_comment(4)],
    ///         &[TriviaPiece::whitespace(3)],
    ///     );
    /// });
    /// let pieces: Vec<_> = node.first_leading_trivia().unwrap().pieces().collect();
    /// assert_eq!(TextRange::new(0.into(), 3.into()), pieces[0].text_range());
    /// ```
    pub fn text_range(&self) -> TextRange {
        TextRange::at(self.offset, self.text_len())
    }

    /// Returns true if this trivia piece is a [SyntaxTriviaPieceNewline].
    ///
    /// ```
    /// use rome_rowan::*;
    /// use rome_rowan::raw_language::{RawLanguage, RawLanguageKind, RawSyntaxTreeBuilder};
    /// use std::iter::Iterator;
    /// let mut node = RawSyntaxTreeBuilder::wrap_with_node(RawLanguageKind::ROOT,|builder| {
    ///     builder.token_with_trivia(
    ///         RawLanguageKind::LET_TOKEN,
    ///         "\n\t/**/let",
    ///         &[TriviaPiece::newline(1), TriviaPiece::whitespace(1), TriviaPiece::single_line_comment(4)],
    ///         &[],
    ///     );
    /// });
    /// let pieces: Vec<_> = node.first_leading_trivia().unwrap().pieces().collect();
    /// assert!(pieces[0].is_newline())
    /// ```
    pub fn is_newline(&self) -> bool {
        self.trivia.kind.is_newline()
    }

    /// Returns true if this trivia piece is a [SyntaxTriviaPieceWhitespace].
    ///
    /// ```
    /// use rome_rowan::*;
    /// use rome_rowan::raw_language::{RawLanguage, RawLanguageKind, RawSyntaxTreeBuilder};
    /// use std::iter::Iterator;
    /// let mut node = RawSyntaxTreeBuilder::wrap_with_node(RawLanguageKind::ROOT,|builder| {
    ///     builder.token_with_trivia(
    ///         RawLanguageKind::LET_TOKEN,
    ///         "\n\t/**/let",
    ///         &[TriviaPiece::newline(1), TriviaPiece::whitespace(1), TriviaPiece::single_line_comment(4)],
    ///         &[],
    ///     );
    /// });
    /// let pieces: Vec<_> = node.first_leading_trivia().unwrap().pieces().collect();
    /// assert!(pieces[1].is_whitespace())
    /// ```
    pub fn is_whitespace(&self) -> bool {
        self.trivia.kind.is_whitespace()
    }

    /// Returns true if this trivia piece is a [SyntaxTriviaPieceComments].
    ///
    /// ```
    /// use rome_rowan::*;
    /// use rome_rowan::raw_language::{RawLanguage, RawLanguageKind, RawSyntaxTreeBuilder};
    /// use std::iter::Iterator;
    /// let mut node = RawSyntaxTreeBuilder::wrap_with_node(RawLanguageKind::ROOT,|builder| {
    ///     builder.token_with_trivia(
    ///         RawLanguageKind::LET_TOKEN,
    ///         "\n\t/**/let",
    ///         &[TriviaPiece::newline(1), TriviaPiece::whitespace(1), TriviaPiece::single_line_comment(4)],
    ///         &[],
    ///     );
    /// });
    /// let pieces: Vec<_> = node.first_leading_trivia().unwrap().pieces().collect();
    /// assert!(pieces[2].is_comments())
    /// ```
    pub fn is_comments(&self) -> bool {
        matches!(
            self.trivia.kind,
            TriviaPieceKind::SingleLineComment | TriviaPieceKind::MultiLineComment
        )
    }

    /// Cast this trivia piece to [SyntaxTriviaPieceNewline].
    ///
    /// ```
    /// use rome_rowan::*;
    /// use rome_rowan::raw_language::{RawLanguage, RawLanguageKind, RawSyntaxTreeBuilder};
    /// use std::iter::Iterator;
    /// let mut node = RawSyntaxTreeBuilder::wrap_with_node(RawLanguageKind::ROOT,|builder| {
    ///     builder.token_with_trivia(
    ///         RawLanguageKind::LET_TOKEN,
    ///         "\n/**/let \t\t",
    ///         &[TriviaPiece::newline(1), TriviaPiece::single_line_comment(4)],
    ///         &[TriviaPiece::newline(3)],
    ///     );
    /// });
    /// let pieces: Vec<_> = node.first_leading_trivia().unwrap().pieces().collect();
    /// let w = pieces[0].as_newline();
    /// assert!(w.is_some());
    /// let w = pieces[1].as_newline();
    /// assert!(w.is_none());
    /// ```
    pub fn as_newline(&self) -> Option<SyntaxTriviaPieceNewline<L>> {
        match &self.trivia.kind {
            TriviaPieceKind::Newline => Some(SyntaxTriviaPieceNewline(self.clone())),
            _ => None,
        }
    }

    /// Cast this trivia piece to [SyntaxTriviaPieceWhitespace].
    ///
    /// ```
    /// use rome_rowan::*;
    /// use rome_rowan::raw_language::{RawLanguage, RawLanguageKind, RawSyntaxTreeBuilder};
    /// use std::iter::Iterator;
    /// let mut node = RawSyntaxTreeBuilder::wrap_with_node(RawLanguageKind::ROOT,|builder| {
    ///     builder.token_with_trivia(
    ///         RawLanguageKind::LET_TOKEN,
    ///         "\t /**/let \t\t",
    ///         &[TriviaPiece::whitespace(2), TriviaPiece::single_line_comment(4)],
    ///         &[TriviaPiece::whitespace(3)],
    ///     );
    /// });
    /// let pieces: Vec<_> = node.first_leading_trivia().unwrap().pieces().collect();
    /// let w = pieces[0].as_whitespace();
    /// assert!(w.is_some());
    /// let w = pieces[1].as_whitespace();
    /// assert!(w.is_none());
    /// ```
    pub fn as_whitespace(&self) -> Option<SyntaxTriviaPieceWhitespace<L>> {
        match &self.trivia.kind {
            TriviaPieceKind::Whitespace => Some(SyntaxTriviaPieceWhitespace(self.clone())),
            _ => None,
        }
    }

    /// Cast this trivia piece to [SyntaxTriviaPieceComments].
    ///
    /// ```
    /// use rome_rowan::*;
    /// use rome_rowan::raw_language::{RawLanguage, RawLanguageKind, RawSyntaxTreeBuilder};
    /// use std::iter::Iterator;
    /// let mut node = RawSyntaxTreeBuilder::wrap_with_node(RawLanguageKind::ROOT,|builder| {
    ///     builder.token_with_trivia(
    ///         RawLanguageKind::LET_TOKEN,
    ///         "\n\t /**/let \t\t",
    ///         &[TriviaPiece::whitespace(3), TriviaPiece::single_line_comment(4)],
    ///         &[TriviaPiece::whitespace(3)],
    ///     );
    /// });
    /// let pieces: Vec<_> = node.first_leading_trivia().unwrap().pieces().collect();
    /// let w = pieces[0].as_comments();
    /// assert!(w.is_none());
    /// let w = pieces[1].as_comments();
    /// assert!(w.is_some());
    /// ```
    pub fn as_comments(&self) -> Option<SyntaxTriviaPieceComments<L>> {
        match &self.trivia.kind {
            TriviaPieceKind::SingleLineComment | TriviaPieceKind::MultiLineComment => {
                Some(SyntaxTriviaPieceComments(self.clone()))
            }
            _ => None,
        }
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SyntaxTrivia<L: Language> {
    raw: cursor::SyntaxTrivia,
    _p: PhantomData<L>,
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SyntaxNode<L: Language> {
    raw: cursor::SyntaxNode,
    _p: PhantomData<L>,
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SyntaxToken<L: Language> {
    raw: cursor::SyntaxToken,
    _p: PhantomData<L>,
}

pub type SyntaxElement<L> = NodeOrToken<SyntaxNode<L>, SyntaxToken<L>>;

impl<L: Language> fmt::Debug for SyntaxNode<L> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if f.alternate() {
            let mut level = 0;
            for event in self.raw.preorder_slots() {
                match event {
                    WalkEvent::Enter(element) => {
                        for _ in 0..level {
                            write!(f, "  ")?;
                        }
                        match element {
                            cursor::SyntaxSlot::Node(node) => {
                                writeln!(f, "{}: {:?}", node.index(), SyntaxNode::<L>::from(node))?
                            }
                            cursor::SyntaxSlot::Token(token) => writeln!(
                                f,
                                "{}: {:?}",
                                token.index(),
                                SyntaxToken::<L>::from(token)
                            )?,
                            cursor::SyntaxSlot::Empty { index, .. } => {
                                writeln!(f, "{}: (empty)", index)?
                            }
                        }
                        level += 1;
                    }
                    WalkEvent::Leave(_) => level -= 1,
                }
            }
            assert_eq!(level, 0);
            Ok(())
        } else {
            write!(f, "{:?}@{:?}", self.kind(), self.text_range())
        }
    }
}

impl<L: Language> fmt::Display for SyntaxNode<L> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.raw, f)
    }
}

fn print_debug_str<S: AsRef<str>>(text: S, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let text = text.as_ref();
    return if text.len() < 25 {
        write!(f, "{:?}", text)
    } else {
        for idx in 21..25 {
            if text.is_char_boundary(idx) {
                let text = format!("{} ...", &text[..idx]);
                return write!(f, "{:?}", text);
            }
        }
        write!(f, "")
    };
}

fn print_debug_trivia_piece<L: Language>(
    piece: SyntaxTriviaPiece<L>,
    f: &mut fmt::Formatter<'_>,
) -> fmt::Result {
    match piece.trivia.kind {
        TriviaPieceKind::Newline => write!(f, "Newline(")?,
        TriviaPieceKind::Whitespace => write!(f, "Whitespace(")?,
        TriviaPieceKind::SingleLineComment | TriviaPieceKind::MultiLineComment => {
            write!(f, "Comments(")?
        }
    }
    print_debug_str(piece.text(), f)?;
    write!(f, ")")
}

impl<L: Language> fmt::Debug for SyntaxToken<L> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:?}@{:?} {:?} ",
            self.kind(),
            self.text_range(),
            self.text_trimmed()
        )?;

        write!(f, "[")?;
        let mut first_piece = true;
        for piece in self.leading_trivia().pieces() {
            if !first_piece {
                write!(f, ", ")?;
            }
            first_piece = false;
            print_debug_trivia_piece(piece, f)?;
        }
        write!(f, "] [")?;

        let mut first_piece = true;
        for piece in self.trailing_trivia().pieces() {
            if !first_piece {
                write!(f, ", ")?;
            }
            first_piece = false;
            print_debug_trivia_piece(piece, f)?;
        }
        write!(f, "]")
    }
}

impl<L: Language> fmt::Display for SyntaxToken<L> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.raw, f)
    }
}

impl<L: Language> From<SyntaxNode<L>> for SyntaxElement<L> {
    fn from(node: SyntaxNode<L>) -> SyntaxElement<L> {
        NodeOrToken::Node(node)
    }
}

impl<L: Language> From<SyntaxToken<L>> for SyntaxElement<L> {
    fn from(token: SyntaxToken<L>) -> SyntaxElement<L> {
        NodeOrToken::Token(token)
    }
}

pub struct SyntaxTriviaPiecesIterator<L: Language> {
    iter: cursor::SyntaxTriviaPiecesIterator,
    _p: PhantomData<L>,
}

impl<L: Language> Iterator for SyntaxTriviaPiecesIterator<L> {
    type Item = SyntaxTriviaPiece<L>;

    fn next(&mut self) -> Option<Self::Item> {
        let (offset, trivia) = self.iter.next()?;
        Some(SyntaxTriviaPiece {
            raw: self.iter.raw.clone(),
            offset,
            trivia,
            _p: PhantomData,
        })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<L: Language> DoubleEndedIterator for SyntaxTriviaPiecesIterator<L> {
    fn next_back(&mut self) -> Option<Self::Item> {
        let (offset, trivia) = self.iter.next_back()?;
        Some(SyntaxTriviaPiece {
            raw: self.iter.raw.clone(),
            offset,
            trivia,
            _p: PhantomData,
        })
    }
}

impl<L: Language> ExactSizeIterator for SyntaxTriviaPiecesIterator<L> {}

impl<L: Language> SyntaxTrivia<L> {
    /// Returns all [SyntaxTriviaPiece] of this trivia.
    ///
    /// ```
    /// use rome_rowan::*;
    /// use rome_rowan::raw_language::{RawLanguage, RawLanguageKind, RawSyntaxTreeBuilder};
    /// use std::iter::Iterator;
    /// use crate::*;
    /// let mut node = RawSyntaxTreeBuilder::wrap_with_node(RawLanguageKind::ROOT,|builder| {
    /// builder.token_with_trivia(
    ///     RawLanguageKind::LET_TOKEN,
    ///     "\n\t /**/let \t\t",
    ///     &[TriviaPiece::whitespace(3), TriviaPiece::single_line_comment(4)],
    ///     &[TriviaPiece::whitespace(3)],
    /// );
    /// });
    /// let pieces: Vec<_> = node.first_leading_trivia().unwrap().pieces().collect();
    /// assert_eq!(2, pieces.len());
    /// let pieces: Vec<_> = node.last_trailing_trivia().unwrap().pieces().collect();
    /// assert_eq!(1, pieces.len());
    /// ```
    pub fn pieces(&self) -> SyntaxTriviaPiecesIterator<L> {
        SyntaxTriviaPiecesIterator {
            iter: self.raw.pieces(),
            _p: PhantomData,
        }
    }

    pub fn text(&self) -> &str {
        self.raw.text()
    }

    pub fn text_range(&self) -> TextRange {
        self.raw.text_range()
    }
}

impl<L: Language> SyntaxNode<L> {
    pub(crate) fn new_root(green: GreenNode) -> SyntaxNode<L> {
        SyntaxNode::from(cursor::SyntaxNode::new_root(green))
    }

    /// Returns the element stored in the slot with the given index. Returns [None] if the slot is empty.
    ///
    /// ## Panics
    /// If the slot index is out of bounds
    pub fn element_in_slot(&self, slot: u32) -> Option<SyntaxElement<L>> {
        self.raw.element_in_slot(slot).map(SyntaxElement::from)
    }

    pub fn kind(&self) -> L::Kind {
        L::Kind::from_raw(self.raw.kind())
    }

    /// Returns the text of all descendants tokens combined, including all trivia.
    ///
    /// ```
    /// use rome_rowan::*;
    /// use rome_rowan::raw_language::{RawLanguage, RawLanguageKind, RawSyntaxTreeBuilder};
    /// let mut node = RawSyntaxTreeBuilder::wrap_with_node(RawLanguageKind::ROOT,|builder| {
    ///     builder.token_with_trivia(
    ///         RawLanguageKind::LET_TOKEN,
    ///         "\n\t let \t\t",
    ///         &[TriviaPiece::whitespace(3)],
    ///         &[TriviaPiece::whitespace(3)],
    ///     );
    ///     builder.token(RawLanguageKind::STRING_TOKEN, "a");
    ///     builder.token_with_trivia(
    ///         RawLanguageKind::SEMICOLON_TOKEN,
    ///         "; \t\t",
    ///         &[TriviaPiece::whitespace(3)],
    ///         &[TriviaPiece::whitespace(3)],
    ///     );
    /// });
    /// assert_eq!("\n\t let \t\ta; \t\t", node.text());
    /// ```
    pub fn text(&self) -> SyntaxText {
        self.raw.text()
    }

    /// Returns the text of all descendants tokens combined,
    /// excluding the first token leading trivia, and the last token trailing trivia.
    /// All other trivia is included.
    ///
    /// ```
    /// use rome_rowan::*;
    /// use rome_rowan::raw_language::{RawLanguage, RawLanguageKind, RawSyntaxTreeBuilder};
    /// let mut node = RawSyntaxTreeBuilder::wrap_with_node(RawLanguageKind::ROOT,|builder| {
    ///     builder.token_with_trivia(
    ///         RawLanguageKind::LET_TOKEN,
    ///         "\n\t let \t\t",
    ///         &[TriviaPiece::whitespace(3)],
    ///         &[TriviaPiece::whitespace(3)],
    ///     );
    ///     builder.token(RawLanguageKind::STRING_TOKEN, "a");
    ///     builder.token_with_trivia(
    ///         RawLanguageKind::SEMICOLON_TOKEN,
    ///         "; \t\t",
    ///         &[],
    ///         &[TriviaPiece::whitespace(3)],
    ///     );
    /// });
    /// assert_eq!("let \t\ta;", node.text_trimmed());
    /// ```
    pub fn text_trimmed(&self) -> SyntaxText {
        self.raw.text_trimmed()
    }

    /// Returns the range corresponding for the text of all descendants tokens combined, including all trivia.
    ///
    /// ```
    /// use rome_rowan::*;
    /// use rome_rowan::raw_language::{RawLanguage, RawLanguageKind, RawSyntaxTreeBuilder};
    /// let mut node = RawSyntaxTreeBuilder::wrap_with_node(RawLanguageKind::ROOT,|builder| {
    ///     builder.token_with_trivia(
    ///         RawLanguageKind::LET_TOKEN,
    ///         "\n\t let \t\t",
    ///         &[TriviaPiece::whitespace(3)],
    ///         &[TriviaPiece::whitespace(3)],
    ///     );
    ///     builder.token(RawLanguageKind::STRING_TOKEN, "a");
    ///     builder.token_with_trivia(
    ///         RawLanguageKind::SEMICOLON_TOKEN,
    ///         "; \t\t",
    ///         &[],
    ///         &[TriviaPiece::whitespace(3)],
    ///     );
    /// });
    /// let range = node.text_range();
    /// assert_eq!(0u32, range.start().into());
    /// assert_eq!(14u32, range.end().into());
    /// ```
    pub fn text_range(&self) -> TextRange {
        self.raw.text_range()
    }

    /// Returns the range corresponding for the text of all descendants tokens combined,
    /// excluding the first token leading  trivia, and the last token trailing trivia.
    /// All other trivia is included.
    ///
    /// ```
    /// use rome_rowan::*;
    /// use rome_rowan::raw_language::{RawLanguage, RawLanguageKind, RawSyntaxTreeBuilder};
    /// let mut node = RawSyntaxTreeBuilder::wrap_with_node(RawLanguageKind::ROOT,|builder| {
    ///     builder.token_with_trivia(
    ///         RawLanguageKind::LET_TOKEN,
    ///         "\n\t let \t\t",
    ///         &[TriviaPiece::whitespace(3)],
    ///         &[TriviaPiece::whitespace(3)],
    ///     );
    ///     builder.token(RawLanguageKind::STRING_TOKEN, "a");
    ///     builder.token_with_trivia(
    ///         RawLanguageKind::SEMICOLON_TOKEN,
    ///         "; \t\t",
    ///         &[],
    ///         &[TriviaPiece::whitespace(3)],
    ///     );
    /// });
    /// let range = node.text_trimmed_range();
    /// assert_eq!(3u32, range.start().into());
    /// assert_eq!(11u32, range.end().into());
    /// ```
    pub fn text_trimmed_range(&self) -> TextRange {
        self.raw.text_trimmed_range()
    }

    /// Returns the leading trivia of the [first_token](SyntaxNode::first_token), or [None] if the node does not have any descendant tokens.
    ///
    /// ```
    /// use rome_rowan::*;
    /// use rome_rowan::raw_language::{RawLanguage, RawLanguageKind, RawSyntaxTreeBuilder};
    /// let mut node = RawSyntaxTreeBuilder::wrap_with_node(RawLanguageKind::ROOT,|builder| {
    ///     builder.token_with_trivia(
    ///         RawLanguageKind::LET_TOKEN,
    ///         "\n\t let \t\t",
    ///         &[TriviaPiece::whitespace(3)],
    ///         &[TriviaPiece::whitespace(3)],
    ///     );
    ///     builder.token(RawLanguageKind::STRING_TOKEN, "a");
    ///     builder.token_with_trivia(
    ///         RawLanguageKind::SEMICOLON_TOKEN,
    ///         "; \t\t",
    ///         &[],
    ///         &[TriviaPiece::whitespace(3)],
    ///     );
    /// });
    /// let trivia = node.first_leading_trivia();
    /// assert!(trivia.is_some());
    /// assert_eq!("\n\t ", trivia.unwrap().text());
    ///
    /// let mut node = RawSyntaxTreeBuilder::wrap_with_node(RawLanguageKind::ROOT,|builder| {});
    /// let trivia = node.first_leading_trivia();
    /// assert!(trivia.is_none());
    /// ```
    pub fn first_leading_trivia(&self) -> Option<SyntaxTrivia<L>> {
        self.raw.first_leading_trivia().map(|raw| SyntaxTrivia {
            raw,
            _p: PhantomData,
        })
    }

    /// Returns the trailing trivia of the  [last_token](SyntaxNode::last_token), or [None] if the node does not have any descendant tokens.
    ///
    /// ```
    /// use rome_rowan::*;
    /// use rome_rowan::raw_language::{RawLanguage, RawLanguageKind, RawSyntaxTreeBuilder};
    /// let mut node = RawSyntaxTreeBuilder::wrap_with_node(RawLanguageKind::ROOT,|builder| {
    ///     builder.token_with_trivia(
    ///         RawLanguageKind::LET_TOKEN,
    ///         "\n\t let \t\t",
    ///         &[TriviaPiece::whitespace(3)],
    ///         &[TriviaPiece::whitespace(3)],
    ///     );
    ///     builder.token(RawLanguageKind::STRING_TOKEN, "a");
    ///     builder.token_with_trivia(
    ///         RawLanguageKind::SEMICOLON_TOKEN,
    ///         "; \t\t",
    ///         &[],
    ///         &[TriviaPiece::whitespace(3)],
    ///     );
    /// });
    /// let trivia = node.last_trailing_trivia();
    /// assert!(trivia.is_some());
    /// assert_eq!(" \t\t", trivia.unwrap().text());
    ///
    /// let mut node = RawSyntaxTreeBuilder::wrap_with_node(RawLanguageKind::ROOT,|builder| {});
    /// let trivia = node.last_trailing_trivia();
    /// assert!(trivia.is_none());
    /// ```
    pub fn last_trailing_trivia(&self) -> Option<SyntaxTrivia<L>> {
        self.raw.last_trailing_trivia().map(|raw| SyntaxTrivia {
            raw,
            _p: PhantomData,
        })
    }

    pub fn parent(&self) -> Option<SyntaxNode<L>> {
        self.raw.parent().map(Self::from)
    }

    pub fn ancestors(&self) -> impl Iterator<Item = SyntaxNode<L>> {
        self.raw.ancestors().map(SyntaxNode::from)
    }

    pub fn children(&self) -> SyntaxNodeChildren<L> {
        SyntaxNodeChildren {
            raw: self.raw.children(),
            _p: PhantomData,
        }
    }

    /// Returns an iterator over all the slots of this syntax node.
    pub fn slots(&self) -> SyntaxSlots<L> {
        SyntaxSlots {
            raw: self.raw.slots(),
            _p: PhantomData,
        }
    }

    pub fn children_with_tokens(&self) -> SyntaxElementChildren<L> {
        SyntaxElementChildren {
            raw: self.raw.children_with_tokens(),
            _p: PhantomData,
        }
    }

    pub fn first_child(&self) -> Option<SyntaxNode<L>> {
        self.raw.first_child().map(Self::from)
    }
    pub fn last_child(&self) -> Option<SyntaxNode<L>> {
        self.raw.last_child().map(Self::from)
    }

    pub fn first_child_or_token(&self) -> Option<SyntaxElement<L>> {
        self.raw.first_child_or_token().map(NodeOrToken::from)
    }
    pub fn last_child_or_token(&self) -> Option<SyntaxElement<L>> {
        self.raw.last_child_or_token().map(NodeOrToken::from)
    }

    pub fn next_sibling(&self) -> Option<SyntaxNode<L>> {
        self.raw.next_sibling().map(Self::from)
    }
    pub fn prev_sibling(&self) -> Option<SyntaxNode<L>> {
        self.raw.prev_sibling().map(Self::from)
    }

    pub fn next_sibling_or_token(&self) -> Option<SyntaxElement<L>> {
        self.raw.next_sibling_or_token().map(NodeOrToken::from)
    }
    pub fn prev_sibling_or_token(&self) -> Option<SyntaxElement<L>> {
        self.raw.prev_sibling_or_token().map(NodeOrToken::from)
    }

    /// Return the leftmost token in the subtree of this node.
    pub fn first_token(&self) -> Option<SyntaxToken<L>> {
        self.raw.first_token().map(SyntaxToken::from)
    }
    /// Return the rightmost token in the subtree of this node.
    pub fn last_token(&self) -> Option<SyntaxToken<L>> {
        self.raw.last_token().map(SyntaxToken::from)
    }

    pub fn siblings(&self, direction: Direction) -> impl Iterator<Item = SyntaxNode<L>> {
        self.raw.siblings(direction).map(SyntaxNode::from)
    }

    pub fn siblings_with_tokens(
        &self,
        direction: Direction,
    ) -> impl Iterator<Item = SyntaxElement<L>> {
        self.raw
            .siblings_with_tokens(direction)
            .map(SyntaxElement::from)
    }

    pub fn descendants(&self) -> impl Iterator<Item = SyntaxNode<L>> {
        self.raw.descendants().map(SyntaxNode::from)
    }

    pub fn descendants_tokens(&self) -> impl Iterator<Item = SyntaxToken<L>> {
        self.descendants_with_tokens()
            .filter_map(|x| x.as_token().cloned())
    }

    pub fn descendants_with_tokens(&self) -> impl Iterator<Item = SyntaxElement<L>> {
        self.raw.descendants_with_tokens().map(NodeOrToken::from)
    }

    /// Traverse the subtree rooted at the current node (including the current
    /// node) in preorder, excluding tokens.
    pub fn preorder(&self) -> Preorder<L> {
        Preorder {
            raw: self.raw.preorder(),
            _p: PhantomData,
        }
    }

    /// Traverse the subtree rooted at the current node (including the current
    /// node) in preorder, including tokens.
    pub fn preorder_with_tokens(&self) -> PreorderWithTokens<L> {
        PreorderWithTokens {
            raw: self.raw.preorder_with_tokens(),
            _p: PhantomData,
        }
    }

    /// Find a token in the subtree corresponding to this node, which covers the offset.
    /// Precondition: offset must be withing node's range.
    pub fn token_at_offset(&self, offset: TextSize) -> TokenAtOffset<SyntaxToken<L>> {
        self.raw.token_at_offset(offset).map(SyntaxToken::from)
    }

    /// Return the deepest node or token in the current subtree that fully
    /// contains the range. If the range is empty and is contained in two leaf
    /// nodes, either one can be returned. Precondition: range must be contained
    /// withing the current node
    pub fn covering_element(&self, range: TextRange) -> SyntaxElement<L> {
        NodeOrToken::from(self.raw.covering_element(range))
    }

    /// Finds a [`SyntaxElement`] which intersects with a given `range`. If
    /// there are several intersecting elements, any one can be returned.
    ///
    /// The method uses binary search internally, so it's complexity is
    /// `O(log(N))` where `N = self.children_with_tokens().count()`.
    pub fn child_or_token_at_range(&self, range: TextRange) -> Option<SyntaxElement<L>> {
        self.raw
            .child_or_token_at_range(range)
            .map(SyntaxElement::from)
    }

    /// Returns an independent copy of the subtree rooted at this node.
    ///
    /// The parent of the returned node will be `None`, the start offset will be
    /// zero, but, otherwise, it'll be equivalent to the source node.
    pub fn clone_subtree(&self) -> SyntaxNode<L> {
        SyntaxNode::from(self.raw.clone_subtree())
    }

    pub fn clone_for_update(&self) -> SyntaxNode<L> {
        SyntaxNode::from(self.raw.clone_for_update())
    }

    pub fn detach(&self) {
        self.raw.detach()
    }

    pub fn splice_children(&self, to_delete: Range<usize>, to_insert: Vec<SyntaxElement<L>>) {
        let to_insert = to_insert
            .into_iter()
            .map(cursor::SyntaxElement::from)
            .collect::<Vec<_>>();
        self.raw.splice_children(to_delete, to_insert)
    }

    pub fn into_list(self) -> SyntaxList<L> {
        SyntaxList::new(self)
    }
}

impl<L: Language> SyntaxToken<L> {
    pub fn kind(&self) -> L::Kind {
        L::Kind::from_raw(self.raw.kind())
    }

    pub fn text_range(&self) -> TextRange {
        self.raw.text_range()
    }

    pub fn text_trimmed_range(&self) -> TextRange {
        self.raw.text_trimmed_range()
    }

    pub fn index(&self) -> usize {
        self.raw.index()
    }

    /// Returns the text of the token, including all trivia.
    ///
    /// ```
    /// use rome_rowan::*;
    /// use rome_rowan::raw_language::{RawLanguage, RawLanguageKind, RawSyntaxTreeBuilder};
    /// let mut token = RawSyntaxTreeBuilder::wrap_with_node(RawLanguageKind::ROOT,|builder| {
    ///     builder.token_with_trivia(
    ///         RawLanguageKind::LET_TOKEN,
    ///         "\n\t let \t\t",
    ///         &[TriviaPiece::whitespace(3)],
    ///         &[TriviaPiece::whitespace(3)],
    ///     );
    /// }).first_token().unwrap();
    /// assert_eq!("\n\t let \t\t", token.text());
    /// ```
    pub fn text(&self) -> &str {
        self.raw.text()
    }

    /// Returns the text of the token, excluding all trivia.
    ///
    /// ```
    /// use rome_rowan::*;
    /// use rome_rowan::raw_language::{RawLanguage, RawLanguageKind, RawSyntaxTreeBuilder};
    /// let mut token = RawSyntaxTreeBuilder::wrap_with_node(RawLanguageKind::ROOT,|builder| {
    ///     builder.token_with_trivia(
    ///         RawLanguageKind::LET_TOKEN,
    ///         "\n\t let \t\t",
    ///         &[TriviaPiece::whitespace(3)],
    ///         &[TriviaPiece::whitespace(3)],
    ///     );
    /// }).first_token().unwrap();
    /// assert_eq!("let", token.text_trimmed());
    /// ```
    pub fn text_trimmed(&self) -> &str {
        self.raw.text_trimmed()
    }

    pub fn parent(&self) -> Option<SyntaxNode<L>> {
        self.raw.parent().map(SyntaxNode::from)
    }

    pub fn ancestors(&self) -> impl Iterator<Item = SyntaxNode<L>> {
        self.raw.ancestors().map(SyntaxNode::from)
    }

    pub fn next_sibling_or_token(&self) -> Option<SyntaxElement<L>> {
        self.raw.next_sibling_or_token().map(NodeOrToken::from)
    }
    pub fn prev_sibling_or_token(&self) -> Option<SyntaxElement<L>> {
        self.raw.prev_sibling_or_token().map(NodeOrToken::from)
    }

    pub fn siblings_with_tokens(
        &self,
        direction: Direction,
    ) -> impl Iterator<Item = SyntaxElement<L>> {
        self.raw
            .siblings_with_tokens(direction)
            .map(SyntaxElement::from)
    }

    /// Next token in the tree (i.e, not necessary a sibling).
    pub fn next_token(&self) -> Option<SyntaxToken<L>> {
        self.raw.next_token().map(SyntaxToken::from)
    }
    /// Previous token in the tree (i.e, not necessary a sibling).
    pub fn prev_token(&self) -> Option<SyntaxToken<L>> {
        self.raw.prev_token().map(SyntaxToken::from)
    }

    pub fn detach(&self) {
        self.raw.detach()
    }

    /// Returns the token leading trivia.
    ///
    /// ```
    /// use rome_rowan::*;
    /// use rome_rowan::raw_language::{RawLanguage, RawLanguageKind, RawSyntaxTreeBuilder};
    /// let mut token = RawSyntaxTreeBuilder::wrap_with_node(RawLanguageKind::ROOT,|builder| {
    ///     builder.token_with_trivia(
    ///         RawLanguageKind::LET_TOKEN,
    ///         "\n\t let \t\t",
    ///         &[TriviaPiece::whitespace(3)],
    ///         &[TriviaPiece::whitespace(3)],
    ///     );
    /// }).first_token().unwrap();
    /// assert_eq!("\n\t ", token.leading_trivia().text());
    /// ```
    #[inline]
    pub fn leading_trivia(&self) -> SyntaxTrivia<L> {
        SyntaxTrivia {
            raw: self.raw.leading_trivia(),
            _p: PhantomData,
        }
    }

    /// Returns the token trailing trivia.
    ///
    /// ```
    /// use rome_rowan::*;
    /// use rome_rowan::raw_language::{RawLanguage, RawLanguageKind, RawSyntaxTreeBuilder};
    /// let mut token = RawSyntaxTreeBuilder::wrap_with_node(RawLanguageKind::ROOT,|builder| {
    ///     builder.token_with_trivia(
    ///         RawLanguageKind::LET_TOKEN,
    ///         "\n\t let \t\t",
    ///         &[TriviaPiece::whitespace(3)],
    ///         &[TriviaPiece::whitespace(3)],
    ///     );
    /// }).first_token().unwrap();
    /// assert_eq!(" \t\t", token.trailing_trivia().text());
    /// ```
    #[inline]
    pub fn trailing_trivia(&self) -> SyntaxTrivia<L> {
        SyntaxTrivia {
            raw: self.raw.trailing_trivia(),
            _p: PhantomData,
        }
    }

    /// Checks if the current token has trailing comments
    pub fn has_trailing_comments(&self) -> bool {
        self.trailing_trivia()
            .pieces()
            .into_iter()
            .any(|piece| piece.is_comments())
    }

    /// Checks if the current token has leading comments
    pub fn has_leading_comments(&self) -> bool {
        self.leading_trivia()
            .pieces()
            .into_iter()
            .any(|piece| piece.is_comments())
    }
}

impl<L: Language> SyntaxElement<L> {
    pub fn text_range(&self) -> TextRange {
        match self {
            NodeOrToken::Node(it) => it.text_range(),
            NodeOrToken::Token(it) => it.text_range(),
        }
    }

    pub fn text_trimmed_range(&self) -> TextRange {
        match self {
            NodeOrToken::Node(it) => it.text_trimmed_range(),
            NodeOrToken::Token(it) => it.text_trimmed_range(),
        }
    }

    pub fn leading_trivia(&self) -> Option<SyntaxTrivia<L>> {
        match self {
            NodeOrToken::Node(it) => it.first_leading_trivia(),
            NodeOrToken::Token(it) => Some(it.leading_trivia()),
        }
    }

    pub fn trailing_trivia(&self) -> Option<SyntaxTrivia<L>> {
        match self {
            NodeOrToken::Node(it) => it.last_trailing_trivia(),
            NodeOrToken::Token(it) => Some(it.trailing_trivia()),
        }
    }

    pub fn kind(&self) -> L::Kind {
        match self {
            NodeOrToken::Node(it) => it.kind(),
            NodeOrToken::Token(it) => it.kind(),
        }
    }

    pub fn parent(&self) -> Option<SyntaxNode<L>> {
        match self {
            NodeOrToken::Node(it) => it.parent(),
            NodeOrToken::Token(it) => it.parent(),
        }
    }

    pub fn ancestors(&self) -> impl Iterator<Item = SyntaxNode<L>> {
        let first = match self {
            NodeOrToken::Node(it) => Some(it.clone()),
            NodeOrToken::Token(it) => it.parent(),
        };
        iter::successors(first, SyntaxNode::parent)
    }

    pub fn next_sibling_or_token(&self) -> Option<SyntaxElement<L>> {
        match self {
            NodeOrToken::Node(it) => it.next_sibling_or_token(),
            NodeOrToken::Token(it) => it.next_sibling_or_token(),
        }
    }

    pub fn prev_sibling_or_token(&self) -> Option<SyntaxElement<L>> {
        match self {
            NodeOrToken::Node(it) => it.prev_sibling_or_token(),
            NodeOrToken::Token(it) => it.prev_sibling_or_token(),
        }
    }

    pub fn detach(&self) {
        match self {
            NodeOrToken::Node(it) => it.detach(),
            NodeOrToken::Token(it) => it.detach(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct SyntaxNodeChildren<L: Language> {
    raw: cursor::SyntaxNodeChildren,
    _p: PhantomData<L>,
}

impl<L: Language> Iterator for SyntaxNodeChildren<L> {
    type Item = SyntaxNode<L>;
    fn next(&mut self) -> Option<Self::Item> {
        self.raw.next().map(SyntaxNode::from)
    }
}

#[derive(Clone)]
pub struct SyntaxElementChildren<L: Language> {
    raw: cursor::SyntaxElementChildren,
    _p: PhantomData<L>,
}

impl<L: Language> Debug for SyntaxElementChildren<L> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.clone()).finish()
    }
}

impl<L: Language> Default for SyntaxElementChildren<L> {
    fn default() -> Self {
        SyntaxElementChildren {
            raw: cursor::SyntaxElementChildren::default(),
            _p: PhantomData,
        }
    }
}

impl<L: Language> Iterator for SyntaxElementChildren<L> {
    type Item = SyntaxElement<L>;
    fn next(&mut self) -> Option<Self::Item> {
        self.raw.next().map(NodeOrToken::from)
    }
}

pub struct Preorder<L: Language> {
    raw: cursor::Preorder,
    _p: PhantomData<L>,
}

impl<L: Language> Preorder<L> {
    pub fn skip_subtree(&mut self) {
        self.raw.skip_subtree()
    }
}

impl<L: Language> Iterator for Preorder<L> {
    type Item = WalkEvent<SyntaxNode<L>>;
    fn next(&mut self) -> Option<Self::Item> {
        self.raw.next().map(|it| it.map(SyntaxNode::from))
    }
}

pub struct PreorderWithTokens<L: Language> {
    raw: cursor::PreorderWithTokens,
    _p: PhantomData<L>,
}

impl<L: Language> PreorderWithTokens<L> {
    pub fn skip_subtree(&mut self) {
        self.raw.skip_subtree()
    }
}

impl<L: Language> Iterator for PreorderWithTokens<L> {
    type Item = WalkEvent<SyntaxElement<L>>;
    fn next(&mut self) -> Option<Self::Item> {
        self.raw.next().map(|it| it.map(SyntaxElement::from))
    }
}

impl<L: Language> From<cursor::SyntaxNode> for SyntaxNode<L> {
    fn from(raw: cursor::SyntaxNode) -> SyntaxNode<L> {
        SyntaxNode {
            raw,
            _p: PhantomData,
        }
    }
}

impl<L: Language> From<SyntaxNode<L>> for cursor::SyntaxNode {
    fn from(node: SyntaxNode<L>) -> cursor::SyntaxNode {
        node.raw
    }
}

impl<L: Language> From<cursor::SyntaxToken> for SyntaxToken<L> {
    fn from(raw: cursor::SyntaxToken) -> SyntaxToken<L> {
        SyntaxToken {
            raw,
            _p: PhantomData,
        }
    }
}

impl<L: Language> From<SyntaxToken<L>> for cursor::SyntaxToken {
    fn from(token: SyntaxToken<L>) -> cursor::SyntaxToken {
        token.raw
    }
}

impl<L: Language> From<cursor::SyntaxElement> for SyntaxElement<L> {
    fn from(raw: cursor::SyntaxElement) -> SyntaxElement<L> {
        match raw {
            NodeOrToken::Node(it) => NodeOrToken::Node(it.into()),
            NodeOrToken::Token(it) => NodeOrToken::Token(it.into()),
        }
    }
}

impl<L: Language> From<SyntaxElement<L>> for cursor::SyntaxElement {
    fn from(element: SyntaxElement<L>) -> cursor::SyntaxElement {
        match element {
            NodeOrToken::Node(it) => NodeOrToken::Node(it.into()),
            NodeOrToken::Token(it) => NodeOrToken::Token(it.into()),
        }
    }
}

/// Each node has a slot for each of its children regardless if the child is present or not.
/// A child that isn't present either because it's optional or because of a syntax error
/// is stored in an [SyntaxSlot::Empty] to preserve the index of each child.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum SyntaxSlot<L: Language> {
    /// Slot that stores a node child
    Node(SyntaxNode<L>),
    /// Slot that stores a token child
    Token(SyntaxToken<L>),
    /// Slot that marks that the child in this position isn't present in the source code.
    Empty,
}

impl<L: Language> SyntaxSlot<L> {
    pub fn into_node(self) -> Option<SyntaxNode<L>> {
        match self {
            SyntaxSlot::Node(node) => Some(node),
            _ => None,
        }
    }

    pub fn into_token(self) -> Option<SyntaxToken<L>> {
        match self {
            SyntaxSlot::Token(token) => Some(token),
            _ => None,
        }
    }

    pub fn kind(&self) -> Option<L::Kind> {
        match self {
            SyntaxSlot::Node(node) => Some(node.kind()),
            SyntaxSlot::Token(token) => Some(token.kind()),
            SyntaxSlot::Empty => None,
        }
    }
}

impl<L: Language> From<cursor::SyntaxSlot> for SyntaxSlot<L> {
    fn from(raw: cursor::SyntaxSlot) -> Self {
        match raw {
            cursor::SyntaxSlot::Node(node) => SyntaxSlot::Node(node.into()),
            cursor::SyntaxSlot::Token(token) => SyntaxSlot::Token(token.into()),
            cursor::SyntaxSlot::Empty { .. } => SyntaxSlot::Empty,
        }
    }
}

/// Iterator over the slots of a node.
#[derive(Debug, Clone)]
pub struct SyntaxSlots<L> {
    raw: cursor::SyntaxSlots,
    _p: PhantomData<L>,
}

impl<L: Language> Iterator for SyntaxSlots<L> {
    type Item = SyntaxSlot<L>;

    fn next(&mut self) -> Option<Self::Item> {
        self.raw.next().map(SyntaxSlot::from)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        self.raw.size_hint()
    }

    fn last(self) -> Option<Self::Item>
    where
        Self: Sized,
    {
        self.raw.last().map(SyntaxSlot::from)
    }

    fn nth(&mut self, n: usize) -> Option<Self::Item> {
        self.raw.nth(n).map(SyntaxSlot::from)
    }
}

impl<'a, L: Language> FusedIterator for SyntaxSlots<L> {}

impl<'a, L: Language> ExactSizeIterator for SyntaxSlots<L> {
    fn len(&self) -> usize {
        self.raw.len()
    }
}

/// A list of `SyntaxNode`s and/or `SyntaxToken`s
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct SyntaxList<L: Language> {
    list: SyntaxNode<L>,
}

impl<L: Language> SyntaxList<L> {
    /// Creates a new list wrapping a List `SyntaxNode`
    fn new(node: SyntaxNode<L>) -> Self {
        Self { list: node }
    }

    /// Iterates over the elements in the list.
    pub fn iter(&self) -> SyntaxSlots<L> {
        self.list.slots()
    }

    /// Returns the number of items in this list
    pub fn len(&self) -> usize {
        self.list.slots().len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn first(&self) -> Option<SyntaxSlot<L>> {
        self.list.slots().next()
    }

    pub fn last(&self) -> Option<SyntaxSlot<L>> {
        self.list.slots().last()
    }

    pub fn node(&self) -> &SyntaxNode<L> {
        &self.list
    }
}

impl<L: Language> IntoIterator for &SyntaxList<L> {
    type Item = SyntaxSlot<L>;
    type IntoIter = SyntaxSlots<L>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<L: Language> IntoIterator for SyntaxList<L> {
    type Item = SyntaxSlot<L>;
    type IntoIter = SyntaxSlots<L>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

#[cfg(test)]
mod tests {
    use text_size::TextRange;

    use crate::api::TriviaPiece;
    use crate::raw_language::{RawLanguageKind, RawSyntaxTreeBuilder};
    use crate::Direction;

    #[test]
    fn empty_list() {
        let mut builder: RawSyntaxTreeBuilder = RawSyntaxTreeBuilder::new();
        builder.start_node(RawLanguageKind::EXPRESSION_LIST);
        builder.finish_node();
        let list = builder.finish().into_list();

        assert!(list.is_empty());
        assert_eq!(list.len(), 0);

        assert_eq!(list.first(), None);
        assert_eq!(list.last(), None);

        assert_eq!(list.iter().collect::<Vec<_>>(), Vec::default());
    }

    #[test]
    fn node_list() {
        let mut builder = RawSyntaxTreeBuilder::new();

        builder.start_node(RawLanguageKind::EXPRESSION_LIST);

        builder.start_node(RawLanguageKind::LITERAL_EXPRESSION);
        builder.token(RawLanguageKind::NUMBER_TOKEN, "1");
        builder.finish_node();

        builder.start_node(RawLanguageKind::LITERAL_EXPRESSION);
        builder.token(RawLanguageKind::NUMBER_TOKEN, "2");
        builder.finish_node();

        builder.finish_node();

        let node = builder.finish();
        let list = node.into_list();

        assert!(!list.is_empty());
        assert_eq!(list.len(), 2);

        let first = list.first().and_then(|e| e.into_node()).unwrap();
        assert_eq!(first.kind(), RawLanguageKind::LITERAL_EXPRESSION);
        assert_eq!(first.text(), "1");

        let last = list.last().and_then(|e| e.into_node()).unwrap();
        assert_eq!(last.kind(), RawLanguageKind::LITERAL_EXPRESSION);
        assert_eq!(last.text(), "2");

        let node_texts: Vec<_> = list
            .iter()
            .map(|e| e.into_node().map(|n| n.text().to_string()))
            .collect();

        assert_eq!(
            node_texts,
            vec![Some(String::from("1")), Some(String::from("2"))]
        )
    }

    #[test]
    fn node_or_token_list() {
        let mut builder = RawSyntaxTreeBuilder::new();

        builder.start_node(RawLanguageKind::SEPARATED_EXPRESSION_LIST);

        builder.start_node(RawLanguageKind::LITERAL_EXPRESSION);
        builder.token(RawLanguageKind::NUMBER_TOKEN, "1");
        builder.finish_node();

        builder.token(RawLanguageKind::NUMBER_TOKEN, ",");

        builder.start_node(RawLanguageKind::LITERAL_EXPRESSION);
        builder.token(RawLanguageKind::NUMBER_TOKEN, "2");
        builder.finish_node();

        builder.finish_node();

        let node = builder.finish();
        let list = node.into_list();

        assert!(!list.is_empty());
        assert_eq!(list.len(), 3);

        let first = list.first().and_then(|e| e.into_node()).unwrap();
        assert_eq!(first.kind(), RawLanguageKind::LITERAL_EXPRESSION);
        assert_eq!(first.text(), "1");

        let last = list.last().and_then(|e| e.into_node()).unwrap();
        assert_eq!(last.kind(), RawLanguageKind::LITERAL_EXPRESSION);
        assert_eq!(last.text(), "2");

        let kinds: Vec<_> = list.iter().map(|e| e.kind()).collect();

        assert_eq!(
            kinds,
            vec![
                Some(RawLanguageKind::LITERAL_EXPRESSION),
                Some(RawLanguageKind::NUMBER_TOKEN),
                Some(RawLanguageKind::LITERAL_EXPRESSION)
            ]
        )
    }

    #[test]
    fn siblings() {
        let mut builder = RawSyntaxTreeBuilder::new();

        // list
        builder.start_node(RawLanguageKind::SEPARATED_EXPRESSION_LIST);

        // element 1
        builder.start_node(RawLanguageKind::LITERAL_EXPRESSION);
        builder.token(RawLanguageKind::NUMBER_TOKEN, "a");
        builder.finish_node();

        // element 2
        builder.start_node(RawLanguageKind::LITERAL_EXPRESSION);
        builder.token(RawLanguageKind::NUMBER_TOKEN, "b");
        builder.finish_node();

        // Missing ,

        // element 3
        builder.start_node(RawLanguageKind::LITERAL_EXPRESSION);
        builder.token(RawLanguageKind::NUMBER_TOKEN, "c");
        builder.finish_node();

        builder.finish_node();

        let root = builder.finish();

        let first = root.children().next().unwrap();
        assert_eq!(first.text().to_string(), "a");
        assert_eq!(
            first.next_sibling().map(|e| e.text().to_string()),
            Some(String::from("b"))
        );

        let second = root.children().nth(1).unwrap();
        assert_eq!(second.text().to_string(), "b");

        // Skips the missing element
        assert_eq!(
            second.next_sibling().map(|e| e.text().to_string()),
            Some(String::from("c"))
        );

        assert_eq!(
            second.prev_sibling().map(|e| e.text().to_string()),
            Some(String::from("a"))
        );

        let last = root.children().last().unwrap();
        assert_eq!(last.text(), "c");
        assert_eq!(last.next_sibling(), None);
        assert_eq!(
            last.prev_sibling().map(|e| e.text().to_string()),
            Some(String::from("b"))
        );

        assert_eq!(
            first
                .siblings(Direction::Next)
                .map(|s| s.text().to_string())
                .collect::<Vec<_>>(),
            vec!["a", "b", "c"]
        );

        assert_eq!(
            last.siblings(Direction::Prev)
                .map(|s| s.text().to_string())
                .collect::<Vec<_>>(),
            vec!["c", "b", "a"]
        );
    }

    #[test]
    fn siblings_with_tokens() {
        let mut builder = RawSyntaxTreeBuilder::new();

        builder.start_node(RawLanguageKind::ROOT);

        builder.token(RawLanguageKind::FOR_KW, "for");
        builder.token(RawLanguageKind::L_PAREN_TOKEN, "(");
        builder.token(RawLanguageKind::SEMICOLON_TOKEN, ";");

        builder.start_node(RawLanguageKind::LITERAL_EXPRESSION);
        builder.token(RawLanguageKind::STRING_TOKEN, "x");
        builder.finish_node();

        builder.token(RawLanguageKind::SEMICOLON_TOKEN, ";");
        builder.token(RawLanguageKind::R_PAREN_TOKEN, ")");

        builder.finish_node();

        let root = builder.finish();

        let first_semicolon = root
            .children_with_tokens()
            .nth(2)
            .and_then(|e| e.into_token())
            .unwrap();

        assert_eq!(first_semicolon.text(), ";");

        assert_eq!(
            first_semicolon
                .siblings_with_tokens(Direction::Next)
                .map(|e| e.to_string())
                .collect::<Vec<_>>(),
            vec!["x", ";", ")"]
        );

        assert_eq!(
            first_semicolon.next_sibling_or_token(),
            first_semicolon.siblings_with_tokens(Direction::Next).next()
        );
        assert_eq!(
            first_semicolon.prev_sibling_or_token(),
            first_semicolon.siblings_with_tokens(Direction::Prev).next()
        );
    }

    #[test]
    pub fn syntax_text_and_len() {
        let mut builder = RawSyntaxTreeBuilder::new();
        builder.start_node(RawLanguageKind::ROOT);
        builder.token_with_trivia(
            RawLanguageKind::LET_TOKEN,
            "\n\t let \t\t",
            &[TriviaPiece::whitespace(3)],
            &[TriviaPiece::whitespace(3)],
        );
        builder.finish_node();

        // // Node texts

        let node = builder.finish();
        assert_eq!("\n\t let \t\t", node.text());
        assert_eq!("let", node.text_trimmed());
        assert_eq!("\n\t ", node.first_leading_trivia().unwrap().text());
        assert_eq!(" \t\t", node.last_trailing_trivia().unwrap().text());

        // Token texts

        let token = node.first_token().unwrap();
        assert_eq!("\n\t let \t\t", token.text());
        assert_eq!("let", token.text_trimmed());
        assert_eq!("\n\t ", token.leading_trivia().text());
        assert_eq!(" \t\t", token.trailing_trivia().text());
    }

    #[test]
    pub fn syntax_range() {
        let mut builder = RawSyntaxTreeBuilder::new();
        builder.start_node(RawLanguageKind::ROOT);
        builder.token_with_trivia(
            RawLanguageKind::LET_TOKEN,
            "\n\t let \t\t",
            &[TriviaPiece::whitespace(3)],
            &[TriviaPiece::whitespace(3)],
        );
        builder.token_with_trivia(
            RawLanguageKind::LET_TOKEN,
            "a ",
            &[TriviaPiece::whitespace(0)],
            &[TriviaPiece::whitespace(1)],
        );
        builder.token_with_trivia(
            RawLanguageKind::EQUAL_TOKEN,
            "\n=\n",
            &[TriviaPiece::whitespace(1)],
            &[TriviaPiece::whitespace(1)],
        );
        builder.token(RawLanguageKind::NUMBER_TOKEN, "1");
        builder.token_with_trivia(
            RawLanguageKind::SEMICOLON_TOKEN,
            ";\t\t",
            &[],
            &[TriviaPiece::whitespace(2)],
        );
        builder.finish_node();

        let node = builder.finish();

        // Node Ranges

        assert_eq!(TextRange::new(0.into(), 18.into()), node.text_range());
        assert_eq!(
            TextRange::new(3.into(), 16.into()),
            node.text_trimmed_range()
        );
        assert_eq!(
            TextRange::new(0.into(), 3.into()),
            node.first_leading_trivia().unwrap().text_range()
        );
        assert_eq!(
            TextRange::new(16.into(), 18.into()),
            node.last_trailing_trivia().unwrap().text_range()
        );

        // as NodeOrToken

        let eq_token = node
            .descendants_with_tokens()
            .find(|x| x.kind() == RawLanguageKind::EQUAL_TOKEN)
            .unwrap();

        assert_eq!(TextRange::new(11.into(), 14.into()), eq_token.text_range());
        assert_eq!(
            TextRange::new(12.into(), 13.into()),
            eq_token.text_trimmed_range()
        );
        assert_eq!(
            TextRange::new(11.into(), 12.into()),
            eq_token.leading_trivia().unwrap().text_range()
        );
        assert_eq!(
            TextRange::new(13.into(), 14.into()),
            eq_token.trailing_trivia().unwrap().text_range()
        );

        // as Token

        let eq_token = eq_token.as_token().unwrap();
        assert_eq!(TextRange::new(11.into(), 14.into()), eq_token.text_range());
        assert_eq!(
            TextRange::new(12.into(), 13.into()),
            eq_token.text_trimmed_range()
        );
        assert_eq!(
            TextRange::new(11.into(), 12.into()),
            eq_token.leading_trivia().text_range()
        );
        assert_eq!(
            TextRange::new(13.into(), 14.into()),
            eq_token.trailing_trivia().text_range()
        );
    }

    #[test]
    pub fn syntax_trivia_pieces() {
        use crate::*;
        let node = RawSyntaxTreeBuilder::wrap_with_node(RawLanguageKind::ROOT, |builder| {
            builder.token_with_trivia(
                RawLanguageKind::LET_TOKEN,
                "\n\t /**/let \t\t",
                &[
                    TriviaPiece::whitespace(3),
                    TriviaPiece::single_line_comment(4),
                ],
                &[TriviaPiece::whitespace(3)],
            );
        });

        let pieces: Vec<_> = node.first_leading_trivia().unwrap().pieces().collect();
        assert_eq!(2, pieces.len());

        assert_eq!("\n\t ", pieces[0].text());
        assert_eq!(TextSize::from(3), pieces[0].text_len());
        assert_eq!(TextRange::new(0.into(), 3.into()), pieces[0].text_range());
        assert!(pieces[0].is_whitespace());

        assert_eq!("/**/", pieces[1].text());
        assert_eq!(TextSize::from(4), pieces[1].text_len());
        assert_eq!(TextRange::new(3.into(), 7.into()), pieces[1].text_range());
        assert!(pieces[1].is_comments());

        let pieces_rev: Vec<_> = node
            .first_leading_trivia()
            .unwrap()
            .pieces()
            .rev()
            .collect();

        assert_eq!(2, pieces_rev.len());
        assert_eq!("/**/", pieces_rev[0].text());
        assert_eq!("\n\t ", pieces_rev[1].text());
    }
}
