use crate::syntax::SyntaxTrivia;
use crate::{cursor, Direction, Language, NodeOrToken, SyntaxElement, SyntaxKind, SyntaxNode};
use std::fmt;
use std::marker::PhantomData;
use text_size::TextRange;

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct SyntaxToken<L: Language> {
    raw: cursor::SyntaxToken,
    _p: PhantomData<L>,
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
        SyntaxTrivia::new(self.raw.leading_trivia())
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
        SyntaxTrivia::new(self.raw.trailing_trivia())
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
            write!(f, "{:?}", piece)?;
        }
        write!(f, "] [")?;

        let mut first_piece = true;
        for piece in self.trailing_trivia().pieces() {
            if !first_piece {
                write!(f, ", ")?;
            }
            first_piece = false;
            write!(f, "{:?}", piece)?;
        }
        write!(f, "]")
    }
}

impl<L: Language> fmt::Display for SyntaxToken<L> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(&self.raw, f)
    }
}

impl<L: Language> From<SyntaxToken<L>> for cursor::SyntaxToken {
    fn from(token: SyntaxToken<L>) -> cursor::SyntaxToken {
        token.raw
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
