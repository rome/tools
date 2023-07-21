use crate::GreenToken;
use rome_text_size::{TextRange, TextSize};
use std::ops::Deref;
use std::{borrow::Borrow, fmt::Formatter};

/// Reference to the text of a SyntaxToken without having to worry about the lifetime of `&str`.
#[derive(Eq, Clone)]
pub struct GreenTokenText {
    // Using a green token to ensure this type is Send + Sync.
    token: GreenToken,
    /// Relative range of the "selected" token text.
    range: TextRange,
}

impl std::hash::Hash for GreenTokenText {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.text().hash(state);
    }
}

impl GreenTokenText {
    pub(crate) fn new(token: GreenToken) -> GreenTokenText {
        let range = TextRange::at(TextSize::default(), token.text_len());
        Self { token, range }
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
    pub fn slice(mut self, range: TextRange) -> GreenTokenText {
        assert!(
            self.range.contains_range(range),
            "Range {range:?} exceeds bounds {:?}",
            self.range
        );

        self.range = range;
        self
    }

    pub fn range(&self) -> TextRange {
        self.range
    }

    pub fn text(&self) -> &str {
        &self.token.text()[self.range]
    }
}

impl Deref for GreenTokenText {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.text()
    }
}

impl std::fmt::Display for GreenTokenText {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.text())
    }
}

impl std::fmt::Debug for GreenTokenText {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.text())
    }
}

impl PartialEq for GreenTokenText {
    fn eq(&self, other: &Self) -> bool {
        **self == **other
    }
}

impl PartialEq<&'_ str> for GreenTokenText {
    fn eq(&self, rhs: &&'_ str) -> bool {
        **self == **rhs
    }
}

impl PartialEq<GreenTokenText> for &'_ str {
    fn eq(&self, other: &GreenTokenText) -> bool {
        **self == **other
    }
}

impl Borrow<str> for GreenTokenText {
    fn borrow(&self) -> &str {
        self.text()
    }
}
