//! Codegen tools for generating Syntax and AST definitions. Derived from Rust analyzer's codegen
//!
//!
mod ast;
mod css_kinds_src;
mod formatter;
mod generate_macros;
mod generate_node_factory;
mod generate_nodes;
mod generate_nodes_mut;
mod generate_syntax_factory;
mod generate_syntax_kinds;
mod json_kinds_src;
mod kinds_src;
mod parser_tests;
mod termcolorful;
mod unicode;
use proc_macro2::TokenStream;
use quote::quote;
use std::path::Path;
use std::str::FromStr;

use xtask::{glue::fs2, Mode, Result};

pub use self::ast::generate_ast;
pub use self::formatter::generate_formatter;
pub use self::parser_tests::generate_parser_tests;
pub use self::unicode::generate_tables;

const JS_SYNTAX_KINDS: &str = "crates/rome_js_syntax/src/generated/kind.rs";
const JS_AST_NODES: &str = "crates/rome_js_syntax/src/generated/nodes.rs";
const JS_AST_NODES_MUT: &str = "crates/rome_js_syntax/src/generated/nodes_mut.rs";
const JS_SYNTAX_FACTORY: &str = "crates/rome_js_factory/src/generated/syntax_factory.rs";
const JS_NODE_FACTORY: &str = "crates/rome_js_factory/src/generated/node_factory.rs";
const JS_AST_MACROS: &str = "crates/rome_js_syntax/src/generated/macros.rs";

const CSS_SYNTAX_KINDS: &str = "crates/rome_css_syntax/src/generated/kind.rs";
const CSS_AST_NODES: &str = "crates/rome_css_syntax/src/generated/nodes.rs";
const CSS_AST_NODES_MUT: &str = "crates/rome_css_syntax/src/generated/nodes_mut.rs";
const CSS_SYNTAX_FACTORY: &str = "crates/rome_css_factory/src/generated/syntax_factory.rs";
const CSS_NODE_FACTORY: &str = "crates/rome_css_factory/src/generated/node_factory.rs";
const CSS_AST_MACROS: &str = "crates/rome_css_syntax/src/generated/macros.rs";

const JSON_SYNTAX_KINDS: &str = "crates/rome_json_syntax/src/generated/kind.rs";
const JSON_AST_NODES: &str = "crates/rome_json_syntax/src/generated/nodes.rs";
const JSON_AST_NODES_MUT: &str = "crates/rome_json_syntax/src/generated/nodes_mut.rs";
const JSON_SYNTAX_FACTORY: &str = "crates/rome_json_factory/src/generated/syntax_factory.rs";
const JSON_NODE_FACTORY: &str = "crates/rome_json_factory/src/generated/node_factory.rs";
const JSON_AST_MACROS: &str = "crates/rome_json_syntax/src/generated/macros.rs";

enum UpdateResult {
    NotUpdated,
    Updated,
}

#[derive(Debug, Eq, Copy, Clone, PartialEq)]
pub enum LanguageKind {
    Js,
    Css,
    Json,
}

impl std::fmt::Display for LanguageKind {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            LanguageKind::Js => write!(f, "js"),
            LanguageKind::Css => write!(f, "css"),
            LanguageKind::Json => write!(f, "json"),
        }
    }
}

pub const ALL_LANGUAGE_KIND: [LanguageKind; 3] =
    [LanguageKind::Js, LanguageKind::Css, LanguageKind::Json];

impl FromStr for LanguageKind {
    type Err = String;

    fn from_str(kind: &str) -> Result<Self, Self::Err> {
        match kind {
            "js" => Ok(LanguageKind::Js),
            "css" => Ok(LanguageKind::Css),
            "json" => Ok(LanguageKind::Json),
            _ => Err(format!(
                "Language {} not supported, please use: `js`, `css` or `json`",
                kind
            )),
        }
    }
}

impl LanguageKind {
    pub(crate) fn syntax_crate(&self) -> TokenStream {
        match self {
            LanguageKind::Js => quote! { rome_js_syntax },
            LanguageKind::Css => quote! { rome_css_syntax },
            LanguageKind::Json => quote! { rome_json_syntax },
        }
    }

    pub(crate) fn syntax_kind(&self) -> TokenStream {
        match self {
            LanguageKind::Js => quote! { JsSyntaxKind },
            LanguageKind::Css => quote! { CssSyntaxKind },
            LanguageKind::Json => quote! {JsonSyntaxKind},
        }
    }

    pub(crate) fn syntax_node(&self) -> TokenStream {
        match self {
            LanguageKind::Js => quote! { JsSyntaxNode },
            LanguageKind::Css => quote! { CssSyntaxNode },
            LanguageKind::Json => quote! { JsonSyntaxNode },
        }
    }

    pub(crate) fn syntax_element(&self) -> TokenStream {
        match self {
            LanguageKind::Js => quote! { JsSyntaxElement },
            LanguageKind::Css => quote! { CssSyntaxElement },
            LanguageKind::Json => quote! { JsonSyntaxElement },
        }
    }

    pub(crate) fn syntax_token(&self) -> TokenStream {
        match self {
            LanguageKind::Js => quote! { JsSyntaxToken },
            LanguageKind::Css => quote! { CssSyntaxToken },
            LanguageKind::Json => quote! { JsonSyntaxToken },
        }
    }

    pub(crate) fn syntax_element_children(&self) -> TokenStream {
        match self {
            LanguageKind::Js => quote! { JsSyntaxElementChildren },
            LanguageKind::Css => quote! { CssSyntaxElementChildren },
            LanguageKind::Json => quote! { JsonSyntaxElementChildren },
        }
    }

    pub(crate) fn syntax_list(&self) -> TokenStream {
        match self {
            LanguageKind::Js => quote! { JsSyntaxList },
            LanguageKind::Css => quote! { CssSyntaxList },
            LanguageKind::Json => quote! { JsonSyntaxList },
        }
    }

    pub(crate) fn language(&self) -> TokenStream {
        match self {
            LanguageKind::Js => quote! { JsLanguage },
            LanguageKind::Css => quote! { CssLanguage },
            LanguageKind::Json => quote! { JsonLanguage },
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
