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
//! Instead use functions such as [parse_script], [parse_module], and [] which offer abstracted versions for parsing.
//!
//! Notable features of the parser are:
//! - Extremely fast parsing and lexing through the extremely fast [`rome_js_lexer`].
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
//! - ANSI syntax highlighting of nodes (through [`util`]) or text through [`rome_js_lexer`].
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
mod lexer;
mod lossless_tree_sink;
mod parse;
mod state;

#[cfg(test)]
mod tests;

pub mod syntax;
mod token_source;

pub use crate::{
    event::{process, Event},
    lexer::{LexContext, ReLexContext},
    lossless_tree_sink::LosslessTreeSink,
    parse::*,
    token_set::TokenSet,
};
pub(crate) use parser::{Checkpoint, CompletedMarker, Marker, ParseRecovery, Parser};
pub(crate) use state::{ParserState, StrictMode};
use std::fmt::Debug;

/// The type of error emitted by the parser, this includes warnings, notes, and errors.
/// It also includes labels and possibly notes
pub type ParseDiagnostic = rome_diagnostics::Diagnostic;

use crate::parser::ToDiagnostic;
pub(crate) use crate::parser::{ParseNodeList, ParseSeparatedList, ParsedSyntax};
pub(crate) use crate::ParsedSyntax::{Absent, Present};
use rome_diagnostics::Diagnostic;
use rome_js_syntax::{JsSyntaxKind, LanguageVariant};
use rome_rowan::TextSize;

/// An abstraction for syntax tree implementations
pub trait TreeSink {
    /// Adds new token to the current branch.
    fn token(&mut self, kind: JsSyntaxKind, end: TextSize);

    /// Start new branch and make it current.
    fn start_node(&mut self, kind: JsSyntaxKind);

    /// Finish current branch and restore previous
    /// branch as current.
    fn finish_node(&mut self);

    /// Emit errors
    fn errors(&mut self, errors: Vec<ParseDiagnostic>);
}

/// A syntax feature that may or may not be supported depending on the file type and parser configuration
pub(crate) trait SyntaxFeature: Sized {
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
            let diagnostics_checkpoint = p.diagnostics.len();
            let syntax = parse(p);
            p.diagnostics.truncate(diagnostics_checkpoint);

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
    Jsx,
}

impl SyntaxFeature for JsSyntaxFeature {
    fn is_supported(&self, p: &Parser) -> bool {
        match self {
            JsSyntaxFeature::SloppyMode => p.state.strict().is_none(),
            JsSyntaxFeature::StrictMode => p.state.strict().is_some(),
            JsSyntaxFeature::TypeScript => p.source_type.language().is_typescript(),
            JsSyntaxFeature::Jsx => p.source_type.variant() == LanguageVariant::Jsx,
        }
    }
}
