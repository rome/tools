//! Generated file, do not edit by hand, see `xtask/src/codegen`

#![allow(clippy::enum_variant_names)]
use crate::{
	ast::*,
	SyntaxKind::{self, *},
	SyntaxNode, SyntaxToken, T,
};
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Flat {
	pub(crate) syntax: SyntaxNode,
}
impl Flat {
	pub fn flat_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![flat]) }
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
pub struct Gamma {
	pub(crate) syntax: SyntaxNode,
}
impl Gamma {
	pub fn gamma_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![gamma]) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Omega {
	pub(crate) syntax: SyntaxNode,
}
impl Omega {
	pub fn omega_token(&self) -> Option<SyntaxToken> { support::token(&self.syntax, T![omega]) }
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Type {
	Flat(Flat),
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Lorem {
	Alpha(Alpha),
	Beta(Beta),
}
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Ipsum {
	Gamma(Gamma),
	Omega(Omega),
}
impl AstNode for Flat {
	fn can_cast(kind: SyntaxKind) -> bool { kind == FLAT }
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
impl AstNode for Gamma {
	fn can_cast(kind: SyntaxKind) -> bool { kind == GAMMA }
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			Some(Self { syntax })
		} else {
			None
		}
	}
	fn syntax(&self) -> &SyntaxNode { &self.syntax }
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
impl From<Flat> for Type {
	fn from(node: Flat) -> Type { Type::Flat(node) }
}
impl AstNode for Type {
	fn can_cast(kind: SyntaxKind) -> bool {
		match kind {
			FLAT => true,
			k if Lorem::can_cast(k) => true,
			k if Ipsum::can_cast(k) => true,
			_ => false,
		}
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			FLAT => Type::Flat(Flat { syntax }),
			_ => {
				if let Some(lorem) = Lorem::cast(syntax.clone()) {
					return Some(Expr::Lorem(lorem));
				}
				if let Some(ipsum) = Ipsum::cast(syntax.clone()) {
					return Some(Expr::Ipsum(ipsum));
				}
				return None;
			}
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			Type::Flat(it) => &it.syntax,
			Expr::Lorem(it) => it.syntax(),
			Expr::Ipsum(it) => it.syntax(),
		}
	}
}
impl From<Alpha> for Lorem {
	fn from(node: Alpha) -> Lorem { Lorem::Alpha(node) }
}
impl From<Beta> for Lorem {
	fn from(node: Beta) -> Lorem { Lorem::Beta(node) }
}
impl AstNode for Lorem {
	fn can_cast(kind: SyntaxKind) -> bool {
		match kind {
			ALPHA | BETA => true,
			_ => false,
		}
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			ALPHA => Lorem::Alpha(Alpha { syntax }),
			BETA => Lorem::Beta(Beta { syntax }),
			_ => return None,
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			Lorem::Alpha(it) => &it.syntax,
			Lorem::Beta(it) => &it.syntax,
		}
	}
}
impl From<Gamma> for Ipsum {
	fn from(node: Gamma) -> Ipsum { Ipsum::Gamma(node) }
}
impl From<Omega> for Ipsum {
	fn from(node: Omega) -> Ipsum { Ipsum::Omega(node) }
}
impl AstNode for Ipsum {
	fn can_cast(kind: SyntaxKind) -> bool {
		match kind {
			GAMMA | OMEGA => true,
			_ => false,
		}
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			GAMMA => Ipsum::Gamma(Gamma { syntax }),
			OMEGA => Ipsum::Omega(Omega { syntax }),
			_ => return None,
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			Ipsum::Gamma(it) => &it.syntax,
			Ipsum::Omega(it) => &it.syntax,
		}
	}
}
impl std::fmt::Display for Type {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
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
impl std::fmt::Display for Flat {
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
impl std::fmt::Display for Gamma {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
impl std::fmt::Display for Omega {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
	}
}
