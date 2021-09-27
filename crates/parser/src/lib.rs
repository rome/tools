//! Experimental parser using [Tree_sitter] and [rowan]
//!
//! It uses tree-sitter to parse source code then converts
//! the parse tree into a rowan syntax tree. The integration
//! code for each grammar is automatically generated based on
//! the [`node-types.json`] that tree-sitter produces when
//! generating a parser.
//!
//! The sourcegen module borrows heavily from [rust-analyzer].
//!
//![Tree_sitter]: <https://github.com/tree-sitter/tree-sitter>
//![rowan]: <https://github.com/rust-analyzer/rowan>
//![rust-analyzer]: <https://github.com/rust-analyzer/rust-analyzer>
//![`node-types.json`]: <https://tree-sitter.github.io/tree-sitter/using-parsers#static-node-types>

use anyhow::{anyhow, Result};
use rowan::{self, GreenNode, GreenNodeBuilder};

mod syntax_kind;
pub use crate::syntax_kind::{get_syntax_kind, SyntaxKind};

pub fn parse_text(src: &str) -> Result<GreenNode> {
	let mut parser = tree_sitter::Parser::new();
	parser.set_language(tree_sitter_typescript::language_tsx())?;

	let tree = parser
		.parse(src, None)
		.ok_or_else(|| anyhow!("Failed Parsing"))?;

	let green = build_green_tree(src, tree);
	Ok(green)
}

// Uses tree-sitter's cursor api to walk the parse tree and build a rowan green tree.
// The cursor goto methods attempt to move the cursor and return true on success.
fn build_green_tree(src: &str, tree: tree_sitter::Tree) -> GreenNode {
	let mut builder = GreenNodeBuilder::new();
	builder.start_node(rowan::SyntaxKind(SyntaxKind::ROOT.into()));
	let mut cursor = tree.walk();
	let mut last_token_byte = 0;

	loop {
		let node = cursor.node();
		let node_kind = get_syntax_kind(node.kind(), node.is_named());
		let kind = rowan::SyntaxKind(node_kind.into());

		// Tree-sitter doesn't include whitespace tokens, but they are necessary
		// for building a lossless rowan green tree
		if last_token_byte < node.start_byte() {
			let text = &src[last_token_byte..node.start_byte()];
			let whitespace = rowan::SyntaxKind(SyntaxKind::Whitespace.into());
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
				// Finish ROOT node
				builder.finish_node();
				return builder.finish();
			}
			builder.finish_node();

			if cursor.goto_next_sibling() {
				break;
			}
		}
	}
}
