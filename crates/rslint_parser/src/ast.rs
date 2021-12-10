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
use std::error::Error;
use std::fmt::{Debug, Formatter};
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
		self.syntax().text_trimmed().to_string()
	}

	fn range(&self) -> TextRange {
		self.syntax().text_trimmed_range()
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
#[derive(Clone)]
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

impl<N: AstNode + Debug> Debug for AstNodeList<N> {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		f.debug_list().entries(self.iter()).finish()
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
		self.iter().next()
	}

	/// Returns the last node from this list or None
	pub fn last(&self) -> Option<N> {
		self.iter().last()
	}

	#[inline]
	pub fn is_empty(&self) -> bool {
		self.inner.is_empty()
	}
}

#[derive(Debug, Clone)]
pub struct AstNodeListIterator<N> {
	inner: SyntaxSlots,
	ph: PhantomData<N>,
}

impl<N: AstNode> AstNodeListIterator<N> {
	fn slot_to_node(slot: &SyntaxSlot) -> N {
		match slot {
			SyntaxSlot::Empty => panic!("Node isn't permitted to contain empty slots"),
			SyntaxSlot::Node(node) => node.to(),
			SyntaxSlot::Token(token) => panic!(
				"Expected node of type `{:?}` but found token `{:?}` instead.",
				std::any::type_name::<N>(),
				token
			),
		}
	}
}

impl<N: AstNode> Iterator for AstNodeListIterator<N> {
	type Item = N;

	fn next(&mut self) -> Option<Self::Item> {
		Some(Self::slot_to_node(&self.inner.next()?))
	}

	fn last(self) -> Option<Self::Item>
	where
		Self: Sized,
	{
		Some(Self::slot_to_node(&self.inner.last()?))
	}

	fn nth(&mut self, n: usize) -> Option<Self::Item> {
		Some(Self::slot_to_node(&self.inner.nth(n)?))
	}
}

impl<N: AstNode> ExactSizeIterator for AstNodeListIterator<N> {
	fn len(&self) -> usize {
		self.inner.len()
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

#[derive(Clone)]
pub struct AstSeparatedElement<N> {
	node: SyntaxResult<N>,
	trailing_separator: SyntaxResult<Option<SyntaxToken>>,
}

impl<N: AstNode + Clone> AstSeparatedElement<N> {
	pub fn node(&self) -> SyntaxResult<N> {
		self.node.clone()
	}

	pub fn trailing_separator(&self) -> SyntaxResult<Option<SyntaxToken>> {
		self.trailing_separator.clone()
	}
}

impl<N: Debug> Debug for AstSeparatedElement<N> {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match &self.node {
			Ok(node) => N::fmt(node, f)?,
			Err(_) => f.write_str("missing element")?,
		};
		match &self.trailing_separator {
			Ok(Some(separator)) => {
				f.write_str(",\n")?;
				separator.fmt(f)
			}
			Err(_) => f.write_str(",\nmissing separator"),
			Ok(None) => Ok(()),
		}
	}
}

/// List of nodes where every two nodes are separated by a token.
/// For example, the elements of an array where every two elements are separated by a comma token.
/// The list expects that the underlying syntax node has a slot for every node and separator
/// even if they are missing from the source code. For example, a list for `a b` where the `,` separator
/// is missing contains the slots `Node(a), Empty, Node(b)`. This also applies for missing nodes:
/// the list for `, b,` must have the slots `Empty, Token(,), Node(b), Token(,)`.
#[derive(Clone)]
pub struct AstSeparatedList<N> {
	list: SyntaxList,
	ph: PhantomData<N>,
}

impl<N: AstNode + Debug> Debug for AstSeparatedList<N> {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		f.debug_list().entries(self.elements()).finish()
	}
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
		AstSeparatedListElementsIterator::new(&self.list)
	}

	/// Returns an iterator over all separator tokens
	pub fn separators(&self) -> impl Iterator<Item = SyntaxResult<SyntaxToken>> {
		self.elements()
			.filter_map(|element| match element.trailing_separator {
				Ok(Some(separator)) => Some(Ok(separator)),
				Err(missing) => Some(Err(missing)),
				_ => None,
			})
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
		(self.list.len() + 1) / 2
	}

	pub fn trailing_separator(&self) -> Option<SyntaxToken> {
		match self.list.last()? {
			SyntaxSlot::Token(token) => Some(token),
			_ => None,
		}
	}
}

