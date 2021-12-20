mod parsed_children;
mod raw_syntax;

use crate::SyntaxKind;
use std::fmt;
use std::iter::{FusedIterator, Peekable};

pub use self::parsed_children::{
	ParsedChildren, ParsedChildrenIntoIterator, ParsedChildrenIterator,
};
pub use self::raw_syntax::{
	RawSyntaxElement, RawSyntaxElementRef, RawSyntaxNode, RawSyntaxNodeRef, RawSyntaxToken,
	RawSyntaxTokenRef,
};

pub trait SyntaxFactory: fmt::Debug {
	type Kind: SyntaxKind;

	fn make_syntax(
		kind: Self::Kind,
		children: ParsedChildren<Self::Kind>,
	) -> RawSyntaxNode<Self::Kind>;

	fn make_node_list_syntax<F>(
		kind: Self::Kind,
		children: ParsedChildren<Self::Kind>,
		can_cast: F,
	) -> RawSyntaxNode<Self::Kind>
	where
		F: Fn(Self::Kind) -> bool,
	{
		let valid = (&children)
			.into_iter()
			.all(|element| can_cast(element.kind()));

		let kind = if valid { kind } else { kind.to_unknown() };

		RawSyntaxNode::new(kind, children.into_iter().map(Some))
	}

	fn make_separated_list_syntax<F>(
		kind: Self::Kind,
		children: ParsedChildren<Self::Kind>,
		can_cast: F,
		separator: Self::Kind,
		allow_trailing: bool,
	) -> RawSyntaxNode<Self::Kind>
	where
		F: Fn(Self::Kind) -> bool,
	{
		let mut next_node = true;
		let mut missing_count = 0;
		let mut valid = true;

		for child in &children {
			let kind = child.kind();

			if next_node {
				if can_cast(kind) {
					next_node = false;
				} else if kind == separator {
					// a missing element
					missing_count += 1;
				} else {
					// an invalid element
					valid = false;
					break;
				}
			} else if kind == separator {
				next_node = true;
			} else if can_cast(kind) {
				// a missing separator
				missing_count += 1;
			} else {
				// something unexpected
				valid = false;
			}
		}

		if next_node && !allow_trailing && !children.is_empty() {
			// a trailing comma in a list that doesn't support trailing commas
			missing_count += 1;
		}

		if !valid {
			RawSyntaxNode::new(kind.to_unknown(), children.into_iter().map(Some))
		} else if missing_count > 0 {
			RawSyntaxNode::new(
				kind,
				SeparatedListWithMissingNodesOrSeparatorSlotsIterator {
					inner: children.into_iter().peekable(),
					missing_count,
					next_node: true,
					separator,
				},
			)
		} else {
			RawSyntaxNode::new(kind, children.into_iter().map(Some))
		}
	}
}

struct SeparatedListWithMissingNodesOrSeparatorSlotsIterator<'a, K: SyntaxKind> {
	inner: Peekable<ParsedChildrenIntoIterator<'a, K>>,
	missing_count: usize,
	next_node: bool,
	separator: K,
}

impl<'a, K: SyntaxKind> Iterator for SeparatedListWithMissingNodesOrSeparatorSlotsIterator<'a, K> {
	type Item = Option<RawSyntaxElement<K>>;

	#[cold]
	fn next(&mut self) -> Option<Self::Item> {
		let peeked = self.inner.peek();

		if let Some(peeked) = peeked {
			let is_separator = self.separator == peeked.kind();

			if self.next_node {
				self.next_node = false;
				if !is_separator {
					Some(self.inner.next())
				} else {
					self.missing_count -= 1;
					Some(None) // Missing separator
				}
			} else if is_separator {
				self.next_node = true;
				Some(self.inner.next())
			} else {
				// Missing node
				self.missing_count -= 1;
				self.next_node = true;
				Some(None)
			}
		} else if self.missing_count > 0 {
			// at a trailing comma in a list that doesn't allow trailing commas.
			self.missing_count -= 1;
			Some(None)
		} else {
			None
		}
	}

	fn size_hint(&self) -> (usize, Option<usize>) {
		let len = self.len();
		(len, Some(len))
	}
}

impl<'a, K: SyntaxKind> FusedIterator
	for SeparatedListWithMissingNodesOrSeparatorSlotsIterator<'a, K>
{
}

impl<'a, K: SyntaxKind> ExactSizeIterator
	for SeparatedListWithMissingNodesOrSeparatorSlotsIterator<'a, K>
{
	fn len(&self) -> usize {
		self.inner.len() + self.missing_count
	}
}

pub struct UnknownNodeChildrenIterator<'a, K: SyntaxKind, const COUNT: usize> {
	layout_elements: std::array::IntoIter<Option<RawSyntaxElement<K>>, COUNT>,
	original_len: usize,
	current_element: Option<RawSyntaxElement<K>>,
	remaining_elements: ParsedChildrenIntoIterator<'a, K>,
}

impl<'a, K: SyntaxKind, const COUNT: usize> UnknownNodeChildrenIterator<'a, K, COUNT> {
	pub fn new(
		slots: [Option<RawSyntaxElement<K>>; COUNT],
		original_len: usize,
		current_element: RawSyntaxElement<K>,
		remaining_elements: ParsedChildrenIntoIterator<'a, K>,
	) -> Self {
		Self {
			layout_elements: slots.into_iter(),
			original_len,
			current_element: Some(current_element),
			remaining_elements,
		}
	}
}

impl<'a, K: SyntaxKind, const COUNT: usize> Iterator for UnknownNodeChildrenIterator<'a, K, COUNT> {
	type Item = Option<RawSyntaxElement<K>>;

	#[cold]
	fn next(&mut self) -> Option<Self::Item> {
		for element in self.layout_elements.by_ref() {
			if element.is_some() {
				self.original_len -= 1;
				return Some(element);
			}
		}

		if let Some(current_element) = self.current_element.take() {
			self.original_len -= 1;
			Some(Some(current_element))
		} else {
			let next = self.remaining_elements.next()?;
			self.original_len -= 1;
			Some(Some(next))
		}
	}

	fn size_hint(&self) -> (usize, Option<usize>) {
		let len = self.len();

		(len, Some(len))
	}
}

impl<'a, K: SyntaxKind, const COUNT: usize> ExactSizeIterator
	for UnknownNodeChildrenIterator<'a, K, COUNT>
{
	fn len(&self) -> usize {
		self.original_len
	}
}
