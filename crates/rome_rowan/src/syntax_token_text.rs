use rome_text_size::{TextRange, TextSize};
use std::ops::Deref;
use std::{borrow::Borrow, fmt::Formatter};

use crate::green::GreenToken;

/// Reference to the text of a SyntaxToken without having to worry about the lifetime of `&str`.
#[derive(Eq, Clone)]
pub struct SyntaxTokenText {
    // Absolute start location of `token`
    token_start: TextSize,
    // Using a green token to ensure this type is Send + Sync.
    token: GreenToken,
    /// Relative range of the "selected" token text.
    range: TextRange,
}

impl std::hash::Hash for SyntaxTokenText {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.text().hash(state);
    }
}

impl SyntaxTokenText {
    pub(crate) fn new(token: GreenToken, token_start: TextSize) -> SyntaxTokenText {
        let range = TextRange::at(TextSize::default(), token.text_len());
        Self {
            token,
            range,
            token_start,
        }
    }

    /// Returns the length of the text
    pub fn len(&self) -> TextSize {
        self.range.len()
    }

    /// Returns `true` if the text is empty
    pub fn is_empty(&self) -> bool {
        self.range.is_empty()
    }

    /// Returns a subslice of the text.
    /// `range.end()` must be lower or equal to `self.len()`
    pub fn slice(mut self, range: TextRange) -> SyntaxTokenText {
        let new_range = range + self.range.start();
        assert!(
            range.end() <= self.len(),
            "Range {range:?} exceeds the text length {:?}",
            self.len()
        );
        self.range = new_range;
        self
    }

    pub fn range(&self) -> TextRange {
        let mut range = self.range;
        range += self.token_start;
        range
    }

    pub fn text(&self) -> &str {
        &self.token.text()[self.range]
    }
}

impl Deref for SyntaxTokenText {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.text()
    }
}

impl std::fmt::Display for SyntaxTokenText {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text())
    }
}

impl std::fmt::Debug for SyntaxTokenText {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.text())
    }
}

impl PartialEq for SyntaxTokenText {
    fn eq(&self, other: &Self) -> bool {
        **self == **other
    }
}

impl PartialEq<&'_ str> for SyntaxTokenText {
    fn eq(&self, rhs: &&'_ str) -> bool {
        **self == **rhs
    }
}

impl PartialEq<SyntaxTokenText> for &'_ str {
    fn eq(&self, other: &SyntaxTokenText) -> bool {
        **self == **other
    }
}

impl Borrow<str> for SyntaxTokenText {
    fn borrow(&self) -> &str {
        self.text()
    }
}
