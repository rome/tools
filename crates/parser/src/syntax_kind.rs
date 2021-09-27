pub mod generated;

pub use generated::{get_syntax_kind, SyntaxKind};

impl From<u16> for SyntaxKind {
	#[inline]
	fn from(d: u16) -> SyntaxKind {
		assert!(d <= (SyntaxKind::__LAST as u16));
		unsafe { std::mem::transmute::<u16, SyntaxKind>(d) }
	}
}
impl From<SyntaxKind> for u16 {
	#[inline]
	fn from(k: SyntaxKind) -> u16 {
		k as u16
	}
}

impl SyntaxKind {
	pub fn is_whitespace(self) -> bool {
		self == SyntaxKind::Whitespace
	}
}
