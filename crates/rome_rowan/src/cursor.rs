//! Implementation of the cursors -- API for convenient access to syntax trees.
//!
//! Functional programmers will recognize that this module implements a zipper
//! for a purely functional (green) tree.
//!
//! A cursor node (`SyntaxNode`) points to a `GreenNode` and a parent
//! `SyntaxNode`. This allows cursor to provide iteration over both ancestors
//! and descendants, as well as a cheep access to absolute offset of the node in
//! file.
//!
//! By default `SyntaxNode`s are immutable, but you can get a mutable copy of
//! the tree by calling `clone_for_update`. Mutation is based on interior
//! mutability and doesn't need `&mut`. You can have two `SyntaxNode`s pointing
//! at different parts of the same tree; mutations via the first node will be
//! reflected in the other.

// Implementation notes:
//
// The implementation is utterly and horribly unsafe. This whole module is an
// unsafety boundary. It is believed that the API here is, in principle, sound,
// but the implementation might have bugs.
//
// The core type is `NodeData` -- a heap-allocated reference counted object,
// which points to a green node or a green token, and to the parent `NodeData`.
// Publicly-exposed `SyntaxNode` and `SyntaxToken` own a reference to
// `NodeData`.
//
// `NodeData`s are transient, and are created and destroyed during tree
// traversals. In general, only currently referenced nodes and their ancestors
// are alive at any given moment.
//
// More specifically, `NodeData`'s ref count is equal to the number of
// outstanding `SyntaxNode` and `SyntaxToken` plus the number of children with
// non-zero ref counts. For example, if the user has only a single `SyntaxNode`
// pointing somewhere in the middle of the tree, then all `NodeData` on the path
// from that point towards the root have ref count equal to one.
//
// `NodeData` which doesn't have a parent (is a root) owns the corresponding
// green node or token, and is responsible for freeing it.
//
// That's mostly it for the immutable subset of the API. Mutation is fun though,
// you'll like it!
//
// Mutability is a run-time property of a tree of `NodeData`. The whole tree is
// either mutable or immutable. `clone_for_update` clones the whole tree of
// `NodeData`s, making it mutable (note that the green tree is re-used).
//
// If the tree is mutable, then all live `NodeData` are additionally liked to
// each other via intrusive liked lists. Specifically, there are two pointers to
// siblings, as well as a pointer to the first child. Note that only live nodes
// are considered. If the user only has `SyntaxNode`s for  the first and last
// children of some particular node, then their `NodeData` will point at each
// other.
//
// The links are used to propagate mutations across the tree. Specifically, each
// `NodeData` remembers it's index in parent. When the node is detached from or
// attached to the tree, we need to adjust the indices of all subsequent
// siblings. That's what makes the `for c in node.children() { c.detach() }`
// pattern work despite the apparent iterator invalidation.
//
// This code is encapsulated into the sorted linked list (`sll`) module.
//
// The actual mutation consist of functionally "mutating" (creating a
// structurally shared copy) the green node, and then re-spinning the tree. This
// is a delicate process: `NodeData` point directly to the green nodes, so we
// must make sure that those nodes don't move. Additionally, during mutation a
// node might become or might stop being a root, so we must take care to not
// double free / leak its green node.
//
// Because we can change green nodes using only shared references, handing out
// references into green nodes in the public API would be unsound. We don't do
// that, but we do use such references internally a lot. Additionally, for
// tokens the underlying green token actually is immutable, so we can, and do
// return `&str`.
//
// Invariants [must not leak outside of the module]:
//    - Mutability is the property of the whole tree. Intermixing elements that
//      differ in mutability is not allowed.
//    - Mutability property is persistent.
//    - References to the green elements' data are not exposed into public API
//      when the tree is mutable.
//    - TBD

use std::{
	borrow::Cow,
	cell::Cell,
	fmt,
	hash::{Hash, Hasher},
	iter,
	mem::{self, ManuallyDrop},
	ops::Range,
	ptr,
};

use countme::Count;

use crate::green::{Child, Children};
use crate::{
	green::{GreenElementRef, GreenNodeData, GreenTokenData, SyntaxKind},
	sll,
	utility_types::Delta,
	Direction, GreenNode, GreenToken, NodeOrToken, SyntaxText, TextRange, TextSize, TokenAtOffset,
	WalkEvent,
};

enum Green {
	Node {
		ptr: Cell<ptr::NonNull<GreenNodeData>>,
	},
	Token {
		ptr: ptr::NonNull<GreenTokenData>,
	},
}

struct _SyntaxElement;

struct NodeData {
	_c: Count<_SyntaxElement>,

	rc: Cell<u32>,
	parent: Cell<Option<ptr::NonNull<NodeData>>>,
	slot: Cell<u32>,
	green: Green,

	/// Invariant: never changes after NodeData is created.
	mutable: bool,
	/// Absolute offset for immutable nodes, unused for mutable nodes.
	offset: TextSize,
	// The following links only have meaning when `mutable` is true.
	first: Cell<*const NodeData>,
	/// Invariant: never null if mutable.
	next: Cell<*const NodeData>,
	/// Invariant: never null if mutable.
	prev: Cell<*const NodeData>,
}

unsafe impl sll::Elem for NodeData {
	fn prev(&self) -> &Cell<*const Self> {
		&self.prev
	}
	fn next(&self) -> &Cell<*const Self> {
		&self.next
	}
	fn key(&self) -> &Cell<u32> {
		&self.slot
	}
}

