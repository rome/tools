//! Extensions to TypeScript AST elements

use crate::{
	ast::*,
	syntax_node::SyntaxNode,
	SyntaxKind::{self, *},
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum TsTypeQueryExpr {
	TsEntityName(TsEntityName),
	TsImport(TsImport),
}

impl AstNode for TsTypeQueryExpr {
	fn can_cast(kind: SyntaxKind) -> bool {
		TsEntityName::can_cast(kind) || TsImport::can_cast(kind)
	}

	fn cast(syntax: SyntaxNode) -> Option<Self> {
		match syntax.kind() {
			n if TsEntityName::can_cast(n) => Some(TsTypeQueryExpr::TsEntityName(
				TsEntityName::cast(syntax).unwrap(),
			)),
			_ => Some(TsTypeQueryExpr::TsImport(TsImport::cast(syntax)?)),
		}
	}

	fn syntax(&self) -> &SyntaxNode {
		match self {
			TsTypeQueryExpr::TsEntityName(it) => it.syntax(),
			TsTypeQueryExpr::TsImport(it) => it.syntax(),
		}
	}
}

impl TsImport {
	pub fn arg(&self) -> Option<SyntaxToken> {
		self.syntax()
			.tokens()
			.into_iter()
			.find(|t| t.kind() == STRING)
	}
}

impl TsMappedTypeParam {
	/// present for alias
	pub fn as_token(&self) -> Option<SyntaxToken> {
		self.syntax()
			.children_with_tokens()
			.filter_map(|x| x.into_token())
			.find(|x| x.kind() == IDENT && x.text() == "as")
	}

	pub fn alias(&self) -> Option<TsType> {
		self.syntax()
			.children()
			.filter_map(|x| x.try_to::<TsType>())
			.nth(1)
	}
}

#[derive(Debug, Clone, PartialEq)]
pub enum TsModuleRef {
	TsExternalModuleRef(TsExternalModuleRef),
	TsEntityName(TsEntityName),
}

impl AstNode for TsModuleRef {
	fn can_cast(kind: SyntaxKind) -> bool {
		kind == TS_EXTERNAL_MODULE_REF || TsEntityName::can_cast(kind)
	}

	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Self::can_cast(syntax.kind()) {
			None
		} else {
			Some(match syntax.kind() {
				TS_EXTERNAL_MODULE_REF => TsModuleRef::TsExternalModuleRef(syntax.to()),
				_ => TsModuleRef::TsEntityName(syntax.to()),
			})
		}
	}

	fn syntax(&self) -> &SyntaxNode {
		match self {
			TsModuleRef::TsExternalModuleRef(it) => it.syntax(),
			TsModuleRef::TsEntityName(it) => it.syntax(),
		}
	}
}
