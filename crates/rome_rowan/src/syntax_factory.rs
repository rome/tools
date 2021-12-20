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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SlotContent {
	Present,
	Absent,
}

#[derive(Debug)]
pub struct RawNodeSlots<const COUNT: usize> {
	slots: [SlotContent; COUNT],
	current_slot: usize,
}

impl<const COUNT: usize> Default for RawNodeSlots<COUNT> {
	fn default() -> Self {
		Self {
			slots: [SlotContent::Absent; COUNT],
			current_slot: 0,
		}
	}
}

impl<const COUNT: usize> RawNodeSlots<COUNT> {
	pub fn mark_absent(&mut self) {
		self.slots[self.current_slot] = SlotContent::Absent;
		self.current_slot += 1;
	}

	pub fn mark_present(&mut self) {
		self.slots[self.current_slot] = SlotContent::Present;
		self.current_slot += 1;
	}

	pub fn into_node<K: SyntaxKind>(
		self,
		kind: K,
		children: ParsedChildren<K>,
	) -> RawSyntaxNode<K> {
		RawSyntaxNode::new(
			kind,
			RawNodeSlotIterator {
				children: children.into_iter(),
				slots: self.slots.as_slice().iter(),
			},
		)
	}
}

struct RawNodeSlotIterator<'a, K: SyntaxKind> {
	children: ParsedChildrenIntoIterator<'a, K>,
	slots: std::slice::Iter<'a, SlotContent>,
}

impl<'a, K: SyntaxKind> Iterator for RawNodeSlotIterator<'a, K> {
	type Item = Option<RawSyntaxElement<K>>;

	fn next(&mut self) -> Option<Self::Item> {
		let slot = self.slots.next()?;

		match slot {
			SlotContent::Present => {
				Some(Some(self.children.next().expect(
					"Expected a present node according to the slot description",
				)))
			}
			SlotContent::Absent => Some(None),
		}
	}

	fn size_hint(&self) -> (usize, Option<usize>) {
		(self.slots.len(), Some(self.slots.len()))
	}
}

impl<'a, K: SyntaxKind> FusedIterator for RawNodeSlotIterator<'a, K> {}
impl<'a, K: SyntaxKind> ExactSizeIterator for RawNodeSlotIterator<'a, K> {}