pub(crate) type SyntaxElement = NodeOrToken<SyntaxNode, SyntaxToken>;

pub(crate) struct SyntaxTrivia {
	offset: TextSize,
	token: SyntaxToken,
	is_leading: bool,
}

impl Clone for SyntaxTrivia {
	#[inline]
	fn clone(&self) -> Self {
		Self {
			offset: self.offset,
			token: self.token.clone(),
			is_leading: self.is_leading,
		}
	}
}

pub(crate) struct SyntaxNode {
	ptr: ptr::NonNull<NodeData>,
}

impl Clone for SyntaxNode {
	#[inline]
	fn clone(&self) -> Self {
		self.data().inc_rc();
		SyntaxNode { ptr: self.ptr }
	}
}

impl Drop for SyntaxNode {
	#[inline]
	fn drop(&mut self) {
		if self.data().dec_rc() {
			unsafe { free(self.ptr) }
		}
	}
}

#[derive(Debug)]
pub(crate) struct SyntaxToken {
	ptr: ptr::NonNull<NodeData>,
}

impl Clone for SyntaxToken {
	#[inline]
	fn clone(&self) -> Self {
		self.data().inc_rc();
		SyntaxToken { ptr: self.ptr }
	}
}

impl Drop for SyntaxToken {
	#[inline]
	fn drop(&mut self) {
		if self.data().dec_rc() {
			unsafe { free(self.ptr) }
		}
	}
}

#[inline(never)]
unsafe fn free(mut data: ptr::NonNull<NodeData>) {
	loop {
		debug_assert_eq!(data.as_ref().rc.get(), 0);
		debug_assert!(data.as_ref().first.get().is_null());
		let node = Box::from_raw(data.as_ptr());
		match node.parent.take() {
			Some(parent) => {
				debug_assert!(parent.as_ref().rc.get() > 0);
				if node.mutable {
					sll::unlink(&parent.as_ref().first, &*node)
				}
				if parent.as_ref().dec_rc() {
					data = parent;
				} else {
					break;
				}
			}
			None => {
				match &node.green {
					Green::Node { ptr } => {
						let _ = GreenNode::from_raw(ptr.get());
					}
					Green::Token { ptr } => {
						let _ = GreenToken::from_raw(*ptr);
					}
				}
				break;
			}
		}
	}
}

impl NodeData {
	#[inline]
	fn new(
		parent: Option<SyntaxNode>,
		slot: u32,
		offset: TextSize,
		green: Green,
		mutable: bool,
	) -> ptr::NonNull<NodeData> {
		let parent = ManuallyDrop::new(parent);
		let res = NodeData {
			_c: Count::new(),
			rc: Cell::new(1),
			parent: Cell::new(parent.as_ref().map(|it| it.ptr)),
			slot: Cell::new(slot),
			green,

			mutable,
			offset,
			first: Cell::new(ptr::null()),
			next: Cell::new(ptr::null()),
			prev: Cell::new(ptr::null()),
		};
		unsafe {
			if mutable {
				let res_ptr: *const NodeData = &res;
				match sll::init(
					(*res_ptr).parent().map(|it| &it.first),
					res_ptr.as_ref().unwrap(),
				) {
					sll::AddToSllResult::AlreadyInSll(node) => {
						if cfg!(debug_assertions) {
							assert_eq!((*node).slot(), (*res_ptr).slot());
							match ((*node).green(), (*res_ptr).green()) {
								(NodeOrToken::Node(lhs), NodeOrToken::Node(rhs)) => {
									assert!(ptr::eq(lhs, rhs))
								}
								(NodeOrToken::Token(lhs), NodeOrToken::Token(rhs)) => {
									assert!(ptr::eq(lhs, rhs))
								}
								it => {
									panic!("node/token confusion: {:?}", it)
								}
							}
						}

						ManuallyDrop::into_inner(parent);
						let res = node as *mut NodeData;
						(*res).inc_rc();
						return ptr::NonNull::new_unchecked(res);
					}
					it => {
						let res = Box::into_raw(Box::new(res));
						it.add_to_sll(res);
						return ptr::NonNull::new_unchecked(res);
					}
				}
			}
			ptr::NonNull::new_unchecked(Box::into_raw(Box::new(res)))
		}
	}

	#[inline]
	fn inc_rc(&self) {
		let rc = match self.rc.get().checked_add(1) {
			Some(it) => it,
			None => std::process::abort(),
		};
		self.rc.set(rc)
	}

	#[inline]
	fn dec_rc(&self) -> bool {
		let rc = self.rc.get() - 1;
		self.rc.set(rc);
		rc == 0
	}

	#[inline]
	fn key(&self) -> (ptr::NonNull<()>, TextSize) {
		let ptr = match &self.green {
			Green::Node { ptr } => ptr.get().cast(),
			Green::Token { ptr } => ptr.cast(),
		};
		(ptr, self.offset())
	}

	#[inline]
	fn parent_node(&self) -> Option<SyntaxNode> {
		let parent = self.parent()?;
		debug_assert!(matches!(parent.green, Green::Node { .. }));
		parent.inc_rc();
		Some(SyntaxNode {
			ptr: ptr::NonNull::from(parent),
		})
	}

	#[inline]
	fn parent(&self) -> Option<&NodeData> {
		self.parent.get().map(|it| unsafe { &*it.as_ptr() })
	}

