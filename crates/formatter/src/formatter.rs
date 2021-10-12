use crate::printer::Printer;
use crate::{
	concat_elements, hard_line_break, join_elements, FormatElement, FormatOptions, FormatResult,
	ToFormatElement,
};
use rslint_parser::ast::{AstChildren, Stmt};
use rslint_parser::{AstNode, SyntaxNode, SyntaxToken};

/// Handles the formatting of a CST and stores the options how the CST should be formatted (user preferences).
/// The formatter is passed to the [ToFormatElement] implementation of every node in the CST so that they
/// can use it to format their children.
#[derive(Debug, Default)]
pub struct Formatter {
	options: FormatOptions,
}

impl Formatter {
	/// Creates a new context that uses the given formatter options
	pub fn new(options: FormatOptions) -> Self {
		Self { options }
	}

	/// Returns the [FormatOptions] specifying how to format the current CST
	#[inline]
	pub fn options(&self) -> &FormatOptions {
		&self.options
	}

	/// Formats a CST
	pub fn format_root(self, root: &SyntaxNode) -> FormatResult {
		let element = concat_elements(vec![
			self.format_node_start(root),
			root.to_format_element(&self),
			self.format_node_end(root),
		]);

		let printer = Printer::new(self.options);
		printer.print(&element)
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

	/// Formats the passed in token
	///
	/// # Examples
	///
	/// ```
	///
	/// use rome_formatter::{Formatter, token};
	/// use rslint_parser::{SyntaxNode, T, SyntaxToken};
	/// use rslint_rowan::{GreenNode, GreenToken, SmolStr, NodeOrToken, SyntaxKind};
	///
	/// let node = SyntaxNode::new_root(
	///   GreenNode::new(SyntaxKind(1), vec![
	///     NodeOrToken::Token(GreenToken::new(SyntaxKind(T![=>].into()), SmolStr::new("=>")))
	///   ])
	/// );
	///
	/// let syntax_token = node.first_token().unwrap();
	///
	/// let formatter = Formatter::default();
	/// let result = formatter.format_token(&syntax_token);
	///
	/// assert_eq!(token("=>"), result)
	/// ```
	pub fn format_token(&self, syntax_token: &SyntaxToken) -> FormatElement {
		syntax_token.to_format_element(self)
	}

	/// Formats a list of statements
	pub fn format_statements(&self, stmts: AstChildren<Stmt>) -> FormatElement {
		join_elements(hard_line_break(), stmts.map(|stmt| self.format_node(stmt)))
	}
}
