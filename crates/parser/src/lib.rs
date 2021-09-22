//! Experimental parser using [tree_sitter] and [rowan]
//!
//! It uses tree-sitter to parse source code then converts
//! the parse tree into a rowan syntax tree. The integration
//! code for each grammar is automatically generated based on
//! the [`node-types.json`] that tree-sitter produces when
//! generating a parser.
//!
//! The sourcegen module borrows heavily from [rust-analyzer].
//!
//![rust-analyzer]: <https://github.com/rust-analyzer/rust-analyzer>
//![`node-types.json`]: <https://tree-sitter.github.io/tree-sitter/using-parsers#static-node-types>

pub mod languages;
pub mod sourcegen;

use anyhow::{anyhow, Result};
use rowan::{self, GreenNode, GreenNodeBuilder, SyntaxKind};

pub trait ParserLanguage {
	type SyntaxNode: std::fmt::Debug;

	fn name(&self) -> &'static str;
	fn get_kind(&self, kind: &str, named: bool) -> u16;
	fn whitespace_kind(&self) -> u16;
	fn new_root(&self, green: GreenNode) -> Self::SyntaxNode;
}

pub struct Parser<L: ParserLanguage> {
	parser: tree_sitter::Parser,
	language: L,
}

impl<L: ParserLanguage> Parser<L> {
	pub fn new(language: L) -> Result<Self> {
		let tree_sitter_language = match language.name() {
			"tsx" => tree_sitter_typescript::language_tsx(),
			"ts" => tree_sitter_typescript::language_typescript(),
			name => return Err(anyhow!("Invalid syntax name: {}", name)),
		};

		let mut parser = tree_sitter::Parser::new();
		parser.set_language(tree_sitter_language)?;
		Ok(Self { parser, language })
	}
	pub fn parse_text(&mut self, src: &str) -> Result<L::SyntaxNode> {
		let tree = self
			.parser
			.parse(src, None)
			.ok_or_else(|| anyhow!("Failed Parsing"))?;

		let green = self.build_green_tree(src, tree);
		let tree = self.language.new_root(green);
		Ok(tree)
	}

	// Uses tree-sitter's cursor api to walk the parse tree and build a rowan green tree.
	// The cursor goto methods attempt to move the cursor and return true on success.
	fn build_green_tree(&self, src: &str, tree: tree_sitter::Tree) -> GreenNode {
		let mut builder = GreenNodeBuilder::new();
		let mut cursor = tree.walk();
		let mut last_token_byte = 0;

		loop {
			let node = cursor.node();
			let node_kind = self.language.get_kind(node.kind(), node.is_named());
			let kind = SyntaxKind(node_kind);

			// Tree-sitter doesn't include whitespace tokens, but they are necessary
			// for building a lossless rowan green tree
			if last_token_byte < node.start_byte() {
				let text = &src[last_token_byte..node.start_byte()];
				let whitespace = SyntaxKind(self.language.whitespace_kind());
				builder.token(whitespace, text);
				last_token_byte += text.len();
			}

			// If the tree-sitter node has children, make it the current rowan node.
			// Otherwise, make it a token and add it to the current rowan node.
			if cursor.goto_first_child() {
				builder.start_node(kind);
				continue;
			} else {
				let text = cursor.node().utf8_text(src.as_bytes()).unwrap();
				builder.token(kind, text);
				last_token_byte = node.end_byte();
			}

			if cursor.goto_next_sibling() {
				continue;
			}

			// If this node has no more siblings, attempt to go up a level. If there is no
			// parent node, the tree is finished. Otherwise, the parent node is finished.
			loop {
				if !cursor.goto_parent() {
					return builder.finish();
				}
				builder.finish_node();

				if cursor.goto_next_sibling() {
					break;
				}
			}
		}
	}
}

#[cfg(test)]
mod tests {
	use super::{languages, Parser};
	use languages::tsx;

	#[test]
	fn ident() {
		let src = "function foo() {}";
		let mut parser = Parser::new(languages::TSX).unwrap();
		let tree = parser.parse_text(src).unwrap();
		let function = tree.first_child().unwrap();
		let ident = function
			.children_with_tokens()
			.find(|t| t.kind() == tsx::SyntaxKind::Identifier)
			.and_then(|t| t.into_token())
			.unwrap();

		assert_eq!(ident.text(), "foo");
	}
}