	#[inline]
	fn green(&self) -> GreenElementRef<'_> {
		match &self.green {
			Green::Node { ptr } => GreenElementRef::Node(unsafe { &*ptr.get().as_ptr() }),
			Green::Token { ptr } => GreenElementRef::Token(unsafe { &*ptr.as_ref() }),
		}
	}

	/// Returns an iterator over the siblings of this node. The iterator is positioned at the current node.
	#[inline]
	fn green_siblings(&self) -> Option<Siblings> {
		match &self.parent()?.green {
			Green::Node { ptr } => {
				let parent = unsafe { &*ptr.get().as_ptr() };

				Some(Siblings::new(parent, self.slot()))
			}
			Green::Token { .. } => {
				debug_assert!(
					false,
					"A token should never be a parent of a token or node."
				);
				None
			}
		}
	}
	#[inline]
	fn slot(&self) -> u32 {
		self.slot.get()
	}

	#[inline]
	fn offset(&self) -> TextSize {
		if self.mutable {
			self.offset_mut()
		} else {
			self.offset
		}
	}

	#[cold]
	fn offset_mut(&self) -> TextSize {
		let mut res = TextSize::from(0);

		let mut node = self;
		while let Some(parent) = node.parent() {
			let green = parent.green().into_node().unwrap();
			res += green
				.slots()
				.nth(node.slot() as usize)
				.unwrap()
				.rel_offset();
			node = parent;
		}

		res
	}

	#[inline]
	fn text_range(&self) -> TextRange {
		let offset = self.offset();
		let len = self.green().text_len();
		TextRange::at(offset, len)
	}

	#[inline]
	fn kind(&self) -> SyntaxKind {
		self.green().kind()
	}

	fn next_sibling(&self) -> Option<SyntaxNode> {
		let siblings = self.green_siblings()?;
		siblings.following().find_map(|child| {
			child.element().into_node().and_then(|green| {
				let parent = self.parent_node()?;
				let offset = parent.offset() + child.rel_offset();
				Some(SyntaxNode::new_child(green, parent, child.slot(), offset))
			})
		})
	}
	fn prev_sibling(&self) -> Option<SyntaxNode> {
		let siblings = self.green_siblings()?;
		siblings.previous().find_map(|child| {
			child.element().into_node().and_then(|green| {
				let parent = self.parent_node()?;
				let offset = parent.offset() + child.rel_offset();
				Some(SyntaxNode::new_child(green, parent, child.slot(), offset))
			})
		})
	}

	fn next_sibling_or_token(&self) -> Option<SyntaxElement> {
		let siblings = self.green_siblings()?;

		siblings.following().next().and_then(|child| {
			let parent = self.parent_node()?;
			let offset = parent.offset() + child.rel_offset();
			Some(SyntaxElement::new(
				child.element(),
				parent,
				child.slot(),
				offset,
			))
		})
	}
	fn prev_sibling_or_token(&self) -> Option<SyntaxElement> {
		let siblings = self.green_siblings()?;

		siblings.previous().next().and_then(|child| {
			let parent = self.parent_node()?;
			let offset = parent.offset() + child.rel_offset();
			Some(SyntaxElement::new(
				child.element(),
				parent,
				child.slot(),
				offset,
			))
		})
	}

	fn detach(&self) {
		assert!(self.mutable);
		assert!(self.rc.get() > 0);
		let parent_ptr = match self.parent.take() {
			Some(parent) => parent,
			None => return,
		};

		unsafe {
			sll::adjust(self, self.slot() + 1, Delta::Sub(1));
			let parent = parent_ptr.as_ref();
			sll::unlink(&parent.first, self);

			// Add strong ref to green
			match self.green().to_owned() {
				NodeOrToken::Node(it) => {
					GreenNode::into_raw(it);
				}
				NodeOrToken::Token(it) => {
					GreenToken::into_raw(it);
				}
			}

			match parent.green() {
				NodeOrToken::Node(green) => {
					let green = green.remove_slot(self.slot() as usize);
					parent.respine(green)
				}
				NodeOrToken::Token(_) => unreachable!(),
			}

			if parent.dec_rc() {
				free(parent_ptr)
			}
		}
	}
	fn attach_child(&self, index: usize, child: &NodeData) {
		assert!(self.mutable && child.mutable && child.parent().is_none());
		assert!(self.rc.get() > 0 && child.rc.get() > 0);

		unsafe {
			child.slot.set(index as u32);
			child.parent.set(Some(self.into()));
			self.inc_rc();

			if !self.first.get().is_null() {
				sll::adjust(&*self.first.get(), index as u32, Delta::Add(1));
			}

			match sll::link(&self.first, child) {
				sll::AddToSllResult::AlreadyInSll(_) => {
					panic!("Child already in sorted linked list")
				}
				it => it.add_to_sll(child),
			}

			match self.green() {
				NodeOrToken::Node(green) => {
					// Child is root, so it owns the green node. Steal it!
					let child_green = match &child.green {
						Green::Node { ptr } => GreenNode::from_raw(ptr.get()).into(),
						Green::Token { ptr } => GreenToken::from_raw(*ptr).into(),
					};

					let green = green.insert_slot(index, Some(child_green));
					self.respine(green);
				}
				NodeOrToken::Token(_) => unreachable!(),
			}
		}
	}
	unsafe fn respine(&self, mut new_green: GreenNode) {
		let mut node = self;
		loop {
			let old_green = match &node.green {
				Green::Node { ptr } => ptr.replace(ptr::NonNull::from(&*new_green)),
				Green::Token { .. } => unreachable!(),
			};
			match node.parent() {
				Some(parent) => match parent.green() {
					NodeOrToken::Node(parent_green) => {
						new_green = parent_green
							.replace_child(node.slot() as usize, Some(new_green.into()));
						node = parent;
					}
					_ => unreachable!(),
				},
				None => {
					mem::forget(new_green);
					let _ = GreenNode::from_raw(old_green);
					break;
				}
			}
		}
	}
}

