use crate::arc::Arc;
use std::{convert::TryFrom, fmt, hash, mem::ManuallyDrop, ptr};

use crate::{green::SyntaxKind, SmolStr, TextSize};

#[repr(align(2))] // NB: this is an at-least annotation
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub(super) struct GreenTokenData {
	pub(super) kind: SyntaxKind,
	pub(super) text: SmolStr,
}

/// Leaf node in the immutable tree.
pub struct GreenToken {
	ptr: ptr::NonNull<GreenTokenData>,
}

unsafe impl Send for GreenToken {} // where GreenTokenData: Send + Sync
unsafe impl Sync for GreenToken {} // where GreenTokenData: Send + Sync

impl GreenToken {
	fn add_tag(ptr: ptr::NonNull<GreenTokenData>) -> ptr::NonNull<GreenTokenData> {
		unsafe {
			let ptr = ((ptr.as_ptr() as usize) | 1) as *mut GreenTokenData;
			ptr::NonNull::new_unchecked(ptr)
		}
	}

	fn remove_tag(ptr: ptr::NonNull<GreenTokenData>) -> ptr::NonNull<GreenTokenData> {
		unsafe {
			let ptr = ((ptr.as_ptr() as usize) & !1) as *mut GreenTokenData;
			ptr::NonNull::new_unchecked(ptr)
		}
	}

	fn data(&self) -> &GreenTokenData {
		unsafe { &*Self::remove_tag(self.ptr).as_ptr() }
	}

	/// Creates new Token.
	#[inline]
	pub fn new(kind: SyntaxKind, text: SmolStr) -> GreenToken {
		let ptr = Arc::into_raw(Arc::new(GreenTokenData { kind, text }));
		let ptr = ptr::NonNull::new(ptr as *mut _).unwrap();
		GreenToken {
			ptr: Self::add_tag(ptr),
		}
	}

	/// Kind of this Token.
	#[inline]
	pub fn kind(&self) -> SyntaxKind {
		self.data().kind
	}

	/// Text of this Token.
	#[inline]
	pub fn text(&self) -> &SmolStr {
		&self.data().text
	}

	/// Returns the length of the text covered by this token.
	#[inline]
	pub fn text_len(&self) -> TextSize {
		TextSize::try_from(self.text().len()).unwrap()
	}
}

impl fmt::Debug for GreenToken {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let data = self.data();
		f.debug_struct("GreenToken")
			.field("kind", &data.kind)
			.field("text", &data.text)
			.finish()
	}
}

impl Clone for GreenToken {
	fn clone(&self) -> Self {
		let ptr = Self::remove_tag(self.ptr);
		let ptr = unsafe {
			let arc = ManuallyDrop::new(Arc::from_raw(ptr.as_ptr()));
			Arc::into_raw(Arc::clone(&arc))
		};
		let ptr = ptr::NonNull::new(ptr as *mut _).unwrap();
		GreenToken {
			ptr: Self::add_tag(ptr),
		}
	}
}

impl Eq for GreenToken {}
impl PartialEq for GreenToken {
	fn eq(&self, other: &Self) -> bool {
		self.data() == other.data()
	}
}

impl hash::Hash for GreenToken {
	fn hash<H>(&self, state: &mut H)
	where
		H: hash::Hasher,
	{
		self.data().hash(state)
	}
}

impl Drop for GreenToken {
	fn drop(&mut self) {
		unsafe {
			Arc::from_raw(Self::remove_tag(self.ptr).as_ptr());
		}
	}
}
