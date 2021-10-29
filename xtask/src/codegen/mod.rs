//! Codegen tools for generating Syntax and AST definitions. Derived from Rust analyzer's codegen
//!
//!
mod ast;
mod generate_nodes;
mod generate_syntax_kinds;
mod generate_tokens;
mod kinds_src;
mod parser_tests;

use std::path::Path;

use crate::{glue::fs2, Result};

pub use self::ast::generate_ast;
pub use self::parser_tests::generate_parser_tests;

// const GRAMMAR_DIR: &str = "crates/ra_parser/src/grammar";
// const OK_INLINE_TESTS_DIR: &str = "crates/ra_syntax/test_data/parser/inline/ok";
// const ERR_INLINE_TESTS_DIR: &str = "crates/ra_syntax/test_data/parser/inline/err";

const SYNTAX_KINDS: &str = "crates/rslint_syntax/src/generated.rs";
const AST_NODES: &str = "crates/rslint_parser/src/ast/generated/nodes.rs";
const AST_TOKENS: &str = "crates/rslint_parser/src/ast/generated/tokens.rs";

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Mode {
	Overwrite,
	Verify,
}

/// A helper to update file on disk if it has changed.
/// With verify = false,
fn update(path: &Path, contents: &str, mode: Mode) -> Result<()> {
	match fs2::read_to_string(path) {
		Ok(old_contents) if normalize(&old_contents) == normalize(contents) => {
			return Ok(());
		}
		_ => (),
	}
	if mode == Mode::Verify {
		anyhow::bail!("`{}` is not up-to-date", path.display());
	}
	eprintln!("updating {}", path.display());
	fs2::write(path, contents)?;
	return Ok(());

	fn normalize(s: &str) -> String {
		s.replace("\r\n", "\n")
	}
}

pub fn to_upper_snake_case(s: &str) -> String {
	let mut buf = String::with_capacity(s.len());
	let mut prev = false;
	for c in s.chars() {
		if c.is_ascii_uppercase() && prev {
			buf.push('_')
		}
		prev = true;

		buf.push(c.to_ascii_uppercase());
	}
	buf
}

pub fn to_lower_snake_case(s: &str) -> String {
	let mut buf = String::with_capacity(s.len());
	let mut prev = false;
	for c in s.chars() {
		if c.is_ascii_uppercase() && prev {
			buf.push('_')
		}
		prev = true;

		buf.push(c.to_ascii_lowercase());
	}
	buf
}
