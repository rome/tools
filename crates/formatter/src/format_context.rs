use crate::{concat_elements, FormatElement, FormatOptions, ToFormatElement};
use rslint_parser::{AstNode, SyntaxNode};

/// Stores shared data relevant for formatting nodes in a CST and allows formatting sub-nodes.
/// The context is passed to the [ToFormatElement] implementation of every node in the CST.
#[derive(Debug, Default)]
pub struct FormatContext {
	options: FormatOptions,
}

impl FormatContext {
	/// Creates a new context that uses the given formatter options
	pub fn new(options: FormatOptions) -> Self {
		Self { options }
	}

	/// Returns the [FormatOptions] specifying how to format the current CST
	#[inline]
	pub fn options(&self) -> &FormatOptions {
		&self.options
	}

	/// Recursively formats the root syntax node and all its children
	pub fn format_root(&self, root: &SyntaxNode) -> FormatElement {
		concat_elements(vec![
			self.format_node_start(root),
			root.to_format_element(self),
			self.format_node_end(root),
		])
	}

	/// Recursively formats the ast node and all its children
	pub fn format_node<T: AstNode + ToFormatElement>(&self, node: T) -> FormatElement {
		concat_elements(vec![
			self.format_node_start(node.syntax()),
			node.to_format_element(self),
			self.format_node_end(node.syntax()),
		])
	}

	/// Helper function that returns what should be printed before the node that work on
	/// the non-generic [SyntaxNode] to avoid unrolling the logic for every [AstNode] type.
	fn format_node_start(&self, _node: &SyntaxNode) -> FormatElement {
		// TODO: Set the marker for the start source map location, add leading comments, ...
		concat_elements(vec![])
	}

	/// Helper function that returns what should be printed after the node that work on
	/// the non-generic [SyntaxNode] to avoid unrolling the logic for every [AstNode] type.
	fn format_node_end(&self, _node: &SyntaxNode) -> FormatElement {
		// TODO: Sets the marker for the end source map location, add trailing comments, ...
		concat_elements(vec![])
	}
}
