//! Extremely fast, lossless, and error tolerant JavaScript Parser.
//!
//! The parser uses an abstraction over non-whitespace tokens.
//! This allows us to losslessly or lossly parse code without requiring explicit handling of whitespace.
//! The parser yields events, not an AST, the events are resolved into untyped syntax nodes, which can then
//! be casted into a typed AST.
//!
//! The parser is able to produce a valid AST from **any** source code.
//! Erroneous productions are wrapped into `ERROR` syntax nodes, the original source code
//! is completely represented in the final syntax nodes.
//!
//! You probably do not want to use the parser struct, unless you want to parse fragments of Js source code or make your own productions.
//! Instead use functions such as [`parse_text`] and [`parse_text_lossy`] which offer abstracted versions for parsing.
//!
//! Notable features of the parser are:
//! - Extremely fast parsing and lexing through the extremely fast [`rslint_lexer`].
//! - Ability to do Lossy or Lossless parsing on demand without explicit whitespace handling.
//! - Customizable, able to parse any fragments of JS code at your discretion.
//! - Completely error tolerant, able to produce an AST from any source code.
//! - Zero cost for converting untyped nodes to a typed AST.
//! - Ability to go from AST to SyntaxNodes to SyntaxTokens to source code and back very easily with nearly zero cost.
//! - Very easy tree traversal through [`SyntaxNode`](rome_rowan::SyntaxNode).
//! - Descriptive errors with multiple labels and notes.
//! - Very cheap cloning, cloning an ast node or syntax node is the cost of adding a reference to an Rc.
//! - Cheap incremental reparsing of changed text.
//!
//! The crate further includes utilities such as:
//! - ANSI syntax highlighting of nodes (through [`util`]) or text through [`rslint_lexer`].
//! - Rich utility functions for syntax nodes through [`SyntaxNodeExt`].
//!
//! It is inspired by the rust analyzer parser but adapted for JavaScript.
//!
//! # Syntax Nodes vs AST Nodes
//! The crate relies on a concept of untyped [`SyntaxNode`]s vs typed [`AstNode`]s.
//! Syntax nodes represent the syntax tree in an untyped way. They represent a location in an immutable
//! tree with two pointers. The syntax tree is composed of [`SyntaxNode`]s and [`SyntaxToken`]s in a nested
//! tree structure. Each node can have parents, siblings, children, descendants, etc.
//!
//! [`AstNode`]s represent a typed version of a syntax node. They have the same exact representation as syntax nodes
//! therefore a conversion between either has zero runtime cost. Every piece of data of an ast node is optional,
//! this is due to the fact that the parser is completely error tolerant.
//!
//! Each representation has its advantages:
//!
//! ### SyntaxNodes
//! - Very simple traversing of the syntax tree through functions on them.
//! - Easily able to convert to underlying text, range, or tokens.
//! - Contain all whitespace bound to the underlying production (in the case of lossless parsing).
//! - Can be easily converted into its typed representation with zero cost.
//! - Can be turned into a pretty representation with fmt debug.
//!
//! ### AST Nodes
//! - Easy access to properties of the underlying production.
//! - Zero cost conversion to a syntax node.
//!
//! In conclusion, the use of both representations means we are not constrained to acting through
//! typed nodes. Which makes traversal hard and you often have to resort to autogenerated visitor patterns.
//! AST nodes are simply a way to easily access subproperties of a syntax node.event;
extern crate core;

mod parser;
#[macro_use]
mod token_set;
mod event;
mod lossless_tree_sink;
mod lossy_tree_sink;
mod numbers;
mod parse;
mod state;
mod syntax_node;
mod token_source;

#[cfg(test)]
mod tests;

#[macro_use]
pub mod ast;
pub mod syntax;
pub mod util;

pub use crate::{
    ast::{AstNode, AstNodeList, AstSeparatedList, AstToken, SyntaxError, SyntaxResult},
    event::{process, Event},
    lossless_tree_sink::LosslessTreeSink,
    lossy_tree_sink::LossyTreeSink,
    numbers::BigInt,
    parse::*,
    parser::{Checkpoint, CompletedMarker, Marker, ParseRecovery, Parser},
    syntax_node::*,
    token_set::TokenSet,
    token_source::TokenSource,
    util::{SyntaxNodeExt, SyntaxTokenExt},
};

pub(crate) use state::{ParserState, StrictMode};

pub use rome_rowan::{SyntaxText, TextRange, TextSize, TokenAtOffset, WalkEvent};

