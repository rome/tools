pub(crate) mod no_double_equals;
pub(crate) mod no_var;
pub(crate) mod use_single_case_statement;
pub(crate) mod use_while;

use once_cell::sync::Lazy;
use rslint_parser::{AstNode, SyntaxNode, TextRange};

use crate::{analysis_server::AnalysisServer, ActionCategory, AnalyzerResult, FileId};

static ALL_ANALYZERS: Lazy<Vec<Analyzer>> = Lazy::new(|| {
	vec![
		no_double_equals::create(),
		no_var::create(),
		use_while::create(),
		use_single_case_statement::create(),
	]
});

pub struct Analyzer {
	pub name: &'static str,
	pub action_categories: Vec<ActionCategory>,
	pub(crate) analyze: fn(&AnalyzerContext) -> AnalyzerResult,
}

pub struct AnalyzerContext<'a> {
	pub file_id: FileId,
	analysis: &'a AnalysisServer,
}

impl<'a> AnalyzerContext<'a> {
	pub fn new(analysis: &'a AnalysisServer, file_id: FileId) -> Self {
		Self { analysis, file_id }
	}

	pub fn tree(&self) -> SyntaxNode {
		self.analysis.parse(self.file_id)
	}

	pub fn query_nodes<T: AstNode>(&self) -> impl Iterator<Item = T> {
		self.analysis.query_nodes(self.file_id)
	}

	pub fn find_node_at_range<T: AstNode>(&self, range: TextRange) -> Option<T> {
		self.analysis.find_node_at_range(self.file_id, range)
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