#[derive(Debug, Clone)]
pub struct AstSeparatedListElementsIterator<N> {
	slots: SyntaxSlots,
	parent: Option<SyntaxNode>,
	ph: PhantomData<N>,
}

impl<N: AstNode> AstSeparatedListElementsIterator<N> {
	fn new(list: &SyntaxList) -> Self {
		Self {
			slots: list.iter(),
			parent: list.node().cloned(),
			ph: PhantomData,
		}
	}
}

impl<N: AstNode> Iterator for AstSeparatedListElementsIterator<N> {
	type Item = AstSeparatedElement<N>;

	fn next(&mut self) -> Option<Self::Item> {
		let slot = self.slots.next()?;

		let node = match slot {
			// The node for this element is missing if the next child is a token instead of a node.
			SyntaxSlot::Token(token) => panic!("Malformed list, node expected but found token {:?} instead. You must add missing markers for missing elements.", token),
			// Missing element
			SyntaxSlot::Empty => Err(SyntaxError::MissingRequiredChild(
					self.parent.as_ref().unwrap().clone(),
				)),
			SyntaxSlot::Node(node) => Ok(node.to::<N>())
		};

		let separator = match self.slots.next() {
			Some(SyntaxSlot::Empty) => Err(
				SyntaxError::MissingRequiredChild(self.parent.as_ref().unwrap().clone()),
			),
			Some(SyntaxSlot::Token(token)) => Ok(Some(token)),
			// End of list, no trailing separator
			None => Ok(None),
			Some(SyntaxSlot::Node(node)) => panic!("Malformed separated list, separator expected but found node {:?} instead. You must add missing markers for missing separators.", node),
		};

		Some(AstSeparatedElement {
			node,
			trailing_separator: separator,
		})
	}
}

impl<N: AstNode> FusedIterator for AstSeparatedListElementsIterator<N> {}

#[derive(Debug, Clone)]
pub struct AstSeparatedListNodesIterator<N> {
	inner: AstSeparatedListElementsIterator<N>,
}

impl<N: AstNode> Iterator for AstSeparatedListNodesIterator<N> {
	type Item = SyntaxResult<N>;
	fn next(&mut self) -> Option<Self::Item> {
		self.inner.next().map(|element| element.node)
	}
}

impl<N: AstNode> FusedIterator for AstSeparatedListNodesIterator<N> {}

impl<N: AstNode> IntoIterator for AstSeparatedList<N> {
	type Item = SyntaxResult<N>;
	type IntoIter = AstSeparatedListNodesIterator<N>;

	fn into_iter(self) -> Self::IntoIter {
		self.iter()
	}
}

impl<N: AstNode> IntoIterator for &AstSeparatedList<N> {
	type Item = SyntaxResult<N>;
	type IntoIter = AstSeparatedListNodesIterator<N>;

	fn into_iter(self) -> Self::IntoIter {
		self.iter()
	}
}

/// Specific result used when navigating nodes using AST APIs
pub type SyntaxResult<ResultType> = Result<ResultType, SyntaxError>;

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum SyntaxError {
	/// Error thrown when a mandatory node is not found
	MissingRequiredChild(SyntaxNode),
}

impl Error for SyntaxError {}

impl std::fmt::Display for SyntaxError {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			SyntaxError::MissingRequiredChild(_) => write!(f, "missing required child"),
		}
	}
}

mod support {
	use super::{AstNode, AstNodeList, AstSeparatedList, SyntaxKind, SyntaxNode, SyntaxToken};
	use crate::ast::{AnyNode, AstChildren};
	use crate::{SyntaxElement, SyntaxElementChildren, SyntaxList, SyntaxNodeExt};
	use crate::{SyntaxError, SyntaxResult};
	use std::fmt::{Debug, Formatter};

