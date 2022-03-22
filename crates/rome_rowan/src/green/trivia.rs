use crate::arc::{HeaderSlice, ThinArc};
use crate::TriviaPiece;
use countme::Count;
use std::fmt;
use std::fmt::Formatter;
use text_size::TextSize;

#[derive(PartialEq, Eq, Hash)]
pub(crate) struct GreenTriviaHead {
    _c: Count<GreenTrivia>,
}

type ReprThin = HeaderSlice<GreenTriviaHead, [TriviaPiece; 0]>;

#[repr(transparent)]
pub(crate) struct GreenTriviaData {
    data: ReprThin,
}

impl GreenTriviaData {
    #[allow(unused)]
    #[inline]
    pub fn header(&self) -> &GreenTriviaHead {
        &self.data.header
    }

    #[inline]
    pub fn pieces(&self) -> &[TriviaPiece] {
        self.data.slice()
    }
}

impl PartialEq for GreenTriviaData {
    fn eq(&self, other: &Self) -> bool {
        self.pieces() == other.pieces()
    }
}

impl fmt::Debug for GreenTriviaData {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.pieces().iter()).finish()
    }
}

/// List of trivia. Used to store either the leading or trailing trivia of a token.
/// The identity of a trivia is defined by the kinds and lengths of its items but not by
/// the texts of an individual piece. That means, that `\r` and `\n` can both be represented
/// by the same trivia, a trivia with a single `LINEBREAK` piece with the length 1.
/// This is safe because the text is stored on the token to which the trivia belongs and
/// `a\n` and `a\r` never resolve to the same tokens. Thus, they only share the trivia but are
/// otherwise two different tokens.
#[derive(Eq, PartialEq, Hash, Clone)]
#[repr(transparent)]
pub(crate) struct GreenTrivia {
    ptr: Option<ThinArc<GreenTriviaHead, TriviaPiece>>,
}

impl fmt::Debug for GreenTrivia {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self.pieces(), f)
    }
}

impl GreenTrivia {
    /// Creates a new trivia containing the passed in pieces
    pub fn new<I>(pieces: I) -> Self
    where
        I: IntoIterator<Item = TriviaPiece>,
        I::IntoIter: ExactSizeIterator,
    {
        let data =
            ThinArc::from_header_and_iter(GreenTriviaHead { _c: Count::new() }, pieces.into_iter());

        GreenTrivia { ptr: Some(data) }
    }

    /// Creates an empty trivia
    pub fn empty() -> Self {
        GreenTrivia { ptr: None }
    }

    /// Returns the total length of all pieces
    pub fn text_len(&self) -> TextSize {
        let mut len: Option<TextSize> = Some(TextSize::default());

        for piece in self.pieces() {
            len = len.and_then(|len| len.checked_add(piece.length))
        }

        // Realistically we will never have files bigger than usize::MAX, nor u32::MAX
        len.unwrap_or_else(|| TextSize::from(u32::MAX))
    }

    /// Returns the pieces count
    pub fn len(&self) -> usize {
        match &self.ptr {
            None => 0,
            Some(ptr) => ptr.len(),
        }
    }

    /// Returns the pieces of the trivia
    pub fn pieces(&self) -> &[TriviaPiece] {
        static EMPTY: [TriviaPiece; 0] = [];

        match &self.ptr {
            None => &EMPTY,
            Some(ptr) => ptr.slice(),
        }
    }

    /// Returns the piece at the given index.
    pub fn get_piece(&self, index: usize) -> Option<&TriviaPiece> {
        self.pieces().get(index)
    }
}

#[cfg(test)]
mod tests {
    use crate::api::TriviaPieceKind;
    use crate::green::trivia::{GreenTrivia, GreenTriviaHead};
    use crate::TriviaPiece;
    use text_size::TextSize;

    impl GreenTrivia {
        /// Creates a trivia with a single whitespace piece
        pub fn whitespace<L: Into<TextSize>>(len: L) -> Self {
            Self::single(TriviaPieceKind::Whitespace, len.into())
        }

        /// Creates a trivia with one single line comment piece
        pub fn single_line_comment<L: Into<TextSize>>(len: L) -> Self {
            Self::single(TriviaPieceKind::SingleLineComment, len.into())
        }

        /// Creates a trivia containing a single piece
        pub fn single<L: Into<TextSize>>(kind: TriviaPieceKind, len: L) -> Self {
            Self::new(std::iter::once(TriviaPiece::new(kind, len)))
        }
    }

    #[test]
    fn sizes() {
        assert_eq!(0, std::mem::size_of::<GreenTriviaHead>());
        assert_eq!(8, std::mem::size_of::<GreenTrivia>());
    }
}
