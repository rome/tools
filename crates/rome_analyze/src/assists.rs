#![allow(unused)]

pub mod flip_bin_exp;

use rome_js_syntax::{AstNode, SyntaxNode, SyntaxToken, TextRange, TextSize, TokenAtOffset};

use crate::{ActionCategory, Analysis, AnalysisServer, AnalyzerContext, FileId};

static ALL_ASSIST_PROVIDERS: &[AssistProvider] = &[flip_bin_exp::ASSIST];

pub struct AssistProvider {
    pub name: &'static str,
    pub action_categories: &'static [ActionCategory],
    pub analyze: fn(&AssistContext) -> Option<Analysis>,
}

pub fn all() -> impl Iterator<Item = &'static AssistProvider> {
    ALL_ASSIST_PROVIDERS.iter()
}

pub struct AssistContext<'a> {
    file_id: FileId,
    cursor_range: TextRange,
    offset: TextSize,
    analysis_server: &'a AnalysisServer,
    assist_provider: &'a AssistProvider,
}

impl<'a> AssistContext<'a> {
    pub(crate) fn new(
        analysis_server: &'a AnalysisServer,
        file_id: FileId,
        cursor_range: TextRange,
        assist_provider: &'a AssistProvider,
    ) -> Self {
        let offset = cursor_range.start();
        Self {
            cursor_range,
            offset,
            analysis_server,
            file_id,
            assist_provider,
        }
    }

    /// Get the cursor_range for this AssistContext
    pub(crate) fn range(&self) -> TextRange {
        self.cursor_range
    }

    /// Get the root [SyntaxNode] for the file being analyzed
    pub fn tree(&self) -> SyntaxNode {
        self.analysis_server.parse(self.file_id)
    }

    /// Iterate over syntax nodes in the file being analyzed that can be cast to T
    pub fn query_nodes<T: AstNode>(&self) -> impl Iterator<Item = T> {
        self.analysis_server.query_nodes(self.file_id)
    }

    /// Find the deepest AST node of type T that covers a TextRange
    pub fn find_node_at_range<T: AstNode>(&self, range: TextRange) -> Option<T> {
        self.analysis_server.find_node_at_range(self.file_id, range)
    }

    /// Find the deepest AST node of type T that covers this AssistContext's cursor_range
    pub fn find_node_at_cursor_range<T: AstNode>(&self) -> Option<T> {
        self.analysis_server
            .find_node_at_range(self.file_id, self.cursor_range)
    }

    /// Find the token that covers the start of the AssistContext's cursor_range
    pub(crate) fn token_at_offset(&self) -> TokenAtOffset<SyntaxToken> {
        self.tree().token_at_offset(self.offset)
    }
}
