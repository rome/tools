use crate::{cursor, Language, SyntaxToken};
use std::fmt;
use std::marker::PhantomData;
use text_size::{TextRange, TextSize};

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
    /// Token that the parser skipped for some reason.
    Skipped,
}

impl TriviaPieceKind {
    pub const fn is_newline(&self) -> bool {
        matches!(self, TriviaPieceKind::Newline)
    }

    pub const fn is_whitespace(&self) -> bool {
        matches!(self, TriviaPieceKind::Whitespace)
    }

    pub const fn is_single_line_comment(&self) -> bool {
        matches!(self, TriviaPieceKind::SingleLineComment)
    }

    pub const fn is_multiline_comment(&self) -> bool {
        matches!(self, TriviaPieceKind::MultiLineComment)
    }

    pub const fn is_skipped(&self) -> bool {
        matches!(self, TriviaPieceKind::Skipped)
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct TriviaPiece {
    pub(crate) kind: TriviaPieceKind,
    pub(crate) length: TextSize,
}

impl TriviaPiece {
    /// Creates a new whitespace trivia piece with the given length
    pub fn whitespace<L: Into<TextSize>>(len: L) -> Self {
        Self::new(TriviaPieceKind::Whitespace, len)
    }

    /// Creates a new newline trivia piece with the given text length
    pub fn newline<L: Into<TextSize>>(len: L) -> Self {
        Self::new(TriviaPieceKind::Newline, len)
    }

    /// Creates a new comment trivia piece that does not contain any line breaks.
    /// For example, JavaScript's `//` comments are guaranteed to not spawn multiple lines. However,
    /// this can also be a `/* ... */` comment if it doesn't contain any line break characters.
    pub fn single_line_comment<L: Into<TextSize>>(len: L) -> Self {
        Self::new(TriviaPieceKind::SingleLineComment, len)
    }

    /// Creates a new comment trivia piece that contains at least one line breaks.
    /// For example, a JavaScript `/* ... */` comment that spawns at least two lines (contains at least one line break character).
    pub fn multi_line_comment<L: Into<TextSize>>(len: L) -> Self {
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

#[derive(Debug, Clone)]
pub struct SyntaxTriviaPieceNewline<L: Language>(SyntaxTriviaPiece<L>);
#[derive(Debug, Clone)]
pub struct SyntaxTriviaPieceWhitespace<L: Language>(SyntaxTriviaPiece<L>);
#[derive(Debug, Clone)]
pub struct SyntaxTriviaPieceComments<L: Language>(SyntaxTriviaPiece<L>);
#[derive(Debug, Clone)]
pub struct SyntaxTriviaPieceSkipped<L: Language>(SyntaxTriviaPiece<L>);

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

    /// Returns a reference to its [SyntaxTriviaPiece]
    pub fn as_piece(&self) -> &SyntaxTriviaPiece<L> {
        &self.0
    }

    /// Returns its [SyntaxTriviaPiece]
    pub fn into_piece(self) -> SyntaxTriviaPiece<L> {
        self.0
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

    /// Returns a reference to its [SyntaxTriviaPiece]
    pub fn as_piece(&self) -> &SyntaxTriviaPiece<L> {
        &self.0
    }

    /// Returns its [SyntaxTriviaPiece]
    pub fn into_piece(self) -> SyntaxTriviaPiece<L> {
        self.0
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

    /// Returns a reference to its [SyntaxTriviaPiece]
    pub fn as_piece(&self) -> &SyntaxTriviaPiece<L> {
        &self.0
    }

    /// Returns its [SyntaxTriviaPiece]
    pub fn into_piece(self) -> SyntaxTriviaPiece<L> {
        self.0
    }
}

impl<L: Language> SyntaxTriviaPieceSkipped<L> {
    pub fn text(&self) -> &str {
        self.0.text()
    }

    pub fn text_len(&self) -> TextSize {
        self.0.text_len()
    }

    pub fn text_range(&self) -> TextRange {
        self.0.text_range()
    }

    /// Returns a reference to its [SyntaxTriviaPiece]
    pub fn as_piece(&self) -> &SyntaxTriviaPiece<L> {
        &self.0
    }

    /// Returns its [SyntaxTriviaPiece]
    pub fn into_piece(self) -> SyntaxTriviaPiece<L> {
        self.0
    }
}

/// [SyntaxTriviaPiece] gives access to the most granular information about the trivia
/// that was specified by the lexer at the token creation time.
///
/// For example:
///
/// ```no_test
/// builder.token_with_trivia(
///     RawSyntaxKind(1),
///     "\n\t /**/let \t\t",
///     &[TriviaPiece::whitespace(3), TriviaPiece::single_line_comment(4)],
///     &[TriviaPiece::whitespace(3)],
/// );
/// });
/// ```
/// This token has two pieces in the leading trivia, and one piece at the trailing trivia. Each
/// piece is defined by the [TriviaPiece]; its content is irrelevant.
///
#[derive(Clone)]
pub struct SyntaxTriviaPiece<L: Language> {
    raw: cursor::SyntaxTrivia,
    /// Absolute offset from the beginning of the file
    offset: TextSize,
    trivia: TriviaPiece,
    _p: PhantomData<L>,
}

impl<L: Language> SyntaxTriviaPiece<L> {
    /// Returns the internal kind of this trivia piece
    pub fn kind(&self) -> TriviaPieceKind {
        self.trivia.kind()
    }

    /// Returns the associated text just for this trivia piece. This is different from [SyntaxTrivia::text()],
    /// which returns the text of the whole trivia.
    ///
    /// ```
    /// use rome_rowan::raw_language::{RawLanguage, RawLanguageKind, RawSyntaxTreeBuilder};
    /// use rome_rowan::*;
    /// use std::iter::Iterator;
    /// let mut node = RawSyntaxTreeBuilder::wrap_with_node(RawLanguageKind::ROOT, |builder| {
    ///     builder.token_with_trivia(
    ///         RawLanguageKind::LET_TOKEN,
    ///         "\n\t /**/let \t\t",
    ///         &[
    ///             TriviaPiece::whitespace(3),
    ///             TriviaPiece::single_line_comment(4),
    ///         ],
    ///         &[TriviaPiece::whitespace(3)],
    ///     );
    /// });
    /// let leading: Vec<_> = node.first_leading_trivia().unwrap().pieces().collect();
    /// assert_eq!("\n\t ", leading[0].text());
    /// assert_eq!("/**/", leading[1].text());
    ///
    /// let trailing: Vec<_> = node.last_trailing_trivia().unwrap().pieces().collect();
    /// assert_eq!(" \t\t", trailing[0].text());
    /// ```
    pub fn text(&self) -> &str {
        let token = self.raw.token();
        let txt = token.text();

        // Compute the offset relative to the token
        let start = self.offset - token.text_range().start();
        let end = start + self.text_len();

        // Don't use self.raw.text(). It iterates over all pieces
        &txt[start.into()..end.into()]
    }

    /// Returns the associated text length just for this trivia piece. This is different from `SyntaxTrivia::len()`,
    /// which returns the text length of the whole trivia.
    ///
    /// ```
    /// use rome_rowan::raw_language::{RawLanguage, RawLanguageKind, RawSyntaxTreeBuilder};
    /// use rome_rowan::*;
    /// use std::iter::Iterator;
    /// let mut node = RawSyntaxTreeBuilder::wrap_with_node(RawLanguageKind::ROOT, |builder| {
    ///     builder.token_with_trivia(
    ///         RawLanguageKind::LET_TOKEN,
    ///         "\n\t /**/let \t\t",
    ///         &[
    ///             TriviaPiece::whitespace(3),
    ///             TriviaPiece::single_line_comment(4),
    ///         ],
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
    /// use rome_rowan::raw_language::{RawLanguage, RawLanguageKind, RawSyntaxTreeBuilder};
    /// use rome_rowan::*;
    /// use std::iter::Iterator;
    /// let mut node = RawSyntaxTreeBuilder::wrap_with_node(RawLanguageKind::ROOT, |builder| {
    ///     builder.token_with_trivia(
    ///         RawLanguageKind::LET_TOKEN,
    ///         "\n\t /**/let \t\t",
    ///         &[
    ///             TriviaPiece::whitespace(3),
    ///             TriviaPiece::single_line_comment(4),
    ///         ],
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
    /// use rome_rowan::raw_language::{RawLanguage, RawLanguageKind, RawSyntaxTreeBuilder};
    /// use rome_rowan::*;
    /// use std::iter::Iterator;
    /// let mut node = RawSyntaxTreeBuilder::wrap_with_node(RawLanguageKind::ROOT, |builder| {
    ///     builder.token_with_trivia(
    ///         RawLanguageKind::LET_TOKEN,
    ///         "\n\t/**/let",
    ///         &[
    ///             TriviaPiece::newline(1),
    ///             TriviaPiece::whitespace(1),
    ///             TriviaPiece::single_line_comment(4),
    ///         ],
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
    /// use rome_rowan::raw_language::{RawLanguage, RawLanguageKind, RawSyntaxTreeBuilder};
    /// use rome_rowan::*;
    /// use std::iter::Iterator;
    /// let mut node = RawSyntaxTreeBuilder::wrap_with_node(RawLanguageKind::ROOT, |builder| {
    ///     builder.token_with_trivia(
    ///         RawLanguageKind::LET_TOKEN,
    ///         "\n\t/**/let",
    ///         &[
    ///             TriviaPiece::newline(1),
    ///             TriviaPiece::whitespace(1),
    ///             TriviaPiece::single_line_comment(4),
    ///         ],
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
    /// use rome_rowan::raw_language::{RawLanguage, RawLanguageKind, RawSyntaxTreeBuilder};
    /// use rome_rowan::*;
    /// use std::iter::Iterator;
    /// let mut node = RawSyntaxTreeBuilder::wrap_with_node(RawLanguageKind::ROOT, |builder| {
    ///     builder.token_with_trivia(
    ///         RawLanguageKind::LET_TOKEN,
    ///         "\n\t/**/let",
    ///         &[
    ///             TriviaPiece::newline(1),
    ///             TriviaPiece::whitespace(1),
    ///             TriviaPiece::single_line_comment(4),
    ///         ],
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

    /// Returns true if this trivia piece is a [SyntaxTriviaPieceSkipped].
    pub fn is_skipped(&self) -> bool {
        self.trivia.kind.is_skipped()
    }

    /// Cast this trivia piece to [SyntaxTriviaPieceNewline].
    ///
    /// ```
    /// use rome_rowan::raw_language::{RawLanguage, RawLanguageKind, RawSyntaxTreeBuilder};
    /// use rome_rowan::*;
    /// use std::iter::Iterator;
    /// let mut node = RawSyntaxTreeBuilder::wrap_with_node(RawLanguageKind::ROOT, |builder| {
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
    /// use rome_rowan::raw_language::{RawLanguage, RawLanguageKind, RawSyntaxTreeBuilder};
    /// use rome_rowan::*;
    /// use std::iter::Iterator;
    /// let mut node = RawSyntaxTreeBuilder::wrap_with_node(RawLanguageKind::ROOT, |builder| {
    ///     builder.token_with_trivia(
    ///         RawLanguageKind::LET_TOKEN,
    ///         "\t /**/let \t\t",
    ///         &[
    ///             TriviaPiece::whitespace(2),
    ///             TriviaPiece::single_line_comment(4),
    ///         ],
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
    /// use rome_rowan::raw_language::{RawLanguage, RawLanguageKind, RawSyntaxTreeBuilder};
    /// use rome_rowan::*;
    /// use std::iter::Iterator;
    /// let mut node = RawSyntaxTreeBuilder::wrap_with_node(RawLanguageKind::ROOT, |builder| {
    ///     builder.token_with_trivia(
    ///         RawLanguageKind::LET_TOKEN,
    ///         "\n\t /**/let \t\t",
    ///         &[
    ///             TriviaPiece::whitespace(3),
    ///             TriviaPiece::single_line_comment(4),
    ///         ],
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

    /// Casts this piece to a skipped trivia piece.
    pub fn as_skipped(&self) -> Option<SyntaxTriviaPieceSkipped<L>> {
        match &self.trivia.kind {
            TriviaPieceKind::Skipped => Some(SyntaxTriviaPieceSkipped(self.clone())),
            _ => None,
        }
    }

    pub fn token(&self) -> SyntaxToken<L> {
        SyntaxToken::from(self.raw.token().clone())
    }
}

impl<L: Language> fmt::Debug for SyntaxTriviaPiece<L> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.trivia.kind {
            TriviaPieceKind::Newline => write!(f, "Newline(")?,
            TriviaPieceKind::Whitespace => write!(f, "Whitespace(")?,
            TriviaPieceKind::SingleLineComment | TriviaPieceKind::MultiLineComment => {
                write!(f, "Comments(")?
            }
            TriviaPieceKind::Skipped => write!(f, "Skipped(")?,
        }
        print_debug_str(self.text(), f)?;
        write!(f, ")")
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SyntaxTrivia<L: Language> {
    raw: cursor::SyntaxTrivia,
    _p: PhantomData<L>,
}

#[derive(Clone)]
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
    pub(super) fn new(raw: cursor::SyntaxTrivia) -> Self {
        Self {
            raw,
            _p: PhantomData,
        }
    }

    /// Returns all [SyntaxTriviaPiece] of this trivia.
    ///
    /// ```
    /// use crate::*;
    /// use rome_rowan::raw_language::{RawLanguage, RawLanguageKind, RawSyntaxTreeBuilder};
    /// use rome_rowan::*;
    /// use std::iter::Iterator;
    /// let mut node = RawSyntaxTreeBuilder::wrap_with_node(RawLanguageKind::ROOT, |builder| {
    ///     builder.token_with_trivia(
    ///         RawLanguageKind::LET_TOKEN,
    ///         "\n\t /**/let \t\t",
    ///         &[
    ///             TriviaPiece::whitespace(3),
    ///             TriviaPiece::single_line_comment(4),
    ///         ],
    ///         &[TriviaPiece::whitespace(3)],
    ///     );
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

    pub fn last(&self) -> Option<SyntaxTriviaPiece<L>> {
        let piece = self.raw.last()?;

        Some(SyntaxTriviaPiece {
            raw: self.raw.clone(),
            offset: self.raw.text_range().end() - piece.length,
            trivia: *piece,
            _p: Default::default(),
        })
    }

    pub fn first(&self) -> Option<SyntaxTriviaPiece<L>> {
        let piece = self.raw.first()?;

        Some(SyntaxTriviaPiece {
            raw: self.raw.clone(),
            offset: self.raw.text_range().end() - piece.length,
            trivia: *piece,
            _p: Default::default(),
        })
    }

    pub fn text(&self) -> &str {
        self.raw.text()
    }

    pub fn text_range(&self) -> TextRange {
        self.raw.text_range()
    }
}

fn print_debug_str<S: AsRef<str>>(text: S, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let text = text.as_ref();
    if text.len() < 25 {
        write!(f, "{:?}", text)
    } else {
        for idx in 21..25 {
            if text.is_char_boundary(idx) {
                let text = format!("{} ...", &text[..idx]);
                return write!(f, "{:?}", text);
            }
        }
        write!(f, "")
    }
}
