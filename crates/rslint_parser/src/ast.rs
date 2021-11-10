//! AST definitions for converting untyped syntax nodes into typed AST nodes.
//!
//! Every field of every AST node is optional, this is to allow the parser to recover
//! from any error and produce an ast from any source code. If you don't want to account for
//! optionals for everything, you can use ...

#[macro_use]
mod expr_ext;
mod generated;
mod stmt_ext;
mod ts_ext;

use crate::{syntax_node::*, util::SyntaxNodeExt, SyntaxKind, SyntaxList, TextRange};
use std::iter::FusedIterator;
use std::marker::PhantomData;

pub use self::{
	expr_ext::*,
	generated::{nodes::*, tokens::*},
	stmt_ext::*,
	ts_ext::*,
};

/// The main trait to go from untyped `SyntaxNode`  to a typed ast. The
/// conversion itself has zero runtime cost: ast and syntax nodes have exactly
/// the same representation: a pointer to the tree root and a pointer to the
/// node itself.
pub trait AstNode {
	fn can_cast(kind: SyntaxKind) -> bool
	where
		Self: Sized;

	fn cast(syntax: SyntaxNode) -> Option<Self>
	where
		Self: Sized;

	fn syntax(&self) -> &SyntaxNode;

	fn text(&self) -> std::string::String {
		self.syntax().trimmed_text().to_string()
	}

	fn range(&self) -> TextRange {
		self.syntax().trimmed_range()
	}
}

/// Like `AstNode`, but wraps tokens rather than interior nodes.
pub trait AstToken {
	fn can_cast(token: SyntaxKind) -> bool
	where
		Self: Sized;

	fn cast(syntax: SyntaxToken) -> Option<Self>
	where
		Self: Sized;

	fn syntax(&self) -> &SyntaxToken;

	fn text(&self) -> &str {
		self.syntax().text()
	}
}

/// An iterator over `SyntaxNode` children of a particular AST type.
#[derive(Debug, Clone)]
pub struct AstChildren<N> {
	inner: SyntaxNodeChildren,
	ph: PhantomData<N>,
}

impl<N> AstChildren<N> {
	fn new(parent: &SyntaxNode) -> Self {
		AstChildren {
			inner: parent.children(),
			ph: PhantomData,
		}
	}
}

impl<N: AstNode> Iterator for AstChildren<N> {
	type Item = N;
	fn next(&mut self) -> Option<N> {
		self.inner.find_map(N::cast)
	}
}

/// List of homogenous nodes
#[derive(Debug, Clone)]
pub struct AstNodeList<N> {
	inner: SyntaxList,
	ph: PhantomData<N>,
}

impl<N> Default for AstNodeList<N> {
	fn default() -> Self {
		AstNodeList {
			inner: SyntaxList::default(),
			ph: PhantomData,
		}
	}
}

impl<N: AstNode> AstNodeList<N> {
	/// Creates a new node list wrapping the passed in syntax list
	fn new(list: SyntaxList) -> Self {
		AstNodeList {
			inner: list,
			ph: PhantomData,
		}
	}

	pub fn iter(&self) -> AstNodeListIterator<N> {
		AstNodeListIterator {
			inner: self.inner.iter(),
			ph: PhantomData,
		}
	}

	#[inline]
	pub fn len(&self) -> usize {
		self.inner.len()
	}

	/// Returns the first node from this list or None
	#[inline]
	pub fn first(&self) -> Option<N> {
		// TODO 1724: Use inner once trivia is attached to tokens (not safe yet)
		self.iter().next()
	}

	/// Returns the last node from this list or None
	pub fn last(&self) -> Option<N> {
		// TODO 1724: Use inner once trivia is attached to tokens (not safe yet)
		self.iter().last()
	}

	#[inline]
	pub fn is_empty(&self) -> bool {
		self.inner.is_empty()
	}
}

#[derive(Debug, Clone)]
pub struct AstNodeListIterator<N> {
	inner: SyntaxElementChildren,
	ph: PhantomData<N>,
}

impl<N: AstNode> Iterator for AstNodeListIterator<N> {
	type Item = N;