	pub(super) fn node<N: AstNode>(parent: &SyntaxNode) -> Option<N> {
		parent.children().find_map(N::cast)
	}

	pub(super) fn required_node<N: AstNode>(parent: &SyntaxNode) -> SyntaxResult<N> {
		node(parent).ok_or_else(|| SyntaxError::MissingRequiredChild(parent.clone()))
	}

	pub(super) fn elements(parent: &SyntaxNode) -> SyntaxElementChildren {
		parent.children_with_tokens()
	}

	pub(super) fn children<N: AstNode>(parent: &SyntaxNode) -> AstChildren<N> {
		AstChildren::new(parent)
	}

	fn nth_syntax_list(parent: &SyntaxNode, index: usize) -> SyntaxList {
		// TODO 1724 change parser to insert a missing for empty lists. Gracefully handle this here.
		parent
			.children()
			.filter_map(|node| node.into_list())
			.nth(index)
			.unwrap_or_default()
	}

	pub(super) fn node_list<N: AstNode>(parent: &SyntaxNode, index: usize) -> AstNodeList<N> {
		AstNodeList::new(nth_syntax_list(parent, index))
	}

	pub(super) fn separated_list<N: AstNode>(
		parent: &SyntaxNode,
		index: usize,
	) -> AstSeparatedList<N> {
		AstSeparatedList::new(nth_syntax_list(parent, index))
	}

	pub(super) fn token(parent: &SyntaxNode, kind: SyntaxKind) -> Option<SyntaxToken> {
		parent
			.children_with_tokens()
			.filter_map(|it| it.into_token())
			.find(|it| it.kind() == kind)
	}

	pub(super) fn required_token(
		parent: &SyntaxNode,
		kind: SyntaxKind,
	) -> SyntaxResult<SyntaxToken> {
		token(parent, kind).ok_or_else(|| SyntaxError::MissingRequiredChild(parent.clone()))
	}

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

	pub(super) fn find_required_token(
		parent: &SyntaxNode,
		possible_kinds: &[SyntaxKind],
	) -> SyntaxResult<SyntaxToken> {
		find_token(parent, possible_kinds)
			.ok_or_else(|| SyntaxError::MissingRequiredChild(parent.clone()))
	}

	/// New-type wrapper to flatten the debug output of syntax result fields when printing [AstNode]s.
	/// Omits the [Ok] if the node is present and prints `missing (required)` if the child is missing
	pub(super) struct DebugSyntaxResult<N>(pub(super) SyntaxResult<N>);

	impl<N: Debug> Debug for DebugSyntaxResult<N> {
		fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
			match &self.0 {
				Ok(node) => std::fmt::Debug::fmt(node, f),
				Err(SyntaxError::MissingRequiredChild(_)) => f.write_str("missing (required)"),
			}
		}
	}

	/// New-type wrapper to flatten the debug output of optional children when printing [AstNode]s.
	/// Omits the [Some] if the node is present and prints `missing (optional)` if the child is missing
	pub(super) struct DebugOptionalElement<N>(pub(super) Option<N>);

	impl<N: Debug> Debug for DebugOptionalElement<N> {
		fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
			match &self.0 {
				Some(node) => std::fmt::Debug::fmt(node, f),
				None => f.write_str("missing (optional)"),
			}
		}
	}

	struct DebugSyntaxElement(SyntaxElement);

	impl Debug for DebugSyntaxElement {
		fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
			match &self.0 {
				SyntaxElement::Node(node) => match node.kind() {
					SyntaxKind::LIST => {
						Debug::fmt(&DebugSyntaxElementChildren(node.children_with_tokens()), f)
					}
					_ => Debug::fmt(&node.to::<AnyNode>(), f),
				},
				SyntaxElement::Token(token) => Debug::fmt(token, f),
			}
		}
	}

	#[derive(Clone)]
	pub(super) struct DebugSyntaxElementChildren(pub(super) SyntaxElementChildren);

	impl Debug for DebugSyntaxElementChildren {
		fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
			f.debug_list()
				.entries(self.clone().0.map(DebugSyntaxElement))
				.finish()
		}
	}
}

