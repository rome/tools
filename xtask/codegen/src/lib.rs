//! Codegen tools for generating Syntax and AST definitions. Derived from Rust analyzer's codegen
//!
//!
mod ast;
mod generate_macros;
mod generate_nodes;
mod generate_syntax_factory;
mod generate_syntax_kinds;
mod kinds_src;
mod parser_tests;
mod unicode;

use std::path::Path;

use xtask::{glue::fs2, Mode, Result};

pub use self::ast::generate_ast;
pub use self::parser_tests::generate_parser_tests;
pub use self::unicode::generate_tables;

const SYNTAX_KINDS: &str = "crates/rslint_syntax/src/generated.rs";
const AST_NODES: &str = "crates/rslint_parser/src/ast/generated/nodes.rs";
const SYNTAX_FACTORY: &str = "crates/rslint_parser/src/ast/generated/syntax_factory.rs";
const AST_MACROS: &str = "crates/rslint_parser/src/ast/generated/macros.rs";

/// A helper to update file on disk if it has changed.
/// With verify = false,
fn update(path: &Path, contents: &str, mode: Mode) -> Result<()> {
    match fs2::read_to_string(path) {
        Ok(old_contents) if old_contents == contents => {
            return Ok(());
        }
        _ => (),
    }
    if mode == Mode::Verify {
        anyhow::bail!("`{}` is not up-to-date", path.display());
    }
    eprintln!("updating {}", path.display());
    fs2::write(path, contents)?;
    Ok(())
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