	fn next(&mut self) -> Option<Self::Item> {
		// TODO 1724: Replace find map with force cast element into `N`.
		// The code gen guarantees us that all elements must be of type N
		self.inner.find_map(|e| {
			let syntax = e.into_node()?;

			if syntax.kind() == SyntaxKind::ERROR {
				None
			} else {
				Some(syntax.to::<N>())
			}
		})
	}
}

impl<N: AstNode> FusedIterator for AstNodeListIterator<N> {}

impl<N: AstNode> IntoIterator for &AstNodeList<N> {
	type Item = N;
	type IntoIter = AstNodeListIterator<N>;

	fn into_iter(self) -> Self::IntoIter {
		self.iter()
	}
}

impl<N: AstNode> IntoIterator for AstNodeList<N> {
	type Item = N;
	type IntoIter = AstNodeListIterator<N>;

	fn into_iter(self) -> Self::IntoIter {
		self.iter()
	}
}

#[derive(Debug, Clone)]
pub struct AstSeparatedElement<N> {
	node: N,
	trailing_separator: Option<SyntaxToken>,
}

/// List of nodes where every two nodes are separated by a token.
/// For example, the elements of an array where every two elements are separated by a comma token.
#[derive(Debug, Clone)]
pub struct AstSeparatedList<N> {
	list: SyntaxList,
	ph: PhantomData<N>,
}

impl<N: AstNode> AstSeparatedList<N> {
	fn new(list: SyntaxList) -> Self {
		Self {
			list,
			ph: PhantomData,
		}
	}

	/// Returns an iterator over all nodes with their trailing separator
	pub fn elements(&self) -> AstSeparatedListElementsIterator<N> {
		AstSeparatedListElementsIterator {
			next: self.list.iter(),
			ph: PhantomData,
		}
	}

	/// Returns an iterator over all separator tokens
	pub fn separators(&self) -> impl Iterator<Item = SyntaxToken> {
		self.elements()
			.flat_map(|element| element.trailing_separator)
	}

	/// Returns an iterator over all nodes
	pub fn iter(&self) -> AstSeparatedListNodesIterator<N> {
		AstSeparatedListNodesIterator {
			inner: self.elements(),
		}
	}

	#[inline]
	pub fn is_empty(&self) -> bool {
		self.len() == 0
	}

	pub fn len(&self) -> usize {
		// TODO 1724 replace with (self.list.len() + 1) << 2 once trivia are attached to tokens
		self.iter().count()
	}

	pub fn trailing_separator(&self) -> Option<SyntaxToken> {
		// TODO 1724: Replace with simple match once trivia is no longer stored in the body
		// match self.list.last() {
		// 	NodeOrToken::Token(token) => Some(token),
		// 	_ => None
		// }

		self.elements().last()?.trailing_separator
	}
}

#[derive(Debug, Clone)]
pub struct AstSeparatedListElementsIterator<N> {
	next: SyntaxElementChildren,
	ph: PhantomData<N>,
}

impl<N> AstSeparatedListElementsIterator<N> {
	// TODO 1724: Replace with call to next once trivia are no longer stored in tokens and errors are part of the union types.
	fn next_non_trivia_or_error(&mut self) -> Option<SyntaxElement> {
		self.next.find_map(|element| match &element {
			NodeOrToken::Node(node) => {
				if node.kind() == SyntaxKind::ERROR {
					None
				} else {
					Some(element)
				}
			}
			NodeOrToken::Token(token) => {
				if token.kind().is_trivia() {
					None
				} else {
					Some(element)
				}
			}
		})
	}
}

impl<N: AstNode> Iterator for AstSeparatedListElementsIterator<N> {
	type Item = AstSeparatedElement<N>;

	fn next(&mut self) -> Option<Self::Item> {
		let node_or_token = self.next_non_trivia_or_error()?;

		let element = match node_or_token {
			// The node for this element is missing if the next child is a token instead of a node.
			NodeOrToken::Token(token) => panic!(
				"Missing element in separated list, found {:?} token instead",
				token
			),
			NodeOrToken::Node(node) => {
				let separator = self.next.find_map(|element| {
					match element {
						NodeOrToken::Node(_) => panic!("Expected separator but found node. Two nodes must always be separated by a separator token."),
						NodeOrToken::Token(token) => if token.kind().is_trivia() {
							None
						} else {
							Some(token)
						}
					}
				});

				AstSeparatedElement {
					node: node.to::<N>(),
					trailing_separator: separator,
				}
			}
		};

		Some(element)
	}
}

