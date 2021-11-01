use std::{
	borrow::Borrow,
	fmt,
	mem::{self, ManuallyDrop},
	ops, ptr,
};

use countme::Count;

use crate::{
	arc::{Arc, HeaderSlice, ThinArc},
	green::SyntaxKind,
	TextSize,
};

#[derive(PartialEq, Eq, Hash, Debug)]
pub enum GreenTokenTrivia {
	Whitespace,
}

#[derive(PartialEq, Eq, Hash)]
struct GreenTokenHead {
	kind: SyntaxKind,
	_c: Count<GreenToken>,
	leading_trivia: Vec<GreenTokenTrivia>,
	trailing_trivia: Vec<GreenTokenTrivia>,
}

type Repr = HeaderSlice<GreenTokenHead, [u8]>;
type ReprThin = HeaderSlice<GreenTokenHead, [u8; 0]>;
#[repr(transparent)]
pub(crate) struct GreenTokenData {
	data: ReprThin,
}

impl PartialEq for GreenTokenData {
	fn eq(&self, other: &Self) -> bool {
		//TODO is the compiler smart enought to optimize here?
		let kind_eq = self.kind() == other.kind() && self.text() == other.text();
		let leading_eq = self
			.data
			.header
			.leading_trivia
			.eq(&other.data.header.leading_trivia);
		let trailing_eq = self
			.data
			.header
			.trailing_trivia
			.eq(&other.data.header.trailing_trivia);
		kind_eq && leading_eq && trailing_eq
	}
}

/// Leaf node in the immutable tree.
#[derive(PartialEq, Eq, Hash, Clone)]
#[repr(transparent)]
pub(crate) struct GreenToken {
	ptr: ThinArc<GreenTokenHead, u8>,
}

impl ToOwned for GreenTokenData {
	type Owned = GreenToken;

	#[inline]
	fn to_owned(&self) -> GreenToken {
		unsafe {
			let green = GreenToken::from_raw(ptr::NonNull::from(self));
			let green = ManuallyDrop::new(green);
			GreenToken::clone(&green)
		}
	}
}

impl Borrow<GreenTokenData> for GreenToken {
	#[inline]
	fn borrow(&self) -> &GreenTokenData {
		&*self
	}
}

impl fmt::Debug for GreenTokenData {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		f.debug_struct("GreenToken")
			.field("kind", &self.kind())
			.field("text", &self.text())
			.field("leading_trivia", &self.data.header.leading_trivia)
			.field("trailing_trivia", &self.data.header.trailing_trivia)
			.finish()
	}
}

impl fmt::Debug for GreenToken {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let data: &GreenTokenData = &*self;
		fmt::Debug::fmt(data, f)
	}
}

impl fmt::Display for GreenToken {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		let data: &GreenTokenData = &*self;
		fmt::Display::fmt(data, f)
	}
}

impl fmt::Display for GreenTokenData {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", self.text())
	}
}

impl GreenTokenData {
	/// Kind of this Token.
	#[inline]
	pub fn kind(&self) -> SyntaxKind {
		self.data.header.kind
	}

	/// Text of this Token.
	#[inline]
	pub fn text(&self) -> &str {
		unsafe { std::str::from_utf8_unchecked(self.data.slice()) }
	}

	/// Returns the length of the text covered by this token.
	#[inline]
	pub fn text_len(&self) -> TextSize {
		TextSize::of(self.text())
	}
}

impl GreenToken {
	/// Creates new Token.
	#[inline]
	pub fn new(
		kind: SyntaxKind,
		text: &str,
		leading_trivia: Vec<GreenTokenTrivia>,
		trailing_trivia: Vec<GreenTokenTrivia>,
	) -> GreenToken {
		let head = GreenTokenHead {
			kind,
			_c: Count::new(),
			leading_trivia,
			trailing_trivia,
		};
		let ptr = ThinArc::from_header_and_iter(head, text.bytes());
		GreenToken { ptr }
	}

	#[inline]
	pub(crate) fn into_raw(this: GreenToken) -> ptr::NonNull<GreenTokenData> {
		let green = ManuallyDrop::new(this);
		let green: &GreenTokenData = &*green;
		ptr::NonNull::from(&*green)
	}

	#[inline]
	pub(crate) unsafe fn from_raw(ptr: ptr::NonNull<GreenTokenData>) -> GreenToken {
		let arc = Arc::from_raw(&ptr.as_ref().data as *const ReprThin);
		let arc = mem::transmute::<Arc<ReprThin>, ThinArc<GreenTokenHead, u8>>(arc);
		GreenToken { ptr: arc }
	}
}

impl ops::Deref for GreenToken {
	type Target = GreenTokenData;

	#[inline]
	fn deref(&self) -> &GreenTokenData {
		unsafe {
			let repr: &Repr = &self.ptr;
			let repr: &ReprThin = &*(repr as *const Repr as *const ReprThin);
			mem::transmute::<&ReprThin, &GreenTokenData>(repr)
		}
	}
}
