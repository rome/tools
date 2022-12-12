use rome_rowan::{Language, SyntaxNode};

/// Returns true if this node contains newlines in trivias.
pub fn node_has_leading_newline<L: Language>(node: &SyntaxNode<L>) -> bool {
    if let Some(leading_trivia) = node.first_leading_trivia() {
        for piece in leading_trivia.pieces() {
            if piece.is_newline() {
                return true;
            }
        }
    }
    false
}