impl<N: AstNode> FusedIterator for AstSeparatedListElementsIterator<N> {}

#[derive(Debug, Clone)]
pub struct AstSeparatedListNodesIterator<N> {
	inner: AstSeparatedListElementsIterator<N>,
}

impl<N: AstNode> Iterator for AstSeparatedListNodesIterator<N> {
	type Item = N;
	fn next(&mut self) -> Option<Self::Item> {
		Some(self.inner.next()?.node)
	}
}

impl<N: AstNode> FusedIterator for AstSeparatedListNodesIterator<N> {}

impl<N: AstNode> IntoIterator for AstSeparatedList<N> {
	type Item = N;
	type IntoIter = AstSeparatedListNodesIterator<N>;

	fn into_iter(self) -> Self::IntoIter {
		self.iter()
	}
}

impl<N: AstNode> IntoIterator for &AstSeparatedList<N> {
	type Item = N;
	type IntoIter = AstSeparatedListNodesIterator<N>;

	fn into_iter(self) -> Self::IntoIter {
		self.iter()
	}
}

/// Specific result used when navigating nodes using AST APIs
pub type SyntaxResult<ResultType> = Result<ResultType, SyntaxError>;

#[derive(Debug)]
pub enum SyntaxError {
	/// Error thrown when a mandatory node is not found
	MissingRequiredChild(SyntaxNode),
}

mod support {
	use super::{
		AstNode, AstNodeList, AstSeparatedList, SyntaxElementChildren, SyntaxKind, SyntaxNode,
		SyntaxToken,
	};
	use crate::ast::AstChildren;
	use crate::{SyntaxError, SyntaxResult};
	use crate::{SyntaxList, SyntaxNodeExt};
	use rome_rowan::NodeOrToken;

	#[deprecated]
	pub(super) fn child<N: AstNode>(parent: &SyntaxNode) -> Option<N> {
		parent.children().find_map(N::cast)
	}

	pub(super) fn nth_child<N: AstNode>(parent: &SyntaxNode, index: u32) -> Option<N> {
		parent.element_in_slot(index).map(|element| match element {
			NodeOrToken::Node(node) => node.to::<N>(),
			NodeOrToken::Token(token) => panic!(
				"Expected node {} but found token {:?}",
				std::any::type_name::<N>(),
				token
			),
		})
	}

	pub(super) fn nth_required_child<N: AstNode>(
		parent: &SyntaxNode,
		index: u32,
	) -> SyntaxResult<N> {
		nth_child(parent, index).ok_or_else(|| SyntaxError::MissingRequiredChild(parent.clone()))
	}

	pub(super) fn nth_token(parent: &SyntaxNode, index: u32) -> Option<SyntaxToken> {
		parent.element_in_slot(index).map(|element| match element {
			NodeOrToken::Token(token) => token,
			NodeOrToken::Node(node) => panic!("Expected token but found node {:?}", node),
		})
	}

	pub(super) fn nth_required_token(parent: &SyntaxNode, index: u32) -> SyntaxResult<SyntaxToken> {
		nth_token(parent, index).ok_or_else(|| SyntaxError::MissingRequiredChild(parent.clone()))
	}

	fn nth_syntax_list(parent: &SyntaxNode, index: u32) -> SyntaxList {
		parent
			.element_in_slot(index)
			.map(|element| {
				element
					.clone()
					.into_list()
					.unwrap_or_else(|| panic!("Expected list node but found element {:?}", element))
			})
			.unwrap_or_else(SyntaxList::default)
	}

	pub(super) fn nth_node_list<N: AstNode>(parent: &SyntaxNode, index: u32) -> AstNodeList<N> {
		AstNodeList::new(nth_syntax_list(parent, index))
	}