impl SyntaxTrivia {
	pub(crate) fn leading_range(&self) -> TextRange {
		let len = self.token.green().leading().text_len();
		TextRange::at(self.offset, len)
	}

	pub(crate) fn trailing_range(&self) -> TextRange {
		let len = self.token.green().trailing().text_len();
		TextRange::at(self.offset, len)
	}

	pub(crate) fn text(&self) -> &str {
		if self.is_leading {
			self.token.green().text_leading()
		} else {
			self.token.green().text_trailing()
		}
	}

	pub(crate) fn text_len(&self) -> TextSize {
		if self.is_leading {
			self.token.green().leading().text_len()
		} else {
			self.token.green().trailing().text_len()
		}
	}
}

impl SyntaxNode {
	pub(crate) fn new_root(green: GreenNode) -> SyntaxNode {
		let green = GreenNode::into_raw(green);
		let green = Green::Node {
			ptr: Cell::new(green),
		};
		SyntaxNode {
			ptr: NodeData::new(None, 0, 0.into(), green, false),
		}
	}

	pub(crate) fn new_root_mut(green: GreenNode) -> SyntaxNode {
		let green = GreenNode::into_raw(green);
		let green = Green::Node {
			ptr: Cell::new(green),
		};
		SyntaxNode {
			ptr: NodeData::new(None, 0, 0.into(), green, true),
		}
	}

	fn new_child(
		green: &GreenNodeData,
		parent: SyntaxNode,
		slot: u32,
		offset: TextSize,
	) -> SyntaxNode {
		let mutable = parent.data().mutable;
		let green = Green::Node {
			ptr: Cell::new(green.into()),
		};
		SyntaxNode {
			ptr: NodeData::new(Some(parent), slot, offset, green, mutable),
		}
	}

	pub fn clone_for_update(&self) -> SyntaxNode {
		assert!(!self.data().mutable);
		match self.parent() {
			Some(parent) => {
				let parent = parent.clone_for_update();
				SyntaxNode::new_child(self.green_ref(), parent, self.data().slot(), self.offset())
			}
			None => SyntaxNode::new_root_mut(self.green_ref().to_owned()),
		}
	}

	pub fn clone_subtree(&self) -> SyntaxNode {
		SyntaxNode::new_root(self.green().into())
	}

	#[inline]
	fn data(&self) -> &NodeData {
		unsafe { self.ptr.as_ref() }
	}

	#[inline]
	pub fn kind(&self) -> SyntaxKind {
		self.data().kind()
	}

	#[inline]
	fn offset(&self) -> TextSize {
		self.data().offset()
	}

	pub(crate) fn element_in_slot(&self, slot_index: u32) -> Option<SyntaxElement> {
		let slot = self
			.green_ref()
			.slots()
			.nth(slot_index as usize)
			.expect("Slot index out of bounds");

		slot.as_ref().map(|element| {
			SyntaxElement::new(
				element,
				self.clone(),
				slot_index,
				self.offset() + slot.rel_offset(),
			)
		})
	}

	#[inline]
	pub fn text_range(&self) -> TextRange {
		self.data().text_range()
	}

	#[inline]
	pub fn index(&self) -> usize {
		self.data().slot() as usize
	}

	#[inline]
	pub fn text(&self) -> SyntaxText {
		SyntaxText::new(self.clone())
	}

