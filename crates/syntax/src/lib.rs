pub mod ast;
pub mod hacky;
pub mod node_ext;
pub mod syntax_node;

use anyhow::Result;

pub use syntax_node::{
	SyntaxElement, SyntaxElementChildren, SyntaxNode, SyntaxNodeChildren, SyntaxToken,
};

pub use parser::SyntaxKind;

pub fn parse(src: &str) -> Result<SyntaxNode> {
	let green = parser::parse_text(src)?;
	let node = SyntaxNode::new_root(green);
	Ok(node)
}