#[cfg(test)]
mod tests {
	use crate::ast::{AstSeparatedElement, AstSeparatedList, JsNumberLiteralExpression};
	use crate::{JsLanguage, SyntaxKind, SyntaxResult};
	use rome_rowan::TreeBuilder;

	/// Creates a ast separated list over a sequence of numbers separated by ",".
	/// The elements are pairs of: (value, separator).
	fn build_list<'a>(
		elements: impl IntoIterator<Item = (Option<i32>, Option<&'a str>)>,
	) -> AstSeparatedList<JsNumberLiteralExpression> {
		let mut builder: TreeBuilder<JsLanguage> = TreeBuilder::new();

		builder.start_node(SyntaxKind::LIST);

		let mut had_missing_separator = false;

		for (node, separator) in elements.into_iter() {
			if had_missing_separator {
				builder.missing();
				had_missing_separator = false;
			}

			if let Some(node) = node {
				builder.start_node(SyntaxKind::JS_NUMBER_LITERAL_EXPRESSION);
				builder.token(SyntaxKind::JS_NUMBER_LITERAL, node.to_string().as_str());
				builder.finish_node();
			} else {
				builder.missing()
			}

			if let Some(separator) = separator {
				builder.token(SyntaxKind::COMMA, separator);
			} else {
				had_missing_separator = true;
			}
		}

		builder.finish_node();

		let node = builder.finish();

		AstSeparatedList::new(node.into_list().unwrap())
	}

	fn assert_elements<'a>(
		actual: impl Iterator<Item = AstSeparatedElement<JsNumberLiteralExpression>>,
		expected: impl IntoIterator<Item = (Option<f64>, Option<&'a str>)>,
	) {
		let actual = actual.map(|element| {
			(
				element.node.ok().map(|n| n.as_number().unwrap()),
				element
					.trailing_separator
					.ok()
					.flatten()
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
		actual: impl Iterator<Item = SyntaxResult<JsNumberLiteralExpression>>,
		expected: impl IntoIterator<Item = f64>,
	) {
		assert_eq!(
			actual
				.map(|literal| literal.unwrap().as_number().unwrap())
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
				(Some(1.), Some(",")),
				(Some(2.), Some(",")),
				(Some(3.), Some(",")),
				(Some(4.), None),
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
				(Some(1.), Some(",")),
				(Some(2.), Some(",")),
				(Some(3.), Some(",")),
				(Some(4.), Some(",")),
			],
		);
		assert!(list.trailing_separator().is_some());
	}

	#[test]
	fn separated_with_two_successive_separators() {
		// list([1,,])
		let list = build_list(vec![(Some(1), Some(",")), (None, Some(","))]);

		assert_eq!(list.len(), 2);
		assert!(!list.is_empty());
		assert_eq!(list.separators().count(), 2);

		assert_elements(
			list.elements(),
			vec![(Some(1.), Some(",")), (None, Some(","))],
		);
	}

	#[test]
	fn separated_with_leading_separator() {
		// list([,3])
		let list = build_list(vec![(None, Some(",")), (Some(3), None)]);

		assert_eq!(list.len(), 2);
		assert!(!list.is_empty());
		assert_eq!(list.separators().count(), 1);

		assert_elements(
			list.elements(),
			vec![
				// missing first element
				(None, Some(",")),
				(Some(3.), None),
			],
		);
	}

	#[test]
	fn separated_with_two_successive_nodes() {
		// list([1 2,])
		let list = build_list(vec![(Some(1), None), (Some(2), Some(","))]);

		assert_eq!(list.len(), 2);
		assert!(!list.is_empty());
		assert_eq!(list.separators().count(), 2);

		assert_elements(
			list.elements(),
			vec![(Some(1.), None), (Some(2.), Some(","))],
		);
	}
}