	#[inline]
	pub(crate) fn green(&self) -> Cow<'_, GreenNodeData> {
		let green_ref = self.green_ref();
		match self.data().mutable {
			false => Cow::Borrowed(green_ref),
			true => Cow::Owned(green_ref.to_owned()),
		}
	}
	#[inline]
	fn green_ref(&self) -> &GreenNodeData {
		self.data().green().into_node().unwrap()
	}

	#[inline]
	pub fn parent(&self) -> Option<SyntaxNode> {
		self.data().parent_node()
	}

	#[inline]
	pub fn ancestors(&self) -> impl Iterator<Item = SyntaxNode> {
		iter::successors(Some(self.clone()), SyntaxNode::parent)
	}

	#[inline]
	pub fn children(&self) -> SyntaxNodeChildren {
		SyntaxNodeChildren::new(self.clone())
	}

	#[inline]
	pub fn children_with_tokens(&self) -> SyntaxElementChildren {
		SyntaxElementChildren::new(self.clone())
	}

	pub fn first_child(&self) -> Option<SyntaxNode> {
		self.green_ref().children().find_map(|child| {
			child.element().into_node().map(|green| {
				SyntaxNode::new_child(
					green,
					self.clone(),
					child.slot() as u32,
					self.offset() + child.rel_offset(),
				)
			})
		})
	}
	pub fn last_child(&self) -> Option<SyntaxNode> {
		self.green_ref().children().rev().find_map(|child| {
			child.element().into_node().map(|green| {
				SyntaxNode::new_child(
					green,
					self.clone(),
					child.slot(),
					self.offset() + child.rel_offset(),
				)
			})
		})
	}

	pub fn first_child_or_token(&self) -> Option<SyntaxElement> {
		self.green_ref().children().next().map(|child| {
			SyntaxElement::new(
				child.element(),
				self.clone(),
				child.slot(),
				self.offset() + child.rel_offset(),
			)
		})
	}
	pub fn last_child_or_token(&self) -> Option<SyntaxElement> {
		self.green_ref().children().next_back().map(|child| {
			SyntaxElement::new(
				child.element(),
				self.clone(),
				child.slot(),
				self.offset() + child.rel_offset(),
			)
		})
	}

	pub fn next_sibling(&self) -> Option<SyntaxNode> {
		self.data().next_sibling()
	}
	pub fn prev_sibling(&self) -> Option<SyntaxNode> {
		self.data().prev_sibling()
	}

	pub fn next_sibling_or_token(&self) -> Option<SyntaxElement> {
		self.data().next_sibling_or_token()
	}
	pub fn prev_sibling_or_token(&self) -> Option<SyntaxElement> {
		self.data().prev_sibling_or_token()
	}

	pub fn first_token(&self) -> Option<SyntaxToken> {
		self.first_child_or_token()?.first_token()
	}
	pub fn last_token(&self) -> Option<SyntaxToken> {
		self.last_child_or_token()?.last_token()
	}

	#[inline]
	pub fn siblings(&self, direction: Direction) -> impl Iterator<Item = SyntaxNode> {
		iter::successors(Some(self.clone()), move |node| match direction {
			Direction::Next => node.next_sibling(),
			Direction::Prev => node.prev_sibling(),
		})
	}

	#[inline]
	pub fn siblings_with_tokens(
		&self,
		direction: Direction,
	) -> impl Iterator<Item = SyntaxElement> {
		let me: SyntaxElement = self.clone().into();
		iter::successors(Some(me), move |el| match direction {
			Direction::Next => el.next_sibling_or_token(),
			Direction::Prev => el.prev_sibling_or_token(),
		})
	}

	#[inline]
	pub fn descendants(&self) -> impl Iterator<Item = SyntaxNode> {
		self.preorder().filter_map(|event| match event {
			WalkEvent::Enter(node) => Some(node),
			WalkEvent::Leave(_) => None,
		})
	}

	#[inline]
	pub fn descendants_with_tokens(&self) -> impl Iterator<Item = SyntaxElement> {
		self.preorder_with_tokens().filter_map(|event| match event {
			WalkEvent::Enter(it) => Some(it),
			WalkEvent::Leave(_) => None,
		})
	}

	#[inline]
	pub fn preorder(&self) -> Preorder {
		Preorder::new(self.clone())
	}

	#[inline]
	pub fn preorder_with_tokens(&self) -> PreorderWithTokens {
		PreorderWithTokens::new(self.clone())
	}

	pub fn token_at_offset(&self, offset: TextSize) -> TokenAtOffset<SyntaxToken> {
		// TODO: this could be faster if we first drill-down to node, and only
		// then switch to token search. We should also replace explicit
		// recursion with a loop.
		let range = self.text_range();
		assert!(
			range.start() <= offset && offset <= range.end(),
			"Bad offset: range {:?} offset {:?}",
			range,
			offset
		);
		if range.is_empty() {
			return TokenAtOffset::None;
		}

		let mut children = self.children_with_tokens().filter(|child| {
			let child_range = child.text_range();
			!child_range.is_empty()
				&& (child_range.start() <= offset && offset <= child_range.end())
		});

		let left = children.next().unwrap();
		let right = children.next();
		assert!(children.next().is_none());

		if let Some(right) = right {
			match (left.token_at_offset(offset), right.token_at_offset(offset)) {
				(TokenAtOffset::Single(left), TokenAtOffset::Single(right)) => {
					TokenAtOffset::Between(left, right)
				}
				_ => unreachable!(),
			}
		} else {
			left.token_at_offset(offset)
		}
	}

	pub fn covering_element(&self, range: TextRange) -> SyntaxElement {
		let mut res: SyntaxElement = self.clone().into();
		loop {
			assert!(
				res.text_range().contains_range(range),
				"Bad range: node range {:?}, range {:?}",
				res.text_range(),
				range,
			);
			res = match &res {
				NodeOrToken::Token(_) => return res,
				NodeOrToken::Node(node) => match node.child_or_token_at_range(range) {
					Some(it) => it,
					None => return res,
				},
			};
		}
	}

	pub fn child_or_token_at_range(&self, range: TextRange) -> Option<SyntaxElement> {
		let rel_range = range - self.offset();
		self.green_ref()
			.slot_at_range(rel_range)
			.and_then(|(index, rel_offset, slot)| {
				slot.as_ref().map(|green| {
					SyntaxElement::new(
						green,
						self.clone(),
						index as u32,
						self.offset() + rel_offset,
					)
				})
			})
	}

	pub fn splice_children(&self, to_delete: Range<usize>, to_insert: Vec<SyntaxElement>) {
		assert!(self.data().mutable, "immutable tree: {}", self);
		for (i, child) in self.children_with_tokens().enumerate() {
			if to_delete.contains(&i) {
				child.detach();
			}
		}
		let mut index = to_delete.start;
		for child in to_insert {
			self.attach_child(index, child);
			index += 1;
		}
	}

	pub fn detach(&self) {
		assert!(self.data().mutable, "immutable tree: {}", self);
		self.data().detach()
	}

	fn attach_child(&self, index: usize, child: SyntaxElement) {
		assert!(self.data().mutable, "immutable tree: {}", self);
		child.detach();
		let data = match &child {
			NodeOrToken::Node(it) => it.data(),
			NodeOrToken::Token(it) => it.data(),
		};
		self.data().attach_child(index, data)
	}
}

