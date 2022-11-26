//! Codegen tools for generating Syntax and AST definitions. Derived from Rust analyzer's codegen
//!
mod ast;
mod css_kinds_src;
mod formatter;
mod generate_analyzer;
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
use proc_macro2::{Ident, Span, TokenStream};
use quote::quote;
use std::path::Path;
use std::str::FromStr;

use xtask::{glue::fs2, Mode, Result};

pub use self::ast::generate_ast;
pub use self::formatter::generate_formatters;
pub use self::generate_analyzer::generate_analyzer;
pub use self::parser_tests::generate_parser_tests;
pub use self::unicode::generate_tables;

pub enum UpdateResult {
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
    pub(crate) fn syntax_crate_ident(&self) -> Ident {
        Ident::new(self.syntax_crate_name(), Span::call_site())
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

    pub fn formatter_crate_name(&self) -> &'static str {
        match self {
            LanguageKind::Js => "rome_js_formatter",
            LanguageKind::Css => "rome_css_formatter",
            LanguageKind::Json => "rome_json_formatter",
        }
    }

    pub fn syntax_crate_name(&self) -> &'static str {
        match self {
            LanguageKind::Js => "rome_js_syntax",
            LanguageKind::Css => "rome_css_syntax",
            LanguageKind::Json => "rome_json_syntax",
        }
    }

    pub fn factory_crate_name(&self) -> &'static str {
        match self {
            LanguageKind::Js => "rome_js_factory",
            LanguageKind::Css => "rome_css_factory",
            LanguageKind::Json => "rome_json_factory",
        }
    }
}

/// A helper to update file on disk if it has changed.
/// With verify = false,
pub fn update(path: &Path, contents: &str, mode: &Mode) -> Result<UpdateResult> {
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

pub fn to_camel_case(s: &str) -> String {
    let mut buf = String::with_capacity(s.len());
    let mut prev = false;
    for c in s.chars() {
        if c == '_' {
            prev = true;
        } else if prev {
            buf.push(c.to_ascii_uppercase());
            prev = false;
        } else {
            buf.push(c);
        }
    }
    buf
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
