//! Generated file, do not edit by hand, see `xtask/src/codegen`

use crate::{
	ast::*,
	SyntaxKind::{self, *},
	SyntaxNode, SyntaxToken, T,
};
#[allow(clippy::enum_variant_names)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Omega {
	pub(crate) syntax: SyntaxNode,
}
impl Omega {
	pub fn omega_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![omega]) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Alpha {
	pub(crate) syntax: SyntaxNode,
}
impl Alpha {
	pub fn alpha_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![alpha]) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Beta {
	pub(crate) syntax: SyntaxNode,
}
impl Beta {
	pub fn beta_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![beta]) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Lorem {
	Ipsum(Ipsum),
	Omega(Omega),
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Ipsum {
	Alpha(Alpha),
	Beta(Beta),
}
impl AstNode for Omega {
	fn can_cast(kind: SyntaxKind) -> bool { kind == OMEGA }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for Alpha {
	fn can_cast(kind: SyntaxKind) -> bool { kind == ALPHA }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl AstNode for Beta {
	fn can_cast(kind: SyntaxKind) -> bool { kind == BETA }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
}
impl From<Ipsum> for Lorem {
	fn from(node: Ipsum) -> Lorem { Lorem::Ipsum(node) }
}
impl From<Omega> for Lorem {
	fn from(node: Omega) -> Lorem { Lorem::Omega(node) }
}
impl AstNode for Lorem {
	fn can_cast(kind: SyntaxKind) -> bool { matches!(kind, IPSUM | OMEGA) }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			IPSUM => Lorem::Ipsum(Ipsum::cast(syntax)?),
			OMEGA => Lorem::Omega(Omega { syntax }),
			_ => return None,
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			Lorem::Ipsum(it) => it.syntax(),
			Lorem::Omega(it) => &it.syntax,
		}
	}
}
impl From<Alpha> for Ipsum {
	fn from(node: Alpha) -> Ipsum { Ipsum::Alpha(node) }
}
impl From<Beta> for Ipsum {
	fn from(node: Beta) -> Ipsum { Ipsum::Beta(node) }
}
impl AstNode for Ipsum {
	fn can_cast(kind: SyntaxKind) -> bool { matches!(kind, ALPHA | BETA) }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			ALPHA => Ipsum::Alpha(Alpha { syntax }),
			BETA => Ipsum::Beta(Beta { syntax }),
			_ => return None,
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			Ipsum::Alpha(it) => &it.syntax,
			Ipsum::Beta(it) => &it.syntax,
		}
	}
}
impl std::fmt::Display for Lorem {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for Ipsum {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for Omega {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for Alpha {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for Beta {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
