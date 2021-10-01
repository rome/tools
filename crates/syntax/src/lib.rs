pub mod ast;
mod node_ext;
mod syntax_node;

use anyhow::Result;

use rowan::{GreenNode, GreenNodeBuilder};
pub use syntax_node::{
	SyntaxElement, SyntaxElementChildren, SyntaxNode, SyntaxNodeChildren, SyntaxToken,
};

use parser::TreeSink;
pub use parser::{Language, SyntaxKind};

struct Builder<'a> {
	inner: GreenNodeBuilder<'a>,
}

impl Builder<'_> {
	fn new() -> Self {
		Self {
			inner: GreenNodeBuilder::new(),
		}
	}

	fn finish(self) -> GreenNode {
		self.inner.finish()
	}
}

impl TreeSink for Builder<'_> {
	fn token(&mut self, kind: SyntaxKind, text: &str) {
		self.inner.token(rowan::SyntaxKind(kind.into()), text)
	}

	fn start_node(&mut self, kind: SyntaxKind) {
		self.inner.start_node(rowan::SyntaxKind(kind.into()))
	}

	fn finish_node(&mut self) {
		self.inner.finish_node()
	}
}

pub fn parse(src: &str, language: Language) -> Result<SyntaxNode> {
	let mut builder = Builder::new();
	parser::parse(src, &mut builder, language)?;
	let green = builder.finish();
	let node = SyntaxNode::new_root(green);
	Ok(node)
}
