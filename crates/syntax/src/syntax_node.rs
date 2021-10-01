use crate::SyntaxKind;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum TSX {}
impl rowan::Language for TSX {
	type Kind = SyntaxKind;
	fn kind_from_raw(raw: rowan::SyntaxKind) -> SyntaxKind {
		SyntaxKind::from(raw.0)
	}
	fn kind_to_raw(kind: SyntaxKind) -> rowan::SyntaxKind {
		rowan::SyntaxKind(kind.into())
	}
}
pub type SyntaxNode = rowan::SyntaxNode<TSX>;
pub type SyntaxToken = rowan::SyntaxToken<TSX>;
pub type SyntaxElement = rowan::SyntaxElement<TSX>;
pub type SyntaxNodeChildren = rowan::SyntaxNodeChildren<TSX>;
pub type SyntaxElementChildren = rowan::SyntaxElementChildren<TSX>;
