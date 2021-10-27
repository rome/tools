//! Extended AST node definitions for statements which are unique and special enough to generate code for manually

use crate::{ast::*, syntax_node::SyntaxNode, SyntaxKind, SyntaxKind::*, SyntaxNodeExt, T};

/// Either a statement or a declaration such as a function
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum StmtListItem {
	Stmt(Stmt),
	Decl(Decl),
}

impl AstNode for StmtListItem {
	fn can_cast(kind: SyntaxKind) -> bool {
		Stmt::can_cast(kind) || Decl::can_cast(kind)
	}

	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if Stmt::can_cast(syntax.kind()) {
			Some(StmtListItem::Stmt(Stmt::cast(syntax)?))
		} else {
			Some(StmtListItem::Decl(Decl::cast(syntax)?))
		}
	}

	fn syntax(&self) -> &SyntaxNode {
		match self {
			StmtListItem::Stmt(stmt) => stmt.syntax(),
			StmtListItem::Decl(decl) => decl.syntax(),
		}
	}
}

impl VarDecl {
	// TODO: switch this to a contextual keyword once the typescript pr lands
	pub fn let_token(&self) -> Option<SyntaxToken> {
		self.syntax()
			.first_lossy_token()
			.filter(|t| t.kind() == T![ident] && t.text() == "let")
	}

	/// Whether the declaration is a const declaration
	pub fn is_const(&self) -> bool {
		self.const_token().is_some()
	}

	/// Whether the declaration is a let declaration
	pub fn is_let(&self) -> bool {
		self.let_token().is_some()
	}

	/// Whether the declaration is a let declaration
	pub fn is_var(&self) -> bool {
		self.var_token().is_some()
	}
}

impl ImportDecl {
	/// The source of the import, such as `import a from "a"` ("a"), or `import "foo"` ("foo")
	pub fn source(&self) -> Option<Literal> {
		self.syntax()
			.children()
			.find_map(|x| x.try_to::<Literal>().filter(|x| x.is_string()))
	}
}

impl ExportDecl {
	/// The source of the export, such as `export a from "a"` ("a"), or `export "foo"` ("foo")
	pub fn source(&self) -> Option<Literal> {
		self.syntax().children().find_map(|x| {
			x.children()
				.find_map(|x| x.try_to::<Literal>().filter(|x| x.is_string()))
		})
	}
}

impl Specifier {
	pub fn as_token(&self) -> Option<SyntaxToken> {
		self.syntax()
			.children_with_tokens()
			.filter_map(|x| x.into_token())
			.nth(1)
	}

	pub fn alias(&self) -> Option<Name> {
		self.syntax().children().nth(1).and_then(|x| x.try_to())
	}
}

impl WildcardImport {
	pub fn alias(&self) -> Option<Name> {
		self.syntax().children().find_map(|x| x.try_to())
	}
}

impl IfStmt {
	pub fn get_cons(&self) -> Option<Stmt> {
		self.syntax().child_with_ast::<Stmt>().filter(|cons| {
			cons.syntax().text_range().start()
				<= self
					.else_token()
					.map(|x| x.text_range().start())
					.unwrap_or_else(|| cons.syntax().text_range().start())
		})
	}

	pub fn alt(&self) -> Option<Stmt> {
		let possible_blocks = self
			.syntax()
			.children()
			.filter(|child| child.is::<Stmt>())
			.collect::<Vec<_>>();

		// handle if (true) else {}
		if let Some(else_block) = possible_blocks.get(1) {
			Some(else_block.to())
		} else {
			possible_blocks
				.first()
				.filter(|node| {
					node.text_range().start()
						> self
							.else_token()
							.map(|x| x.text_range().start())
							.unwrap_or_else(|| node.text_range().start())
				})
				.map(|x| x.to())
		}
	}
}

impl SwitchCase {
	pub fn into_case(self) -> Option<CaseClause> {
		if let SwitchCase::CaseClause(clause) = self {
			Some(clause)
		} else {
			None
		}
	}
}

#[cfg(test)]
mod tests {
	use crate::*;

	#[test]
	fn var_decl_let_token() {
		let parsed = parse_text("/* */let a = 5;", 0).syntax();

		assert!(parsed
			.child_with_ast::<ast::VarDecl>()
			.unwrap()
			.let_token()
			.is_some());
	}
}

impl TsEnumMember {
	pub fn string_token(&self) -> Option<SyntaxToken> {
		support::token(&self.syntax, STRING)
	}
}
