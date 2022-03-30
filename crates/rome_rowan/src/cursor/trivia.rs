use crate::cursor::SyntaxToken;
use crate::green::GreenTrivia;
use crate::TriviaPiece;
use std::fmt;
use text_size::{TextRange, TextSize};

#[derive(PartialEq, Eq, Clone, Hash)]
pub(crate) struct SyntaxTrivia {
    offset: TextSize,
    token: SyntaxToken,
    is_leading: bool,
}

impl SyntaxTrivia {
    pub(super) fn leading(offset: TextSize, token: SyntaxToken) -> Self {
        Self {
            offset,
            token,
            is_leading: true,
        }
    }

    pub(super) fn trailing(offset: TextSize, token: SyntaxToken) -> Self {
        Self {
            offset,
            token,
            is_leading: false,
        }
    }

    pub(crate) fn text(&self) -> &str {
        let green_token = self.token.green();
        if self.is_leading {
            green_token.text_leading_trivia()
        } else {
            green_token.text_trailing_trivia()
        }
    }

    pub(crate) fn text_range(&self) -> TextRange {
        let green_token = self.token.green();
        if self.is_leading {
            TextRange::at(self.offset, green_token.leading_trivia().text_len())
        } else {
            let (_, trailing_len, total_len) = green_token.leading_trailing_total_len();
            TextRange::at(self.offset + total_len - trailing_len, trailing_len)
        }
    }

    /// Get the number of TriviaPiece inside this trivia
    fn len(&self) -> usize {
        self.green_trivia().len()
    }

    /// Get the total length of text of the TriviaPieces inside this trivia
    fn text_len(&self) -> TextSize {
        self.green_trivia().text_len()
    }

    /// Gets index-th trivia piece when the token associated with this trivia was created.
    /// See [SyntaxTriviaPiece].
    pub(crate) fn get_piece(&self, index: usize) -> Option<&TriviaPiece> {
        self.green_trivia().get_piece(index)
    }

    fn green_trivia(&self) -> &GreenTrivia {
        match self.is_leading {
            true => self.token.green().leading_trivia(),
            false => self.token.green().trailing_trivia(),
        }
    }

    /// Returns the last trivia piece element
    pub(crate) fn last(&self) -> Option<&TriviaPiece> {
        self.green_trivia().pieces().last()
    }

    /// Iterate over all pieces of the trivia. The iterator returns the offset
    /// of the trivia as [TextSize] and its data as [Trivia], which contains its length.
    /// See [SyntaxTriviaPiece].
    pub(crate) fn pieces(&self) -> SyntaxTriviaPiecesIterator {
        SyntaxTriviaPiecesIterator {
            raw: self.clone(),
            next_index: 0,
            next_offset: self.offset,
            end_index: self.len(),
            end_offset: self.offset + self.text_len(),
        }
    }

    #[inline]
    pub(crate) fn offset(&self) -> TextSize {
        self.offset
    }
}

impl fmt::Debug for SyntaxTrivia {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut f = f.debug_struct("SyntaxTrivia");
        f.field("text_range", &self.text_range());
        f.finish()
    }
}

impl fmt::Display for SyntaxTrivia {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self.text(), f)
    }
}

pub struct SyntaxTriviaPiecesIterator {
    pub(crate) raw: SyntaxTrivia,
    pub(crate) next_index: usize,
    pub(crate) next_offset: TextSize,
    pub(crate) end_index: usize,
    pub(crate) end_offset: TextSize,
}

impl Iterator for SyntaxTriviaPiecesIterator {
    type Item = (TextSize, TriviaPiece);

    fn next(&mut self) -> Option<Self::Item> {
        let trivia = self.raw.get_piece(self.next_index)?;
        let piece = (self.next_offset, *trivia);

        self.next_index += 1;
        self.next_offset += trivia.text_len();

        Some(piece)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.end_index.saturating_sub(self.next_index);
        (len, Some(len))
    }
}

impl DoubleEndedIterator for SyntaxTriviaPiecesIterator {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.end_index == self.next_index {
            return None;
        }

        self.end_index -= 1;

        let trivia = self.raw.get_piece(self.end_index)?;
        self.end_offset -= trivia.text_len();

        Some((self.end_offset, *trivia))
    }
}

impl ExactSizeIterator for SyntaxTriviaPiecesIterator {}
