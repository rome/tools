#![allow(unused)]

pub mod flip_bin_exp;

use once_cell::sync::Lazy;
use rslint_parser::{AstNode, SyntaxNode, SyntaxToken, TextRange, TextSize, TokenAtOffset};

use crate::{ActionCategory, Analysis, AnalyzerContext, FileId};

static ALL_ASSIST_PROVIDERS: Lazy<Vec<AssistProvider>> = Lazy::new(|| vec![flip_bin_exp::create()]);

pub struct AssistProvider {
	pub name: &'static str,
	pub action_categories: Vec<ActionCategory>,
	pub analyze: fn(&AssistContext) -> Option<Analysis>,
}
pub fn all() -> impl Iterator<Item = &'static AssistProvider> {
	ALL_ASSIST_PROVIDERS.iter()
}

pub struct AssistContext<'a> {
	file_id: FileId,
	cursor_range: TextRange,
	offset: TextSize,
	analyzer_context: &'a AnalyzerContext<'a>,
}

impl<'a> AssistContext<'a> {
	pub(crate) fn new(analyzer_context: &'a AnalyzerContext, cursor_range: TextRange) -> Self {
		let offset = cursor_range.start();
		Self {
			cursor_range,
			offset,
			analyzer_context,
			file_id: analyzer_context.file_id,
		}
	}

	pub(crate) fn range(&self) -> TextRange {
		self.cursor_range
	}

	pub(crate) fn tree(&self) -> SyntaxNode {
		self.analyzer_context.tree()
	}

	pub(crate) fn query_nodes<T: AstNode>(&self) -> impl Iterator<Item = T> {
		self.analyzer_context.query_nodes()
	}

	pub(crate) fn find_node_at_cursor_range<T: AstNode>(&self) -> Option<T> {
		self.analyzer_context.find_node_at_range(self.cursor_range)
	}

	pub(crate) fn token_at_offset(&self) -> TokenAtOffset<SyntaxToken> {
		self.tree().token_at_offset(self.offset)
	}
}
