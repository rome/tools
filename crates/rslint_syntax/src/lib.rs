//! A crate for generated Syntax node definitions and utility macros.
//! Both rslint_lexer and rslint_parser rely on these definitions, therefore
//! they are wrapped in this crate to prevent cyclic dependencies

#[macro_use]
mod generated;

pub use self::generated::SyntaxKind;

impl From<u16> for SyntaxKind {
	fn from(d: u16) -> SyntaxKind {
		assert!(d <= (SyntaxKind::__LAST as u16));
		unsafe { std::mem::transmute::<u16, SyntaxKind>(d) }
	}
}

impl From<SyntaxKind> for u16 {
	fn from(k: SyntaxKind) -> u16 {
		k as u16
	}
}

impl SyntaxKind {
	pub fn is_trivia(self) -> bool {
		matches!(self, SyntaxKind::WHITESPACE | SyntaxKind::COMMENT)
	}
}