	pub(super) fn nth_separated_list<N: AstNode>(
		parent: &SyntaxNode,
		index: u32,
	) -> AstSeparatedList<N> {
		AstSeparatedList::new(nth_syntax_list(parent, index))
	}

	#[deprecated]
	pub(super) fn as_optional_node<N: AstNode>(parent: &SyntaxNode) -> Option<N> {
		parent.children().find_map(N::cast)
	}

	pub(super) fn elements(parent: &SyntaxNode) -> SyntaxElementChildren {
		parent.children_with_tokens()
	}

	#[deprecated]
	pub(super) fn children<N: AstNode>(parent: &SyntaxNode) -> AstChildren<N> {
		AstChildren::new(parent)
	}

	#[deprecated]
	fn syntax_list(parent: &SyntaxNode) -> SyntaxList {
		// TODO 1724 change parser to insert a missing for empty lists. Gracefully handle this here.
		parent
			.children()
			.filter_map(|node| node.into_list())
			.next()
			.unwrap_or_default()
	}

	#[allow(deprecated)]
	#[deprecated]
	pub(super) fn node_list<N: AstNode>(parent: &SyntaxNode) -> AstNodeList<N> {
		AstNodeList::new(syntax_list(parent))
	}

	#[allow(deprecated)]
	#[deprecated]
	pub(super) fn separated_list<N: AstNode>(parent: &SyntaxNode) -> AstSeparatedList<N> {
		AstSeparatedList::new(syntax_list(parent))
	}

	#[deprecated]
	pub(super) fn token(parent: &SyntaxNode, kind: SyntaxKind) -> Option<SyntaxToken> {
		parent
			.children_with_tokens()
			.filter_map(|it| it.into_token())
			.find(|it| it.kind() == kind)
	}

	#[deprecated]
	pub(super) fn as_optional_token(parent: &SyntaxNode, kind: SyntaxKind) -> Option<SyntaxToken> {
		parent
			.children_with_tokens()
			.filter_map(|it| it.into_token())
			.find(|it| it.kind() == kind)
	}

	#[deprecated]
	pub(super) fn as_mandatory_node<N: AstNode>(parent: &SyntaxNode) -> SyntaxResult<N> {
		parent
			.children()
			.find_map(N::cast)
			.ok_or_else(|| SyntaxError::MissingRequiredChild(parent.clone()))
	}

	#[deprecated]
	pub(super) fn as_mandatory_token(
		parent: &SyntaxNode,
		kind: SyntaxKind,
	) -> SyntaxResult<SyntaxToken> {
		parent
			.children_with_tokens()
			.filter_map(|it| it.into_token())
			.find(|it| it.kind() == kind)
			.ok_or_else(|| SyntaxError::MissingRequiredChild(parent.clone()))
	}

	#[deprecated]
	pub(super) fn find_token(
		parent: &SyntaxNode,
		possible_kinds: &[SyntaxKind],
	) -> Option<SyntaxToken> {
		parent
			.children_with_tokens()
			.filter_map(|it| it.into_token())
			.find(|it| {
				possible_kinds
					.iter()
					.any(|possible_kind| *possible_kind == it.kind())
			})
	}
}

#[cfg(test)]
mod tests {
	use crate::ast::{AstSeparatedElement, AstSeparatedList, Literal};
	use crate::{JsLanguage, SyntaxKind};
	use rome_rowan::TreeBuilder;

	/// Creates a ast separated list over a sequence of numbers separated by ",".
	/// The elements are pairs of: (value, separator).
	fn build_list<'a>(
		elements: impl IntoIterator<Item = (Option<i32>, Option<&'a str>)>,
	) -> AstSeparatedList<Literal> {
		let mut builder: TreeBuilder<JsLanguage> = TreeBuilder::new();

		builder.start_node(SyntaxKind::LIST);

		for (node, separator) in elements.into_iter() {
			if let Some(node) = node {
				builder.start_node(SyntaxKind::LITERAL);
				builder.token(SyntaxKind::NUMBER, node.to_string().as_str());
				builder.finish_node();
			}

			if let Some(separator) = separator {
				builder.token(SyntaxKind::COMMA, separator);
			}
		}

		builder.finish_node();

		let node = builder.finish();

		AstSeparatedList::new(node.into_list().unwrap())
	}

