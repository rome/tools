use std::{fmt, hash, mem};

// NOTE: From `thin_dst`:
// This MUST be size=1 such that pointer math actually advances the pointer.
type ErasedPtr = *const u8;

use crate::{
	green::{GreenNode, GreenToken, SyntaxKind},
	NodeOrToken, TextSize,
};

pub(super) type GreenElement = NodeOrToken<GreenNode, GreenToken>;
pub(crate) type GreenElementRef<'a> = NodeOrToken<&'a GreenNode, &'a GreenToken>;

#[repr(transparent)]
pub(super) struct PackedGreenElement {
	ptr: ErasedPtr,
}

impl From<GreenNode> for GreenElement {
	#[inline]
	fn from(node: GreenNode) -> GreenElement {
		NodeOrToken::Node(node)
	}
}

impl<'a> From<&'a GreenNode> for GreenElementRef<'a> {
	#[inline]
	fn from(node: &'a GreenNode) -> GreenElementRef<'a> {
		NodeOrToken::Node(node)
	}
}

impl From<GreenNode> for PackedGreenElement {
	#[inline]
	fn from(node: GreenNode) -> PackedGreenElement {
		unsafe { mem::transmute(node) }
	}
}

impl From<GreenToken> for GreenElement {
	#[inline]
	fn from(token: GreenToken) -> GreenElement {
		NodeOrToken::Token(token)
	}
}

impl<'a> From<&'a GreenToken> for GreenElementRef<'a> {
	#[inline]
	fn from(token: &'a GreenToken) -> GreenElementRef<'a> {
		NodeOrToken::Token(token)
	}
}

impl From<GreenToken> for PackedGreenElement {
	#[inline]
	fn from(token: GreenToken) -> PackedGreenElement {
		unsafe { mem::transmute(token) }
	}
}

impl GreenElement {
	/// Returns kind of this element.
	#[inline]
	pub fn kind(&self) -> SyntaxKind {
		self.as_ref().kind()
	}

	/// Returns the length of the text covered by this element.
	#[inline]
	pub fn text_len(&self) -> TextSize {
		self.as_ref().text_len()
	}
}

impl GreenElementRef<'_> {
	/// Returns kind of this element.
	#[inline]
	pub fn kind(&self) -> SyntaxKind {
		match self {
			NodeOrToken::Node(it) => it.kind(),
			NodeOrToken::Token(it) => it.kind(),
		}
	}

	/// Returns the length of the text covered by this element.
	#[inline]
	pub fn text_len(self) -> TextSize {
		match self {
			NodeOrToken::Node(it) => it.text_len(),
			NodeOrToken::Token(it) => it.text_len(),
		}
	}
}

impl From<GreenElement> for PackedGreenElement {
	fn from(element: GreenElement) -> Self {
		match element {
			NodeOrToken::Node(node) => node.into(),
			NodeOrToken::Token(token) => token.into(),
		}
	}
}

impl From<PackedGreenElement> for GreenElement {
	fn from(element: PackedGreenElement) -> Self {
		if element.is_node() {
			NodeOrToken::Node(element.into_node().unwrap())
		} else {
			NodeOrToken::Token(element.into_token().unwrap())
		}
	}
}

impl PackedGreenElement {
	fn is_node(&self) -> bool {
		self.ptr as usize & 1 == 0
	}

	pub(crate) fn as_node(&self) -> Option<&GreenNode> {
		if self.is_node() {
			unsafe { Some(&*(&self.ptr as *const ErasedPtr as *const GreenNode)) }
		} else {
			None
		}
	}

	pub(crate) fn into_node(self) -> Option<GreenNode> {
		if self.is_node() {
			unsafe { Some(mem::transmute(self)) }
		} else {
			None
		}
	}

	pub(crate) fn as_token(&self) -> Option<&GreenToken> {
		if !self.is_node() {
			unsafe { Some(&*(&self.ptr as *const ErasedPtr as *const GreenToken)) }
		} else {
			None
		}
	}

	pub(crate) fn into_token(self) -> Option<GreenToken> {
		if !self.is_node() {
			unsafe { Some(mem::transmute(self)) }
		} else {
			None
		}
	}

	pub(crate) fn as_ref(&self) -> GreenElementRef<'_> {
		if self.is_node() {
			NodeOrToken::Node(self.as_node().unwrap())
		} else {
			NodeOrToken::Token(self.as_token().unwrap())
		}
	}
}

impl fmt::Debug for PackedGreenElement {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		if self.is_node() {
			self.as_node().unwrap().fmt(f)
		} else {
			self.as_token().unwrap().fmt(f)
		}
	}
}

impl Eq for PackedGreenElement {}
impl PartialEq for PackedGreenElement {
	fn eq(&self, other: &Self) -> bool {
		self.as_node() == other.as_node() && self.as_token() == other.as_token()
	}
}

impl hash::Hash for PackedGreenElement {
	fn hash<H>(&self, state: &mut H)
	where
		H: hash::Hasher,
	{
		if self.is_node() {
			self.as_node().unwrap().hash(state)
		} else {
			self.as_token().unwrap().hash(state)
		}
	}
}

impl Drop for PackedGreenElement {
	fn drop(&mut self) {
		if self.is_node() {
			PackedGreenElement { ptr: self.ptr }.into_node();
		} else {
			PackedGreenElement { ptr: self.ptr }.into_token();
		}
	}
}

unsafe impl Send for PackedGreenElement
where
	GreenToken: Send,
	GreenNode: Send,
{
}
unsafe impl Sync for PackedGreenElement
where
	GreenToken: Sync,
	GreenNode: Sync,
{
}
