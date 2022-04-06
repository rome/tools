//! Extra utilities for untyped syntax nodes, syntax tokens, and AST nodes.

use crate::{AstNode, SyntaxNode, SyntaxToken};

/// Extensions to rowan's SyntaxNode
pub trait SyntaxNodeExt {
    #[doc(hidden)]
    fn to_node(&self) -> &SyntaxNode;

    /// Check if the node is a certain AST node and that it can be casted to it.
    fn is<T: AstNode>(&self) -> bool {
        T::can_cast(self.to_node().kind())
    }

    /// Cast this node to a certain AST node.
    ///
    /// # Panics
    /// Panics if the underlying node cannot be cast to the AST node
    fn to<T: AstNode>(&self) -> T {
        T::cast(self.to_node().to_owned()).unwrap_or_else(|| {
            panic!(
                "Tried to cast node {:?} as `{:?}` but was unable to cast",
                self.to_node(),
                std::any::type_name::<T>()
            )
        })
    }

    /// Try to cast this node to a certain AST node
    fn try_to<T: AstNode>(&self) -> Option<T> {
        T::cast(self.to_node().to_owned())
    }

    /// Whether the node contains any comments.
    fn contains_comments(&self) -> bool {
        self.to_node()
            .descendants_tokens()
            .any(|tok| tok.has_trailing_comments() || tok.has_leading_comments())
    }

    /// Whether the node contains trailing comments.
    fn has_trailing_comments(&self) -> bool {
        self.to_node()
            .last_token()
            .map_or(false, |tok| tok.has_trailing_comments())
    }

    /// Whether the node contains leading comments.
    fn has_leading_comments(&self) -> bool {
        self.to_node()
            .first_token()
            .map_or(false, |tok| tok.has_leading_comments())
    }
}

impl SyntaxNodeExt for SyntaxNode {
    fn to_node(&self) -> &SyntaxNode {
        self
    }
}

/// Concatenate tokens into a string
pub fn concat_tokens(tokens: &[SyntaxToken]) -> String {
    tokens
        .iter()
        .map(|token| token.text().to_string())
        .collect()
}