	fn assert_elements<'a>(
		actual: impl Iterator<Item = AstSeparatedElement<Literal>>,
		expected: impl IntoIterator<Item = (f64, Option<&'a str>)>,
	) {
		let actual = actual.map(|element| {
			(
				element.node.as_number().unwrap(),
				element
					.trailing_separator
					.map(|separator| separator.text().to_string()),
			)
		});

		let expected = expected
			.into_iter()
			.map(|(value, separator)| (value, separator.map(|sep| sep.to_string())))
			.collect::<Vec<_>>();

		assert_eq!(actual.collect::<Vec<_>>(), expected);
	}

	fn assert_nodes(
		actual: impl Iterator<Item = Literal>,
		expected: impl IntoIterator<Item = f64>,
	) {
		assert_eq!(
			actual
				.map(|literal| literal.as_number().unwrap())
				.collect::<Vec<_>>(),
			expected.into_iter().collect::<Vec<_>>()
		);
	}

	#[test]
	fn empty() {
		let list = build_list(vec![]);

		assert_eq!(list.len(), 0);
		assert!(list.is_empty());
		assert_eq!(list.separators().count(), 0);

		assert_nodes(list.iter(), vec![]);
		assert_elements(list.elements(), vec![]);
		assert_eq!(list.trailing_separator(), None);
	}

	#[test]
	fn separated_list() {
		let list = build_list(vec![
			(Some(1), Some(",")),
			(Some(2), Some(",")),
			(Some(3), Some(",")),
			(Some(4), None),
		]);

		assert_eq!(list.len(), 4);
		assert!(!list.is_empty());
		assert_eq!(list.separators().count(), 3);

		assert_nodes(list.iter(), vec![1., 2., 3., 4.]);
		assert_elements(
			list.elements(),
			vec![
				(1., Some(",")),
				(2., Some(",")),
				(3., Some(",")),
				(4., None),
			],
		);
		assert_eq!(list.trailing_separator(), None);
	}

	#[test]
	fn separated_with_trailing() {
		// list(1, 2, 3, 4,)
		let list = build_list(vec![
			(Some(1), Some(",")),
			(Some(2), Some(",")),
			(Some(3), Some(",")),
			(Some(4), Some(",")),
		]);

		assert_eq!(list.len(), 4);
		assert!(!list.is_empty());
		assert_nodes(list.iter(), vec![1., 2., 3., 4.]);
		assert_eq!(list.separators().count(), 4);

		assert_elements(
			list.elements(),
			vec![
				(1., Some(",")),
				(2., Some(",")),
				(3., Some(",")),
				(4., Some(",")),
			],
		);
		assert!(list.trailing_separator().is_some());
	}

	#[test]
	#[should_panic(
		expected = "Missing element in separated list, found COMMA@2..3 \",\" token instead"
	)]
	fn separated_with_two_successive_separators() {
		// list([1,,])
		let list = build_list(vec![(Some(1), Some(",")), (None, Some(","))]);

		// This should panic because having two successive separators is invalid.
		// Grammars should instead model a "hole" node if this is a valid language construct.
		let _ = list.elements().collect::<Vec<_>>();
	}

	#[test]
	#[should_panic(
		expected = "Missing element in separated list, found COMMA@0..1 \",\" token instead"
	)]
	fn separated_with_leading_separator() {
		// list([,3])
		let list = build_list(vec![(None, Some(",")), (Some(3), None)]);

		// This should panic because the first element is a separator instead of a node.
		let _ = list.elements().collect::<Vec<_>>();
	}

	#[test]
	#[should_panic(
		expected = "Expected separator but found node. Two nodes must always be separated by a separator token."
	)]
	fn separated_with_two_successive_nodes() {
		// list([1 2,])
		let list = build_list(vec![(Some(1), None), (Some(2), Some(","))]);

		// This should panic because having two successive nodes is invalid.
		let _ = list.elements().collect::<Vec<_>>();
	}
}
