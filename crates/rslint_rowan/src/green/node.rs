use std::{
	hash::{Hash, Hasher},
	iter::FusedIterator,
	slice,
};

use crate::arc::{Arc, HeaderSlice, HeaderWithLength, ThinArc};
use fxhash::FxHasher32;

use crate::{
	green::{GreenElement, GreenElementRef, PackedGreenElement, SyntaxKind},
	TextSize,
};

#[repr(align(2))] // NB: this is an at-least annotation
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(super) struct GreenNodeHead {
	kind: SyntaxKind,
	text_len: TextSize,
	child_hash: u32,
}

impl GreenNodeHead {
	#[inline]
	pub(super) fn from_child_slice(kind: SyntaxKind, children: &[GreenElement]) -> Self {
		let mut hasher = FxHasher32::default();
		let mut text_len: TextSize = 0.into();
		for child in children {
			text_len += child.text_len();
			child.hash(&mut hasher);
		}
		Self {
			kind,
			text_len,
			child_hash: hasher.finish() as u32,
		}
	}
}

/// Internal node in the immutable tree.
/// It has other nodes and tokens as children.
#[derive(Clone, PartialEq, Eq)]
pub struct GreenNode {
	pub(super) data: ThinArc<GreenNodeHead, PackedGreenElement>,
}

impl std::fmt::Debug for GreenNode {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		self.data.with_arc(|data| data.fmt(f))
	}
}

impl GreenNode {
	/// Creates new Node.
	#[inline]
	pub fn new<I>(kind: SyntaxKind, children: I) -> GreenNode
	where
		I: IntoIterator<Item = GreenElement>,
		I::IntoIter: ExactSizeIterator,
	{
		let mut hasher = FxHasher32::default();
		let mut text_len: TextSize = 0.into();
		let children = children
			.into_iter()
			.inspect(|it| {
				text_len += it.text_len();
				it.hash(&mut hasher);
			})
			.map(PackedGreenElement::from);
		let header = HeaderWithLength::new(
			GreenNodeHead {
				kind,
				text_len: 0.into(),
				child_hash: 0,
			},
			children.len(),
		);
		let mut data = Arc::from_header_and_iter(header, children);

		// XXX: fixup `text_len` and `child_hash` after construction, because
		// we can't iterate `children` twice.
		let header = &mut Arc::get_mut(&mut data).unwrap().header.header;
		header.text_len = text_len;
		header.child_hash = hasher.finish() as u32;
		GreenNode {
			data: Arc::into_thin(data),
		}
	}

	#[inline]
	pub(super) fn from_head_and_children<I>(header: GreenNodeHead, children: I) -> GreenNode
	where
		I: IntoIterator<Item = GreenElement>,
		I::IntoIter: ExactSizeIterator,
	{
		let children = children.into_iter().map(PackedGreenElement::from);
		let header = HeaderWithLength::new(header, children.len());
		GreenNode {
			data: Arc::into_thin(Arc::from_header_and_iter(header, children)),
		}
	}

	/// Kind of this node.
	#[inline]
	pub fn kind(&self) -> SyntaxKind {
		self.data.header.header.kind
	}

	/// Returns the length of the text covered by this node.
	#[inline]
	pub fn text_len(&self) -> TextSize {
		self.data.header.header.text_len
	}

	/// Children of this node.
	#[inline]
	pub fn children(&self) -> Children<'_> {
		Children {
			inner: self.data.slice.iter(),
		}
	}

	#[inline]
	pub(crate) fn ptr(&self) -> *const u8 {
		let r: &HeaderSlice<_, _> = &self.data;
		r as *const _ as _
	}

	/// Tests if this and the passed in node point to the same underlying data (pointer comparison)
	///
	/// # Examples
	///
	/// Returns true for the same node
	/// ```
	/// use rslint_rowan::{GreenNode, SyntaxKind};
	///
	/// let node = GreenNode::new(SyntaxKind(1), vec![]);
	///
	/// assert!(node.shallow_eq(&node))
	/// ```
	///
	/// Returns true for cloned nodes
	/// ```
	/// use rslint_rowan::{GreenNode, SyntaxKind};
	///
	/// let node = GreenNode::new(SyntaxKind(1), vec![]);
	/// let node_2 = node.clone(); // points to the same underlying data
	///
	/// assert!(node.shallow_eq(&node_2));
	/// assert!(node_2.shallow_eq(&node));
	/// ```
	///
	/// Returns `false` for nodes that are structurally equal but were created independently
	/// ```
	/// use rslint_rowan::{GreenNode, SyntaxKind};
	///
	/// let node = GreenNode::new(SyntaxKind(1), vec![]);
	/// let node_2 = GreenNode::new(SyntaxKind(1), vec![]);
	///
	/// // The nodes' structures are equal
	/// assert_eq!(node, node_2);
	///
	/// // but they point to different underlying data structures, which is why they are not shallow equal
	/// assert!(!node.shallow_eq(&node_2));
	/// assert!(!node_2.shallow_eq(&node));
	/// ```
	///
	#[inline]
	pub fn shallow_eq(&self, other: &GreenNode) -> bool {
		self.ptr() == other.ptr()
	}
}

impl Hash for GreenNode {
	#[inline]
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.data.header.header.hash(state);
	}
}

#[derive(Debug, Clone)]
pub struct Children<'a> {
	inner: slice::Iter<'a, PackedGreenElement>,
}

// NB: forward everything stable that iter::Slice specializes as of Rust 1.39.0
impl ExactSizeIterator for Children<'_> {
	#[inline(always)]
	fn len(&self) -> usize {
		self.inner.len()
	}
}

impl<'a> Iterator for Children<'a> {
	type Item = GreenElementRef<'a>;

	#[inline]
	fn next(&mut self) -> Option<GreenElementRef<'a>> {
		self.inner.next().map(PackedGreenElement::as_ref)
	}

	#[inline]
	fn size_hint(&self) -> (usize, Option<usize>) {
		self.inner.size_hint()
	}

	#[inline]
	fn count(self) -> usize
	where
		Self: Sized,
	{
		self.inner.count()
	}

	#[inline]
	fn nth(&mut self, n: usize) -> Option<Self::Item> {
		self.inner.nth(n).map(PackedGreenElement::as_ref)
	}

	#[inline]
	fn last(mut self) -> Option<Self::Item>
	where
		Self: Sized,
	{
		self.next_back()
	}

	#[inline]
	fn fold<Acc, Fold>(mut self, init: Acc, mut f: Fold) -> Acc
	where
		Fold: FnMut(Acc, Self::Item) -> Acc,
	{
		let mut accum = init;
		while let Some(x) = self.next() {
			accum = f(accum, x);
		}
		accum
	}
}

impl<'a> DoubleEndedIterator for Children<'a> {
	#[inline]
	fn next_back(&mut self) -> Option<Self::Item> {
		self.inner.next_back().map(PackedGreenElement::as_ref)
	}

	#[inline]
	fn nth_back(&mut self, n: usize) -> Option<Self::Item> {
		self.inner.nth_back(n).map(PackedGreenElement::as_ref)
	}

	#[inline]
	fn rfold<Acc, Fold>(mut self, init: Acc, mut f: Fold) -> Acc
	where
		Fold: FnMut(Acc, Self::Item) -> Acc,
	{
		let mut accum = init;
		while let Some(x) = self.next_back() {
			accum = f(accum, x);
		}
		accum
	}
}

impl FusedIterator for Children<'_> {}
