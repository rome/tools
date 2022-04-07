//! Codegen tools for generating Syntax and AST definitions. Derived from Rust analyzer's codegen
//!
//!
mod ast;
mod css_kinds_src;
mod formatter;
mod generate_macros;
mod generate_nodes;
mod generate_syntax_factory;
mod generate_syntax_kinds;
mod kinds_src;
mod parser_tests;
mod unicode;

use proc_macro2::TokenStream;
use quote::quote;
use std::path::Path;

use xtask::{glue::fs2, Mode, Result};

pub use self::ast::generate_ast;
pub use self::formatter::generate_formatter;
pub use self::parser_tests::generate_parser_tests;
pub use self::unicode::generate_tables;

const JS_SYNTAX_KINDS: &str = "crates/rome_js_syntax/src/generated/kind.rs";
const JS_AST_NODES: &str = "crates/rome_js_syntax/src/generated/nodes.rs";
const JS_SYNTAX_FACTORY: &str = "crates/rome_js_syntax/src/generated/syntax_factory.rs";
const JS_AST_MACROS: &str = "crates/rome_js_syntax/src/generated/macros.rs";

const CSS_SYNTAX_KINDS: &str = "crates/rome_css_syntax/src/generated/kind.rs";
const CSS_AST_NODES: &str = "crates/rome_css_syntax/src/generated/nodes.rs";
const CSS_SYNTAX_FACTORY: &str = "crates/rome_css_syntax/src/generated/syntax_factory.rs";
const CSS_AST_MACROS: &str = "crates/rome_css_syntax/src/generated/macros.rs";

enum UpdateResult {
    NotUpdated,
    Updated,
}

#[derive(Debug, Eq, Copy, Clone, PartialEq)]
pub enum LanguageKind {
    Js,
    Css,
}

impl LanguageKind {
    pub fn syntax_kind(&self) -> TokenStream {
        match self {
            LanguageKind::Js => quote! { JsSyntaxKind },
            LanguageKind::Css => quote! { CssSyntaxKind },
        }
    }
}

/// A helper to update file on disk if it has changed.
/// With verify = false,
fn update(path: &Path, contents: &str, mode: &Mode) -> Result<UpdateResult> {
    match fs2::read_to_string(path) {
        Ok(old_contents) if old_contents == contents => {
            return Ok(UpdateResult::NotUpdated);
        }
        _ => (),
    }

    if *mode == Mode::Verify {
        anyhow::bail!("`{}` is not up-to-date", path.display());
    }

    eprintln!("updating {}", path.display());
    fs2::write(path, contents)?;
    Ok(UpdateResult::Updated)
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
