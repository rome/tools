use crate::{AstNode, SyntaxToken};

pub trait AstNodeExt: AstNode {
    /// Return a new version of this node with the node `prev_node` replaced with `next_node`
    ///
    /// `prev_node` can be a direct child of this node, or an indirect child through any descendant node
    ///
    /// Returns `None` if `prev_node` is not a descendant of this node
    fn replace_node<N>(self, prev_node: N, next_node: N) -> Option<Self>
    where
        N: AstNode<Language = Self::Language>,
        Self: Sized;

    /// Return a new version of this node with the node `prev_node` replaced with `next_node`,
    /// transfering the leading and trailing trivia of `prev_node` to `next_node`
    ///
    /// `prev_node` can be a direct child of this node, or an indirect child through any descendant node
    ///
    /// Returns `None` if `prev_node` is not a descendant of this node
    fn replace_node_retain_trivia<N>(self, prev_node: N, next_node: N) -> Option<Self>
    where
        N: AstNode<Language = Self::Language>,
        Self: Sized;

    /// Return a new version of this node with the token `prev_token` replaced with `next_token`
    ///
    /// `prev_token` can be a direct child of this node, or an indirect child through any descendant node
    ///
    /// Returns `None` if `prev_token` is not a descendant of this node
    fn replace_token(
        self,
        prev_token: SyntaxToken<Self::Language>,
        next_token: SyntaxToken<Self::Language>,
    ) -> Option<Self>
    where
        Self: Sized;

    /// Return a new version of this node with the token `prev_token` replaced with `next_token`,
    /// transfering the leading and trailing trivia of `prev_token` to `next_token`
    ///
    /// `prev_token` can be a direct child of this node, or an indirect child through any descendant node
    ///
    /// Returns `None` if `prev_token` is not a descendant of this node
    fn replace_token_retain_trivia(
        self,
        prev_token: SyntaxToken<Self::Language>,
        next_token: SyntaxToken<Self::Language>,
    ) -> Option<Self>
    where
        Self: Sized;
}

impl<T> AstNodeExt for T
where
    T: AstNode,
{
    fn replace_node<N>(self, prev_node: N, next_node: N) -> Option<Self>
    where
        N: AstNode<Language = Self::Language>,
        Self: Sized,
    {
        Some(Self::unwrap_cast(self.into_syntax().replace_child(
            prev_node.into_syntax().into(),
            next_node.into_syntax().into(),
        )?))
    }

    fn replace_node_retain_trivia<N>(self, prev_node: N, mut next_node: N) -> Option<Self>
    where
        N: AstNode<Language = Self::Language>,
        Self: Sized,
    {
        // Lookup the first token of `prev_node` and `next_node`, and transfer the leading
        // trivia of the former to the later
        let prev_first = prev_node.syntax().first_token();
        let next_first = next_node.syntax().first_token();

        if let (Some(prev_first), Some(next_first)) = (prev_first, next_first) {
            let pieces: Vec<_> = prev_first.leading_trivia().pieces().collect();

            next_node = next_node.replace_token(
                next_first.clone(),
                next_first
                    .with_leading_trivia(pieces.iter().map(|piece| (piece.kind(), piece.text()))),
            )?;
        }

        // Lookup the last token of `prev_node` and `next_node`, and transfer the trailing
        // trivia of the former to the later
        let prev_last = prev_node.syntax().last_token();
        let next_last = next_node.syntax().last_token();

        if let (Some(prev_last), Some(next_last)) = (prev_last, next_last) {
            let pieces: Vec<_> = prev_last.trailing_trivia().pieces().collect();

            next_node = next_node.replace_token(
                next_last.clone(),
                next_last
                    .with_trailing_trivia(pieces.iter().map(|piece| (piece.kind(), piece.text()))),
            )?;
        }

        // Call replace node with the modified `next_node`
        self.replace_node(prev_node, next_node)
    }

    fn replace_token(
        self,
        prev_token: SyntaxToken<Self::Language>,
        next_token: SyntaxToken<Self::Language>,
    ) -> Option<Self>
    where
        Self: Sized,
    {
        Some(Self::unwrap_cast(
            self.into_syntax()
                .replace_child(prev_token.into(), next_token.into())?,
        ))
    }

    fn replace_token_retain_trivia(
        self,
        prev_token: SyntaxToken<Self::Language>,
        next_token: SyntaxToken<Self::Language>,
    ) -> Option<Self>
    where
        Self: Sized,
    {
        let leading_trivia: Vec<_> = prev_token.leading_trivia().pieces().collect();
        let trailing_trivia: Vec<_> = prev_token.trailing_trivia().pieces().collect();

        self.replace_token(
            prev_token,
            next_token
                .with_leading_trivia(
                    leading_trivia
                        .iter()
                        .map(|piece| (piece.kind(), piece.text())),
                )
                .with_trailing_trivia(
                    trailing_trivia
                        .iter()
                        .map(|piece| (piece.kind(), piece.text())),
                ),
        )
    }
}
