//! Generated file, do not edit by hand, see `xtask/src/codegen`

use crate::{
	ast::AstToken,
	SyntaxKind::{self, *},
	SyntaxToken,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct String {
	pub(crate) syntax: SyntaxToken,
}
impl std::fmt::Display for String {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(&self.syntax, f)
	}
}
impl AstToken for String {
	fn can_cast(kind: SyntaxKind) -> bool { kind == STRING }
	fn cast(syntax: SyntaxToken) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxToken { &self.syntax }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Number {
	pub(crate) syntax: SyntaxToken,
}
impl std::fmt::Display for Number {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(&self.syntax, f)
	}
}
impl AstToken for Number {
	fn can_cast(kind: SyntaxKind) -> bool { kind == NUMBER }
	fn cast(syntax: SyntaxToken) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxToken { &self.syntax }
}
