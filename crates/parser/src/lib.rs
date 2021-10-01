//! Experimental parser using [Tree_sitter] and [rowan]
//!
//! It uses tree-sitter to parse source code then converts
//! the parse tree into a rowan syntax tree. The integration
//! code for each grammar is automatically generated based on
//! the [`node-types.json`] that tree-sitter produces when
//! generating a parser.
//!
//! This borrows heavily from [rust-analyzer].
//!
//![Tree_sitter]: <https://github.com/tree-sitter/tree-sitter>
//![rowan]: <https://github.com/rust-analyzer/rowan>
//![rust-analyzer]: <https://github.com/rust-analyzer/rust-analyzer>
//![`node-types.json`]: <https://tree-sitter.github.io/tree-sitter/using-parsers#static-node-types>

use anyhow::{anyhow, Result};

mod syntax_kind;
pub use crate::syntax_kind::generated::{ts_syntax_kind, tsx_syntax_kind, SyntaxKind};

use tree_sitter_typescript::{language_tsx, language_typescript};

// TODO: Possibly add `error` method. Tree-sitter creates ERROR nodes on the parse
// tree, but we may want to add additional error handling.
pub trait TreeSink {
	/// Adds new token to the current branch.
	fn token(&mut self, kind: SyntaxKind, text: &str);

	/// Start new branch and make it current.
	fn start_node(&mut self, kind: SyntaxKind);

	/// Finish current branch and restore previous
	/// branch as current.
	fn finish_node(&mut self);
}

pub enum Language {
	Ts,
	Tsx,
}

pub fn parse(src: &str, builder: &mut dyn TreeSink, language: Language) -> Result<()> {
	let mut parser = tree_sitter::Parser::new();
	let (ts_lang, convert_kind): (tree_sitter::Language, fn(u16) -> SyntaxKind) = match language {
		Language::Ts => (language_typescript(), ts_syntax_kind),
		Language::Tsx => (language_tsx(), tsx_syntax_kind),
	};
	parser.set_language(ts_lang)?;
	let tree = parser
		.parse(src, None)
		.ok_or_else(|| anyhow!("Failed Parsing"))?;

	convert_tree(src, tree, builder, convert_kind);
	Ok(())
}

// Uses tree-sitter's cursor api to walk the parse tree and build a new tree.
// The cursor goto methods attempt to move the cursor and return true on success.
fn convert_tree(
	src: &str,
	tree: tree_sitter::Tree,
	builder: &mut dyn TreeSink,
	convert_kind: fn(u16) -> SyntaxKind,
) {
	builder.start_node(SyntaxKind::ROOT);
	let mut cursor = tree.walk();
	let mut last_token_byte = 0;

	loop {
		let node = cursor.node();
		let kind = convert_kind(node.kind_id());

		// Tree-sitter doesn't include whitespace tokens, but they are necessary
		// for building a lossless rowan green tree
		if last_token_byte < node.start_byte() {
			let text = &src[last_token_byte..node.start_byte()];
			let whitespace = SyntaxKind::WHITESPACE;
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
				return;
			}
			builder.finish_node();

			if cursor.goto_next_sibling() {
				break;
			}
		}
	}
}
