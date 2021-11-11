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

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub enum Trivia {
	Whitespace(usize),
	Comment(usize),
}

impl Trivia {
	fn text_len(&self) -> TextSize {
		match self {
			Trivia::Whitespace(n) => (*n as u32).into(),
			Trivia::Comment(n) => (*n as u32).into(),
		}
	}
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum GreenTokenTrivia {
	None,
	Whitespace(usize),
	Comment(usize),
	Many(Box<Vec<Trivia>>),
}

impl GreenTokenTrivia {
	pub fn text_len(&self) -> TextSize {
		match self {
			GreenTokenTrivia::None => 0.into(),
			GreenTokenTrivia::Whitespace(len) => (*len as u32).into(),
			GreenTokenTrivia::Comment(len) => (*len as u32).into(),
			GreenTokenTrivia::Many(v) => v.iter().fold(0.into(), |len, x| len + x.text_len()),
		}
	}
}

#[derive(PartialEq, Eq, Hash)]
struct GreenTokenHead {
	kind: SyntaxKind,
	leading_trivia: GreenTokenTrivia,
	trailing_trivia: GreenTokenTrivia,
	_c: Count<GreenToken>,
}

type Repr = HeaderSlice<GreenTokenHead, [u8]>;
type ReprThin = HeaderSlice<GreenTokenHead, [u8; 0]>;

#[repr(transparent)]
pub(crate) struct GreenTokenData {
	data: ReprThin,
}

impl PartialEq for GreenTokenData {
	fn eq(&self, other: &Self) -> bool {
		self.kind() == other.kind() && self.text() == other.text()
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
			.field("text", &self.text_with_trivia())
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
		write!(f, "{}", self.text_with_trivia())
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
		let token_start: usize = self.data.header.leading_trivia.text_len().into();

		let trailing_len: usize = self.data.header.trailing_trivia.text_len().into();
		let token_end = self.data.slice().len() - trailing_len;

		let s = unsafe { std::str::from_utf8_unchecked(self.data.slice()) };
		&s[token_start..token_end]
	}

	/// Text of this Token with trivia.
	#[inline]
	pub fn text_with_trivia(&self) -> &str {
		unsafe { std::str::from_utf8_unchecked(self.data.slice()) }
	}

	/// Returns the length of the text covered by this token.
	#[inline]
	pub fn text_len(&self) -> TextSize {
		TextSize::of(self.text())
	}

	#[inline]
	pub fn text_with_trivia_len(&self) -> TextSize {
		TextSize::of(self.text_with_trivia())
	}

	#[inline]
	pub fn leading(&self) -> &GreenTokenTrivia {
		&self.data.header.leading_trivia
	}

	#[inline]
	pub fn trailing(&self) -> &GreenTokenTrivia {
		&self.data.header.trailing_trivia
	}

	pub(crate) fn cache_hash_of(kind: SyntaxKind, text: &str) -> u64 {
		use std::hash::{Hash, Hasher};
		let mut h = rustc_hash::FxHasher::default();
		kind.hash(&mut h);
		text.hash(&mut h);
		h.finish()
	}

	pub(crate) fn cache_hash(&self) -> u64 {
		Self::cache_hash_of(self.kind(), self.text_with_trivia())
	}
}

impl GreenToken {
	/// Creates new Token.
	#[inline]
	#[allow(dead_code)]
	pub fn new(kind: SyntaxKind, text: &str) -> GreenToken {
		Self::with_trivia(kind, text, GreenTokenTrivia::None, GreenTokenTrivia::None)
	}

	#[inline]
	pub fn with_trivia(
		kind: SyntaxKind,
		text: &str,
		leading_trivia: GreenTokenTrivia,
		trailing_trivia: GreenTokenTrivia,
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

#[test]
fn green_token_text_and_len() {
	let t = GreenToken::with_trivia(
		SyntaxKind(0),
		" let ",
		GreenTokenTrivia::Whitespace(1),
		GreenTokenTrivia::Whitespace(1),
	);

	assert_eq!("let", t.text());
	assert_eq!(text_size::TextSize::from(3), t.text_len());

	assert_eq!(" let ", t.text_with_trivia());
	assert_eq!(text_size::TextSize::from(5), t.text_with_trivia_len());

	assert_eq!(" let ", format!("{}", t));
}

#[test]
fn green_token_hash() {
	let kind = SyntaxKind(0);
	let text = " let ";
	let t1 = GreenToken::with_trivia(
		kind,
		text,
		GreenTokenTrivia::Whitespace(1),
		GreenTokenTrivia::Whitespace(1),
	);
	let t2 = GreenToken::with_trivia(
		kind,
		text,
		GreenTokenTrivia::Whitespace(1),
		GreenTokenTrivia::Whitespace(1),
	);

	assert_eq!(t1.cache_hash(), t2.cache_hash());
	assert_eq!(GreenTokenData::cache_hash_of(kind, text), t1.cache_hash());

	let t3 = GreenToken::with_trivia(
		kind,
		"\n\tlet ",
		GreenTokenTrivia::Whitespace(2),
		GreenTokenTrivia::Whitespace(1),
	);
	assert_ne!(t1.cache_hash(), t3.cache_hash());
}