pub use rslint_syntax::*;

/// The type of error emitted by the parser, this includes warnings, notes, and errors.
/// It also includes labels and possibly notes
pub type ParserError = rslint_errors::Diagnostic;

use crate::parser::ToDiagnostic;
pub use crate::parser::{ParseNodeList, ParseSeparatedList, ParsedSyntax};
pub use crate::ParsedSyntax::{Absent, Present};
use rslint_errors::Diagnostic;
use std::ops::Range;

/// An abstraction for syntax tree implementations
pub trait TreeSink {
    /// Adds new token to the current branch.
    fn token(&mut self, kind: JsSyntaxKind);

    /// Start new branch and make it current.
    fn start_node(&mut self, kind: JsSyntaxKind);

    /// Finish current branch and restore previous
    /// branch as current.
    fn finish_node(&mut self);

    /// Emit errors
    fn errors(&mut self, errors: Vec<ParserError>);

    /// Consume multiple tokens and glue them into one kind
    fn consume_multiple_tokens(&mut self, amount: u8, kind: JsSyntaxKind);
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum LanguageVersion {
    ES2022,
    ESNext,
}

impl LanguageVersion {
    pub const fn latest() -> Self {
        LanguageVersion::ES2022
    }
}

impl Default for LanguageVersion {
    fn default() -> Self {
        Self::latest()
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum ModuleKind {
    Script,
    Module,
}

impl ModuleKind {
    pub fn is_script(&self) -> bool {
        matches!(self, ModuleKind::Script)
    }
    pub fn is_module(&self) -> bool {
        matches!(self, ModuleKind::Module)
    }
}

impl Default for ModuleKind {
    fn default() -> Self {
        ModuleKind::Module
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum LanguageVariant {
    Standard,
    JSX,
}

impl LanguageVariant {
    pub fn is_standard(&self) -> bool {
        matches!(self, LanguageVariant::Standard)
    }
    pub fn is_jsx(&self) -> bool {
        matches!(self, LanguageVariant::JSX)
    }
}

impl Default for LanguageVariant {
    fn default() -> Self {
        LanguageVariant::Standard
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum Language {
    JS,
    TS { definition_file: bool },
}

impl Language {
    pub fn is_javascript(&self) -> bool {
        matches!(self, Language::JS)
    }
    pub fn is_typescript(&self) -> bool {
        matches!(self, Language::TS { .. })
    }
}

impl Default for Language {
    fn default() -> Self {
        Language::JS
    }
}

#[derive(Clone, Debug, Default)]
pub struct SourceType {
    language: Language,
    variant: LanguageVariant,
    module_kind: ModuleKind,
    version: LanguageVersion,
}

impl SourceType {
    /// language: JS, variant: Standard, module_kind: Module, version: Latest
    pub fn js() -> Self {
        Self::default()
    }

    /// language: JS, variant: Standard, module_kind: Script, version: Latest
    pub fn js_script() -> Self {
        Self::default().with_module_kind(ModuleKind::Script)
    }

    /// language: JS, variant: JSX, module_kind: Module, version: Latest
    pub fn jsx() -> SourceType {
        Self::js().with_variant(LanguageVariant::JSX)
    }

    /// language: TS, variant: Standard, module_kind: Module, version: Latest
    pub fn ts() -> SourceType {
        Self {
            language: Language::TS {
                definition_file: false,
            },
            ..Self::default()
        }
    }

    /// language: JS, variant: JSX, module_kind: Module, version: Latest
    pub fn tsx() -> SourceType {
        Self::ts().with_variant(LanguageVariant::JSX)
    }

    /// language: TS, ambient, variant: Standard, module_kind: Module, version: Latest
    pub fn tsd() -> SourceType {
        Self {
            language: Language::TS {
                definition_file: true,
            },
            ..Self::default()
        }
    }

    pub fn from_path(path: &std::path::Path) -> Option<SourceType> {
        let file_name = path.file_name()?.to_str()?;

        let source_type = if file_name.ends_with(".d.ts") || file_name.ends_with(".d.mts") {
            Self::tsd()
        } else if file_name.ends_with(".d.cts") {
            Self::tsd().with_module_kind(ModuleKind::Script)
        } else {
            let extension = path.extension()?.to_str()?;
            match extension {
                "js" | "mjs" => Self::js(),
                "cjs" => Self::js().with_module_kind(ModuleKind::Script),
                "jsx" => Self::jsx(),
                "ts" | "mts" => Self::ts(),
                "cts" => Self::ts().with_module_kind(ModuleKind::Script),
                "tsx" => Self::tsx(),
                _ => return None,
            }
        };

        Some(source_type)
    }

    pub fn with_module_kind(mut self, kind: ModuleKind) -> Self {
        self.module_kind = kind;
        self
    }

    pub fn with_version(mut self, version: LanguageVersion) -> Self {
        self.version = version;
        self
    }

    pub fn with_variant(mut self, variant: LanguageVariant) -> Self {
        self.variant = variant;
        self
    }

    pub fn language(&self) -> Language {
        self.language
    }

    pub fn variant(&self) -> LanguageVariant {
        self.variant
    }

    pub fn version(&self) -> LanguageVersion {
        self.version
    }

    pub fn module_kind(&self) -> ModuleKind {
        self.module_kind
    }

    pub fn is_module(&self) -> bool {
        self.module_kind.is_module()
    }
}

/// A syntax feature that may or may not be supported depending on the file type and parser configuration
pub trait SyntaxFeature: Sized {
    /// Returns `true` if the current parsing context supports this syntax feature.
    fn is_supported(&self, p: &Parser) -> bool;

    /// Returns `true` if the current parsing context doesn't support this syntax feature.
    fn is_unsupported(&self, p: &Parser) -> bool {
        !self.is_supported(p)
    }

    /// Adds a diagnostic and changes the kind of the node to [SyntaxKind::to_unknown] if this feature isn't
    /// supported.
    ///
    /// Returns the parsed syntax.
    fn exclusive_syntax<S, E, D>(&self, p: &mut Parser, syntax: S, error_builder: E) -> ParsedSyntax
    where
        S: Into<ParsedSyntax>,
        E: FnOnce(&Parser, &CompletedMarker) -> D,
        D: ToDiagnostic,
    {
        syntax.into().map(|mut syntax| {
            if self.is_unsupported(p) {
                let error = error_builder(p, &syntax);
                p.error(error);
                syntax.change_to_unknown(p);
                syntax
            } else {
                syntax
            }
        })
    }

    /// Parses a syntax and adds a diagnostic and changes the kind of the node to [SyntaxKind::to_unknown] if this feature isn't
    /// supported.
    ///
    /// Returns the parsed syntax.
    fn parse_exclusive_syntax<P, E>(
        &self,
        p: &mut Parser,
        parse: P,
        error_builder: E,
    ) -> ParsedSyntax
    where
        P: FnOnce(&mut Parser) -> ParsedSyntax,
        E: FnOnce(&Parser, &CompletedMarker) -> Diagnostic,
    {
        if self.is_supported(p) {
            parse(p)
        } else {
            let diagnostics_checkpoint = p.errors.len();
            let syntax = parse(p);
            p.errors.truncate(diagnostics_checkpoint);

            match syntax {
                Present(mut syntax) => {
                    let diagnostic = error_builder(p, &syntax);
                    p.error(diagnostic);
                    syntax.change_to_unknown(p);
                    Present(syntax)
                }
                _ => Absent,
            }
        }
    }

    /// Adds a diagnostic and changes the kind of the node to [SyntaxKind::to_unknown] if this feature is
    /// supported.
    ///
    /// Returns the parsed syntax.
    fn excluding_syntax<S, E>(&self, p: &mut Parser, syntax: S, error_builder: E) -> ParsedSyntax
    where
        S: Into<ParsedSyntax>,
        E: FnOnce(&Parser, &CompletedMarker) -> Diagnostic,
    {
        syntax.into().map(|mut syntax| {
            if self.is_unsupported(p) {
                syntax
            } else {
                let error = error_builder(p, &syntax);
                p.error(error);
                syntax.change_to_unknown(p);
                syntax
            }
        })
    }
}

pub enum JsSyntaxFeature {
    #[allow(unused)]
    #[doc(alias = "LooseMode")]
    SloppyMode,
    StrictMode,
    TypeScript,
}

impl SyntaxFeature for JsSyntaxFeature {
    fn is_supported(&self, p: &Parser) -> bool {
        match self {
            JsSyntaxFeature::SloppyMode => p.state.strict().is_none(),
            JsSyntaxFeature::StrictMode => p.state.strict().is_some(),
            JsSyntaxFeature::TypeScript => p.source_type.language().is_typescript(),
        }
    }
}
