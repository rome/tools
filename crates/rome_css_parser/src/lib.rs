//! Extremely fast, lossless, and error tolerant CSS Parser.

use crate::parser::CssParser;

use crate::syntax::parse_root;
pub use parser::CssParserOptions;
use rome_css_factory::CssSyntaxFactory;
use rome_css_syntax::{CssLanguage, CssRoot, CssSyntaxNode};
pub use rome_parser::prelude::*;
use rome_parser::tree_sink::LosslessTreeSink;
use rome_rowan::{AstNode, NodeCache};

mod lexer;
mod parser;
mod prelude;
mod syntax;
mod token_source;

pub(crate) type CssLosslessTreeSink<'source> =
    LosslessTreeSink<'source, CssLanguage, CssSyntaxFactory>;

pub fn parse_css(source: &str, options: CssParserOptions) -> CssParse {
    let mut cache = NodeCache::default();
    parse_css_with_cache(source, &mut cache, options)
}

/// Parses the provided string as CSS program using the provided node cache.
pub fn parse_css_with_cache(
    source: &str,
    cache: &mut NodeCache,
    config: CssParserOptions,
) -> CssParse {
    tracing::debug_span!("parse").in_scope(move || {
        let mut parser = CssParser::new(source, config);

        parse_root(&mut parser);

        let (events, diagnostics, trivia) = parser.finish();

        let mut tree_sink = CssLosslessTreeSink::with_cache(source, &trivia, cache);
        rome_parser::event::process(&mut tree_sink, events, diagnostics);
        let (green, diagnostics) = tree_sink.finish();

        CssParse::new(green, diagnostics)
    })
}

/// A utility struct for managing the result of a parser job
#[derive(Debug)]
pub struct CssParse {
    root: CssSyntaxNode,
    diagnostics: Vec<ParseDiagnostic>,
}

impl CssParse {
    pub fn new(root: CssSyntaxNode, diagnostics: Vec<ParseDiagnostic>) -> CssParse {
        CssParse { root, diagnostics }
    }

    /// The syntax node represented by this Parse result
    ///
    /// ```
    /// # use rome_css_parser::parse_css;
    /// # use rome_css_syntax::CssSyntaxKind;
    /// # use rome_rowan::{AstNode, AstNodeList, SyntaxError};
    ///
    /// # fn main() -> Result<(), SyntaxError> {
    /// use rome_css_syntax::CssSyntaxKind;
    /// use rome_css_parser::CssParserOptions;
    /// let parse = parse_css(r#""#, CssParserOptions::default());
    ///
    /// let root_value = parse.tree().rules();
    ///
    /// assert_eq!(root_value.syntax().kind(), CssSyntaxKind::CSS_RULE_LIST);
    ///
    /// # Ok(())
    /// # }
    /// ```
    pub fn syntax(&self) -> CssSyntaxNode {
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
    pub fn tree(&self) -> CssRoot {
        CssRoot::unwrap_cast(self.syntax())
    }
}

#[cfg(test)]
mod tests {
    use crate::{parse_css, CssParserOptions};

    #[test]
    fn parser_smoke_test() {
        let src = r#"
"#;

        let _css = parse_css(src, CssParserOptions::default());
    }
}