impl SyntaxToken {
	fn new(
		green: &GreenTokenData,
		parent: SyntaxNode,
		index: u32,
		offset: TextSize,
	) -> SyntaxToken {
		let mutable = parent.data().mutable;
		let green = Green::Token { ptr: green.into() };
		SyntaxToken {
			ptr: NodeData::new(Some(parent), index, offset, green, mutable),
		}
	}

	#[inline]
	fn green(&self) -> &GreenTokenData {
		match self.data().green().as_token() {
			Some(token) => token,
			None => {
				panic!(
					"corrupted tree: a node thinks it is a token: {:?}",
					self.data().green().as_node().unwrap().to_string()
				);
			}
		}
	}

	#[inline]
	fn data(&self) -> &NodeData {
		unsafe { self.ptr.as_ref() }
	}

	#[inline]
	pub fn kind(&self) -> SyntaxKind {
		self.data().kind()
	}

	#[inline]
	pub fn text_range(&self) -> TextRange {
		self.data().text_range()
	}

	#[inline]
	pub fn text_trimmed_range(&self) -> TextRange {
		let g = self.green();
		let leading_len = g.leading().text_len();
		let trailing_len = g.trailing().text_len();

		let range = self.text_range();
		TextRange::new(range.start() + leading_len, range.end() - trailing_len)
	}

	#[inline]
	pub fn leading_range(&self) -> TextRange {
		let g = self.green();
		let leading_len = g.leading().text_len();

		let range = self.text_range();
		let start = range.start();
		TextRange::new(start, start + leading_len)
	}

	#[inline]
	pub fn trailing_range(&self) -> TextRange {
		let g = self.green();
		let trailing_len = g.trailing().text_len();

		let range = self.text_range();
		let end = range.end();
		TextRange::new(end - trailing_len, end)
	}

	#[inline]
	pub fn index(&self) -> usize {
		self.data().slot() as usize
	}

	#[inline]
	pub fn text(&self) -> &str {
		self.green().text()
	}

	#[inline]
	pub fn text_trimmed(&self) -> &str {
		self.green().text_trimmed()
	}

	#[inline]
	pub fn parent(&self) -> Option<SyntaxNode> {
		self.data().parent_node()
	}

	#[inline]
	pub fn ancestors(&self) -> impl Iterator<Item = SyntaxNode> {
		std::iter::successors(self.parent(), SyntaxNode::parent)
	}

	pub fn next_sibling_or_token(&self) -> Option<SyntaxElement> {
		self.data().next_sibling_or_token()
	}
	pub fn prev_sibling_or_token(&self) -> Option<SyntaxElement> {
		self.data().prev_sibling_or_token()
	}

	#[inline]
	pub fn siblings_with_tokens(
		&self,
		direction: Direction,
	) -> impl Iterator<Item = SyntaxElement> {
		let next = move |el: &SyntaxElement| match direction {
			Direction::Next => el.next_sibling_or_token(),
			Direction::Prev => el.prev_sibling_or_token(),
		};

		let me: SyntaxElement = self.clone().into();

		iter::successors(next(&me), next)
	}

	pub fn next_token(&self) -> Option<SyntaxToken> {
		match self.next_sibling_or_token() {
			Some(element) => element.first_token(),
			None => self
				.ancestors()
				.find_map(|it| it.next_sibling_or_token())
				.and_then(|element| element.first_token()),
		}
	}
	pub fn prev_token(&self) -> Option<SyntaxToken> {
		match self.prev_sibling_or_token() {
			Some(element) => element.last_token(),
			None => self
				.ancestors()
				.find_map(|it| it.prev_sibling_or_token())
				.and_then(|element| element.last_token()),
		}
	}

	pub fn detach(&self) {
		assert!(self.data().mutable, "immutable tree: {}", self);
		self.data().detach()
	}

	#[inline]
	pub fn leading(&self) -> SyntaxTrivia {
		SyntaxTrivia {
			offset: self.data().offset,
			token: self.clone(),
			is_leading: true,
		}
	}

	#[inline]
	pub fn trailing(&self) -> SyntaxTrivia {
		SyntaxTrivia {
			offset: self.data().offset,
			token: self.clone(),
			is_leading: false,
		}
	}
}

impl SyntaxElement {
	fn new(
		element: GreenElementRef<'_>,
		parent: SyntaxNode,
		slot: u32,
		offset: TextSize,
	) -> SyntaxElement {
		match element {
			NodeOrToken::Node(node) => {
				SyntaxNode::new_child(node, parent, slot as u32, offset).into()
			}
			NodeOrToken::Token(token) => {
				SyntaxToken::new(token, parent, slot as u32, offset).into()
			}
		}
	}

	#[inline]
	pub fn text_range(&self) -> TextRange {
		match self {
			NodeOrToken::Node(it) => it.text_range(),
			NodeOrToken::Token(it) => it.text_range(),
		}
	}

	#[inline]
	pub fn index(&self) -> usize {
		match self {
			NodeOrToken::Node(it) => it.index(),
			NodeOrToken::Token(it) => it.index(),
		}
	}

