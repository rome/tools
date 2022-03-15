pub(crate) mod no_double_equals;
pub(crate) mod no_var;
pub(crate) mod use_single_case_statement;
pub(crate) mod use_while;

use rome_js_syntax::{AstNode, SyntaxNode, TextRange};
use rslint_errors::{Diagnostic, Span};

use crate::{analysis_server::AnalysisServer, ActionCategory, Analysis, FileId};

const ALL_ANALYZERS: &[Analyzer] = &[
    no_double_equals::ANALYZER,
    no_var::ANALYZER,
    use_while::ANALYZER,
    use_single_case_statement::ANALYZER,
];

pub struct Analyzer {
    pub name: &'static str,
    pub action_categories: &'static [ActionCategory],
    pub(crate) analyze: fn(&AnalyzerContext) -> Option<Analysis>,
}

pub struct AnalyzerContext<'a> {
    pub file_id: FileId,
    analysis_server: &'a AnalysisServer,
    analyzer: &'a Analyzer,
}

impl<'a> AnalyzerContext<'a> {
    pub(crate) fn new(
        analysis_server: &'a AnalysisServer,
        file_id: FileId,
        analyzer: &'a Analyzer,
    ) -> Self {
        Self {
            analysis_server,
            file_id,
            analyzer,
        }
    }

    /// Get the root [SyntaxNode] for the file being analyzed
    pub fn tree(&self) -> SyntaxNode {
        self.analysis_server.parse(self.file_id)
    }

    /// Iterate over syntax nodes in this file that can be cast to T
    pub fn query_nodes<T: AstNode>(&self) -> impl Iterator<Item = T> {
        self.analysis_server.query_nodes(self.file_id)
    }

    /// Find the deepest AST node of type T that covers a TextRange
    pub fn find_node_at_range<T: AstNode>(&self, range: TextRange) -> Option<T> {
        self.analysis_server.find_node_at_range(self.file_id, range)
    }

    /// Create an error [Diagnostic] which includes the [FileId] of the file being analyzed
    /// and the analyzer's name as a "code". The provided [Span] is recorded as the primary
    /// label for the diagnostic.
    #[must_use]
    pub fn error(&self, span: impl Span, message: impl Into<String>) -> Diagnostic {
        let code = self.analyzer.name;
        Diagnostic::error(self.file_id, code, message.into()).primary(span, "")
    }
}

pub fn all() -> impl Iterator<Item = &'static Analyzer> {
    ALL_ANALYZERS.iter()
}

#[allow(unused)]
pub fn action_providers() -> impl Iterator<Item = &'static Analyzer> {
    ALL_ANALYZERS
        .iter()
        .filter(|a| !a.action_categories.is_empty())
}
