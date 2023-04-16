//! Extremely fast, lossless, and error tolerant JSON Parser.

mod lexer;
mod parser;
mod syntax;
mod token_source;

use crate::parser::MarkdownParser;
use crate::syntax::parse_root;
use rome_markdown_factory::MdSyntaxFactory;
use rome_markdown_syntax::{MdLanguage, MdRoot, MdSyntaxNode};
pub use rome_parser::prelude::*;
use rome_parser::tree_sink::LosslessTreeSink;
use rome_parser::AnyParse;
use rome_rowan::{AstNode, NodeCache};

pub(crate) type MdLosslessTreeSink<'source> =
    LosslessTreeSink<'source, MdLanguage, MdSyntaxFactory>;

pub fn parse_markdown(source: &str) -> MarkdownParse {
    let mut cache = NodeCache::default();
    parse_markdown_with_cache(source, &mut cache)
}

/// Parses the provided string as JSON program using the provided node cache.
pub fn parse_markdown_with_cache(source: &str, cache: &mut NodeCache) -> MarkdownParse {
    tracing::debug_span!("parse").in_scope(move || {
        let mut parser = MarkdownParser::new(source);

        parse_root(&mut parser);

        let (events, diagnostics, trivia) = parser.finish();

        let mut tree_sink = MdLosslessTreeSink::with_cache(source, &trivia, cache);
        rome_parser::event::process(&mut tree_sink, events, diagnostics);
        let (green, diagnostics) = tree_sink.finish();

        MarkdownParse::new(green, diagnostics)
    })
}

/// A utility struct for managing the result of a parser job
#[derive(Debug)]
pub struct MarkdownParse {
    root: MdSyntaxNode,
    diagnostics: Vec<ParseDiagnostic>,
}

impl MarkdownParse {
    pub fn new(root: MdSyntaxNode, diagnostics: Vec<ParseDiagnostic>) -> MarkdownParse {
        MarkdownParse { root, diagnostics }
    }

    /// The syntax node represented by this Parse result
    ///
    /// ```
    /// # use rome_json_parser::parse_json;
    /// # use rome_json_syntax::JsonSyntaxKind;
    /// # use rome_rowan::{AstNode, AstNodeList, SyntaxError};
    ///
    /// # fn main() -> Result<(), SyntaxError> {
    /// use rome_json_syntax::JsonSyntaxKind;
    /// let parse = parse_json(r#"["a", 1]"#);
    ///
    /// // Get the root value
    /// let root_value = parse.tree().value()?;
    ///
    /// assert_eq!(root_value.syntax().kind(), JsonSyntaxKind::JSON_ARRAY_VALUE);
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub fn syntax(&self) -> MdSyntaxNode {
        self.root.clone()
    }

    /// Get the diagnostics which occurred when parsing
    pub fn diagnostics(&self) -> &[ParseDiagnostic] {
        &self.diagnostics
    }

    /// Get the diagnostics which occurred when parsing
    pub fn into_diagnostics(self) -> Vec<ParseDiagnostic> {
        self.diagnostics
    }

    /// Returns [true] if the parser encountered some errors during the parsing.
    pub fn has_errors(&self) -> bool {
        self.diagnostics
            .iter()
            .any(|diagnostic| diagnostic.is_error())
    }

    /// Convert this parse result into a typed AST node.
    ///
    /// # Panics
    /// Panics if the node represented by this parse result mismatches.
    pub fn tree(&self) -> MdRoot {
        MdRoot::unwrap_cast(self.syntax())
    }
}

impl From<MarkdownParse> for AnyParse {
    fn from(parse: MarkdownParse) -> Self {
        let root = parse.syntax();
        let diagnostics = parse.into_diagnostics();

        AnyParse::new(
            // SAFETY: the parser should always return a root node
            root.as_send().unwrap(),
            diagnostics,
        )
    }
}
