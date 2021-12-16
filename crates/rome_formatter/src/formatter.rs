use crate::printer::Printer;
use crate::{
	concat_elements, format_elements, if_group_breaks, token, FormatElement, FormatOptions,
	FormatResult, Formatted, ToFormatElement,
};
use rome_rowan::SyntaxElement;
use rslint_parser::{AstNode, AstSeparatedList, SyntaxNode, SyntaxToken};

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
	pub fn format_root(self, root: &SyntaxNode) -> FormatResult<Formatted> {
		let element = self.format_syntax_node(root)?;

		let printer = Printer::new(self.options);
		Ok(printer.print(&element))
	}

	fn format_syntax_node(&self, node: &SyntaxNode) -> FormatResult<FormatElement> {
		let start = self.format_node_start(node);
		let content = node.to_format_element(self)?;
		Ok(concat_elements(vec![
			start,
			content,
			self.format_node_end(node),
		]))
	}

	/// Recursively formats the ast node and all its children
	///
	/// Returns `None` if the node couldn't be formatted because of syntax errors in its sub tree.
	/// The parent may use `format_raw` to insert the node content as is.
	pub fn format_node<T: AstNode + ToFormatElement>(
		&self,
		node: T,
	) -> FormatResult<FormatElement> {
		Ok(concat_elements(vec![
			self.format_node_start(node.syntax()),
			node.to_format_element(self)?,
			self.format_node_end(node.syntax()),
		]))
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

	/// Formats the passed in token.
	///
	/// May return `None` if the token wasn't present in the original source but was inserted
	/// by the parser to "fix" a syntax error and generate a valid tree.
	///
	/// # Examples
	///
	/// ```
	///
	/// use rome_formatter::{Formatter, token};
	/// use rslint_parser::{SyntaxNode, T, SyntaxToken, JsLanguage, SyntaxKind};
	/// use rome_rowan::{NodeOrToken, TreeBuilder};
	///
	/// let mut builder = TreeBuilder::<'_, JsLanguage>::new();
	/// builder.start_node(SyntaxKind::JS_STRING_LITERAL_EXPRESSION);
	/// builder.token(SyntaxKind::JS_STRING_LITERAL, "'abc'");
	/// builder.finish_node();
	/// let node = builder.finish();
	///
	/// let syntax_token = node.first_token().unwrap();
	///
	/// let formatter = Formatter::default();
	/// let result = formatter.format_token(&syntax_token);
	///
	/// assert_eq!(Ok(token("'abc'")), result)
	/// ```
	pub fn format_token(&self, syntax_token: &SyntaxToken) -> FormatResult<FormatElement> {
		Ok(token(syntax_token.text_trimmed()))
	}

	/// Formats each child and returns the result as a list.
	///
	/// Returns [None] if a child couldn't be formatted.
	pub fn format_nodes<T: AstNode + ToFormatElement>(
		&self,
		nodes: impl IntoIterator<Item = T>,
	) -> FormatResult<impl Iterator<Item = FormatElement>> {
		let mut result = Vec::new();

		for node in nodes {
			match self.format_node(node) {
				Ok(formatted) => {
					result.push(formatted);
				}
				Err(err) => return Err(err),
			}
		}

		Ok(result.into_iter())
	}

	pub fn format_separated<T: AstNode + ToFormatElement + Clone, L: AstSeparatedList<T>>(
		&self,
		list: L,
	) -> FormatResult<impl Iterator<Item = FormatElement>> {
		let mut result = Vec::with_capacity(list.len());

		for (index, element) in list.elements().enumerate() {
			let node = self.format_node(element.node()?)?;
			if let Some(separator) = element.trailing_separator()? {
				let formatted_separator = self.format_token(&separator)?;
				if index == list.len() - 1 {
					result.push(format_elements![node, if_group_breaks(formatted_separator)])
				} else {
					result.push(format_elements![node, formatted_separator]);
				}
			} else {
				result.push(node);
			}
		}

		Ok(result.into_iter())
	}

	/// "Formats" a node according to its original formatting in the source text. Being able to format
	/// a node "as is" is useful if a node contains syntax errors. Formatting a node with syntax errors
	/// has the risk that Rome misinterprets the structure of the code and formatting it could
	/// "mess up" the developers, yet incomplete, work or accidentally introduce new syntax errors.
	///
	/// You may be inclined to call `node.text` directly. However, using `text` doesn't track the nodes
	///nor its children source mapping information, resulting in incorrect source maps for this subtree.
	pub fn format_raw(&self, node: &SyntaxNode) -> FormatElement {
		concat_elements(node.children_with_tokens().map(|child| match child {
			SyntaxElement::Node(child_node) => {
				// TODO: Add source map markers before/after node as well as any additional elements that
				// need to be tracked for every node.
				self.format_raw(&child_node)
			}
			SyntaxElement::Token(syntax_token) => token(syntax_token.text()),
		}))
	}
}