	#[inline]
	pub fn kind(&self) -> SyntaxKind {
		match self {
			NodeOrToken::Node(it) => it.kind(),
			NodeOrToken::Token(it) => it.kind(),
		}
	}

	#[inline]
	pub fn parent(&self) -> Option<SyntaxNode> {
		match self {
			NodeOrToken::Node(it) => it.parent(),
			NodeOrToken::Token(it) => it.parent(),
		}
	}

	#[inline]
	pub fn ancestors(&self) -> impl Iterator<Item = SyntaxNode> {
		let first = match self {
			NodeOrToken::Node(it) => Some(it.clone()),
			NodeOrToken::Token(it) => it.parent(),
		};
		iter::successors(first, SyntaxNode::parent)
	}

	pub fn first_token(&self) -> Option<SyntaxToken> {
		match self {
			NodeOrToken::Node(it) => it.first_token(),
			NodeOrToken::Token(it) => Some(it.clone()),
		}
	}
	pub fn last_token(&self) -> Option<SyntaxToken> {
		match self {
			NodeOrToken::Node(it) => it.last_token(),
			NodeOrToken::Token(it) => Some(it.clone()),
		}
	}

	pub fn next_sibling_or_token(&self) -> Option<SyntaxElement> {
		match self {
			NodeOrToken::Node(it) => it.next_sibling_or_token(),
			NodeOrToken::Token(it) => it.next_sibling_or_token(),
		}
	}
	pub fn prev_sibling_or_token(&self) -> Option<SyntaxElement> {
		match self {
			NodeOrToken::Node(it) => it.prev_sibling_or_token(),
			NodeOrToken::Token(it) => it.prev_sibling_or_token(),
		}
	}

	fn token_at_offset(&self, offset: TextSize) -> TokenAtOffset<SyntaxToken> {
		assert!(self.text_range().start() <= offset && offset <= self.text_range().end());
		match self {
			NodeOrToken::Token(token) => TokenAtOffset::Single(token.clone()),
			NodeOrToken::Node(node) => node.token_at_offset(offset),
		}
	}

	pub fn detach(&self) {
		match self {
			NodeOrToken::Node(it) => it.detach(),
			NodeOrToken::Token(it) => it.detach(),
		}
	}
}

// region: impls

impl PartialEq for SyntaxTrivia {
	#[inline]
	fn eq(&self, other: &Self) -> bool {
		self.token == other.token && self.is_leading == other.is_leading
	}
}

impl Eq for SyntaxTrivia {}

impl Hash for SyntaxTrivia {
	#[inline]
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.token.hash(state);
		self.is_leading.hash(state);
	}
}

impl fmt::Debug for SyntaxTrivia {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let mut f = f.debug_struct("SyntaxTrivia");
		if self.is_leading {
			f.field("text_range", &self.leading_range());
		} else {
			f.field("text_range", &self.trailing_range());
		}

		f.finish()
	}
}

impl fmt::Display for SyntaxTrivia {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		fmt::Display::fmt(self.text(), f)
	}
}

// Identity semantics for hash & eq
impl PartialEq for SyntaxNode {
	#[inline]
	fn eq(&self, other: &SyntaxNode) -> bool {
		self.data().key() == other.data().key()
	}
}

impl Eq for SyntaxNode {}

impl Hash for SyntaxNode {
	#[inline]
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.data().key().hash(state);
	}
}

impl fmt::Debug for SyntaxNode {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_struct("SyntaxNode")
			.field("kind", &self.kind())
			.field("text_range", &self.text_range())
			.finish()
	}
}

impl fmt::Display for SyntaxNode {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		self.preorder_with_tokens()
			.filter_map(|event| match event {
				WalkEvent::Enter(NodeOrToken::Token(token)) => Some(token),
				_ => None,
			})
			.try_for_each(|it| fmt::Display::fmt(&it, f))
	}
}

// Identity semantics for hash & eq
impl PartialEq for SyntaxToken {
	#[inline]
	fn eq(&self, other: &SyntaxToken) -> bool {
		self.data().key() == other.data().key()
	}
}

impl Eq for SyntaxToken {}

impl Hash for SyntaxToken {
	#[inline]
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.data().key().hash(state);
	}
}

impl fmt::Display for SyntaxToken {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		fmt::Display::fmt(self.text(), f)
	}
}

impl From<SyntaxNode> for SyntaxElement {
	#[inline]
	fn from(node: SyntaxNode) -> SyntaxElement {
		NodeOrToken::Node(node)
	}
}

impl From<SyntaxToken> for SyntaxElement {
	#[inline]
	fn from(token: SyntaxToken) -> SyntaxElement {
		NodeOrToken::Token(token)
	}
}

// endregion

// region: iterators

#[derive(Clone, Debug)]
pub(crate) struct SyntaxNodeChildren {
	next: Option<SyntaxNode>,
}

impl SyntaxNodeChildren {
	fn new(parent: SyntaxNode) -> SyntaxNodeChildren {
		SyntaxNodeChildren {
			next: parent.first_child(),
		}
	}
}

impl Iterator for SyntaxNodeChildren {
	type Item = SyntaxNode;
	fn next(&mut self) -> Option<SyntaxNode> {
		self.next.take().map(|next| {
			self.next = next.next_sibling();
			next
		})
	}
}

#[derive(Clone, Debug, Default)]
pub(crate) struct SyntaxElementChildren {
	next: Option<SyntaxElement>,
}

