use crate::syntax::SyntaxTrivia;
use crate::{cursor, Language, NodeOrToken, SyntaxNode, SyntaxToken};
use std::iter;
use text_size::TextRange;

pub type SyntaxElement<L> = NodeOrToken<SyntaxNode<L>, SyntaxToken<L>>;

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

impl<L: Language> From<SyntaxToken<L>> for SyntaxElement<L> {
    fn from(token: SyntaxToken<L>) -> SyntaxElement<L> {
        NodeOrToken::Token(token)
    }
}

impl<L: Language> From<SyntaxNode<L>> for SyntaxElement<L> {
    fn from(node: SyntaxNode<L>) -> SyntaxElement<L> {
        NodeOrToken::Node(node)
    }
}