impl SyntaxElementChildren {
	fn new(parent: SyntaxNode) -> SyntaxElementChildren {
		SyntaxElementChildren {
			next: parent.first_child_or_token(),
		}
	}
}

impl Iterator for SyntaxElementChildren {
	type Item = SyntaxElement;
	fn next(&mut self) -> Option<SyntaxElement> {
		self.next.take().map(|next| {
			self.next = next.next_sibling_or_token();
			next
		})
	}
}

pub(crate) struct Preorder {
	start: SyntaxNode,
	next: Option<WalkEvent<SyntaxNode>>,
	skip_subtree: bool,
}

impl Preorder {
	fn new(start: SyntaxNode) -> Preorder {
		let next = Some(WalkEvent::Enter(start.clone()));
		Preorder {
			start,
			next,
			skip_subtree: false,
		}
	}

	pub fn skip_subtree(&mut self) {
		self.skip_subtree = true;
	}

	#[cold]
	fn do_skip(&mut self) {
		self.next = self.next.take().map(|next| match next {
			WalkEvent::Enter(first_child) => WalkEvent::Leave(first_child.parent().unwrap()),
			WalkEvent::Leave(parent) => WalkEvent::Leave(parent),
		})
	}
}

impl Iterator for Preorder {
	type Item = WalkEvent<SyntaxNode>;

	fn next(&mut self) -> Option<WalkEvent<SyntaxNode>> {
		if self.skip_subtree {
			self.do_skip();
			self.skip_subtree = false;
		}
		let next = self.next.take();
		self.next = next.as_ref().and_then(|next| {
			Some(match next {
				WalkEvent::Enter(node) => match node.first_child() {
					Some(child) => WalkEvent::Enter(child),
					None => WalkEvent::Leave(node.clone()),
				},
				WalkEvent::Leave(node) => {
					if node == &self.start {
						return None;
					}
					match node.next_sibling() {
						Some(sibling) => WalkEvent::Enter(sibling),
						None => WalkEvent::Leave(node.parent()?),
					}
				}
			})
		});
		next
	}
}

pub(crate) struct PreorderWithTokens {
	start: SyntaxElement,
	next: Option<WalkEvent<SyntaxElement>>,
	skip_subtree: bool,
}

impl PreorderWithTokens {
	fn new(start: SyntaxNode) -> PreorderWithTokens {
		let next = Some(WalkEvent::Enter(start.clone().into()));
		PreorderWithTokens {
			start: start.into(),
			next,
			skip_subtree: false,
		}
	}

	pub fn skip_subtree(&mut self) {
		self.skip_subtree = true;
	}

	#[cold]
	fn do_skip(&mut self) {
		self.next = self.next.take().map(|next| match next {
			WalkEvent::Enter(first_child) => WalkEvent::Leave(first_child.parent().unwrap().into()),
			WalkEvent::Leave(parent) => WalkEvent::Leave(parent),
		})
	}
}

impl Iterator for PreorderWithTokens {
	type Item = WalkEvent<SyntaxElement>;

	fn next(&mut self) -> Option<WalkEvent<SyntaxElement>> {
		if self.skip_subtree {
			self.do_skip();
			self.skip_subtree = false;
		}
		let next = self.next.take();
		self.next = next.as_ref().and_then(|next| {
			Some(match next {
				WalkEvent::Enter(el) => match el {
					NodeOrToken::Node(node) => match node.first_child_or_token() {
						Some(child) => WalkEvent::Enter(child),
						None => WalkEvent::Leave(node.clone().into()),
					},
					NodeOrToken::Token(token) => WalkEvent::Leave(token.clone().into()),
				},
				WalkEvent::Leave(el) if el == &self.start => return None,
				WalkEvent::Leave(el) => match el.next_sibling_or_token() {
					Some(sibling) => WalkEvent::Enter(sibling),
					None => WalkEvent::Leave(el.parent()?.into()),
				},
			})
		});
		next
	}
}

#[derive(Debug, Clone)]
pub(crate) struct Siblings<'a> {
	parent: &'a GreenNodeData,
	start_slot: u32,
}

impl<'a> Siblings<'a> {
	pub fn new(parent: &'a GreenNodeData, start_slot: u32) -> Self {
		assert!(
			(start_slot as usize) < parent.slots().len(),
			"Start slot {} out of bounds {}",
			start_slot,
			parent.slots().len()
		);

		Self { parent, start_slot }
	}

	/// Creates an iterator over the siblings following the start node.
	/// For example, the following siblings of the if statement's condition are
	/// * the consequence
	/// * potentially the else clause
	pub fn following(&self) -> Children<'a> {
		let mut slots = self.parent.slots().enumerate();

		// Navigate to the start slot so that calling `next` returns the first following sibling
		slots.nth(self.start_slot as usize);

		Children::new(slots)
	}

	/// Creates an iterator over the siblings preceding the start node in reverse order.
	/// For example, the preceding siblings of the if statement's condition are:
	/// * opening parentheses: (
	/// * if keyword: if
	pub fn previous(&self) -> impl Iterator<Item = Child<'a>> {
		let mut slots = self.parent.slots().enumerate();

		// Navigate to the start slot from the back so that calling `next_back` (or rev().next()) returns
		// the first slot preceding the start node
		slots.nth_back(slots.len() - 1 - self.start_slot as usize);

		Children::new(slots).rev()
	}
}

// endregion
