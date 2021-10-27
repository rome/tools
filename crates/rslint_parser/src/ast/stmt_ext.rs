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

/// The beginning to a For or For..in statement which can either be a variable declaration or an expression
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ForHead {
	Decl(VarDecl),
	Expr(Expr),
}

impl AstNode for ForHead {
	fn can_cast(kind: SyntaxKind) -> bool {
		VarDecl::can_cast(kind) || Expr::can_cast(kind)
	}

	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if VarDecl::can_cast(syntax.kind()) {
			Some(ForHead::Decl(VarDecl::cast(syntax)?))
		} else {
			Some(ForHead::Expr(Expr::cast(syntax)?))
		}
	}

	fn syntax(&self) -> &SyntaxNode {
		match self {
			ForHead::Decl(stmt) => stmt.syntax(),
			ForHead::Expr(expr) => expr.syntax(),
		}
	}
}

impl From<BlockStmt> for Stmt {
	fn from(node: BlockStmt) -> Stmt {
		Stmt::BlockStmt(node)
	}
}
impl From<EmptyStmt> for Stmt {
	fn from(node: EmptyStmt) -> Stmt {
		Stmt::EmptyStmt(node)
	}
}
impl From<ExprStmt> for Stmt {
	fn from(node: ExprStmt) -> Stmt {
		Stmt::ExprStmt(node)
	}
}
impl From<IfStmt> for Stmt {
	fn from(node: IfStmt) -> Stmt {
		Stmt::IfStmt(node)
	}
}
impl From<DoWhileStmt> for Stmt {
	fn from(node: DoWhileStmt) -> Stmt {
		Stmt::DoWhileStmt(node)
	}
}
impl From<WhileStmt> for Stmt {
	fn from(node: WhileStmt) -> Stmt {
		Stmt::WhileStmt(node)
	}
}
impl From<ForStmt> for Stmt {
	fn from(node: ForStmt) -> Stmt {
		Stmt::ForStmt(node)
	}
}
impl From<ForInStmt> for Stmt {
	fn from(node: ForInStmt) -> Stmt {
		Stmt::ForInStmt(node)
	}
}
impl From<ContinueStmt> for Stmt {
	fn from(node: ContinueStmt) -> Stmt {
		Stmt::ContinueStmt(node)
	}
}
impl From<BreakStmt> for Stmt {
	fn from(node: BreakStmt) -> Stmt {
		Stmt::BreakStmt(node)
	}
}
impl From<ReturnStmt> for Stmt {
	fn from(node: ReturnStmt) -> Stmt {
		Stmt::ReturnStmt(node)
	}
}
impl From<WithStmt> for Stmt {
	fn from(node: WithStmt) -> Stmt {
		Stmt::WithStmt(node)
	}
}
impl From<LabelledStmt> for Stmt {
	fn from(node: LabelledStmt) -> Stmt {
		Stmt::LabelledStmt(node)
	}
}
impl From<SwitchStmt> for Stmt {
	fn from(node: SwitchStmt) -> Stmt {
		Stmt::SwitchStmt(node)
	}
}
impl From<ThrowStmt> for Stmt {
	fn from(node: ThrowStmt) -> Stmt {
		Stmt::ThrowStmt(node)
	}
}
impl From<TryStmt> for Stmt {
	fn from(node: TryStmt) -> Stmt {
		Stmt::TryStmt(node)
	}
}
impl From<DebuggerStmt> for Stmt {
	fn from(node: DebuggerStmt) -> Stmt {
		Stmt::DebuggerStmt(node)
	}
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Stmt {
	BlockStmt(BlockStmt),
	EmptyStmt(EmptyStmt),
	ExprStmt(ExprStmt),
	IfStmt(IfStmt),
	DoWhileStmt(DoWhileStmt),
	WhileStmt(WhileStmt),
	ForStmt(ForStmt),
	ForInStmt(ForInStmt),
	ContinueStmt(ContinueStmt),
	BreakStmt(BreakStmt),
	ReturnStmt(ReturnStmt),
	WithStmt(WithStmt),
	LabelledStmt(LabelledStmt),
	SwitchStmt(SwitchStmt),
	ThrowStmt(ThrowStmt),
	TryStmt(TryStmt),
	DebuggerStmt(DebuggerStmt),
	Decl(Decl),
}

impl AstNode for Stmt {
	fn can_cast(kind: SyntaxKind) -> bool {
		match kind {
			BLOCK_STMT | EMPTY_STMT | EXPR_STMT | IF_STMT | DO_WHILE_STMT | WHILE_STMT
			| FOR_STMT | FOR_IN_STMT | CONTINUE_STMT | BREAK_STMT | RETURN_STMT | WITH_STMT
			| LABELLED_STMT | SWITCH_STMT | THROW_STMT | TRY_STMT | DEBUGGER_STMT => true,
			t if Decl::can_cast(t) => true,
			_ => false,
		}
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			BLOCK_STMT => Stmt::BlockStmt(BlockStmt { syntax }),
			EMPTY_STMT => Stmt::EmptyStmt(EmptyStmt { syntax }),
			EXPR_STMT => Stmt::ExprStmt(ExprStmt { syntax }),
			IF_STMT => Stmt::IfStmt(IfStmt { syntax }),
			DO_WHILE_STMT => Stmt::DoWhileStmt(DoWhileStmt { syntax }),
			WHILE_STMT => Stmt::WhileStmt(WhileStmt { syntax }),
			FOR_STMT => Stmt::ForStmt(ForStmt { syntax }),
			FOR_IN_STMT => Stmt::ForInStmt(ForInStmt { syntax }),
			CONTINUE_STMT => Stmt::ContinueStmt(ContinueStmt { syntax }),
			BREAK_STMT => Stmt::BreakStmt(BreakStmt { syntax }),
			RETURN_STMT => Stmt::ReturnStmt(ReturnStmt { syntax }),
			WITH_STMT => Stmt::WithStmt(WithStmt { syntax }),
			LABELLED_STMT => Stmt::LabelledStmt(LabelledStmt { syntax }),
			SWITCH_STMT => Stmt::SwitchStmt(SwitchStmt { syntax }),
			THROW_STMT => Stmt::ThrowStmt(ThrowStmt { syntax }),
			TRY_STMT => Stmt::TryStmt(TryStmt { syntax }),
			DEBUGGER_STMT => Stmt::DebuggerStmt(DebuggerStmt { syntax }),
			_ => Stmt::Decl(Decl::cast(syntax)?),
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			Stmt::BlockStmt(it) => &it.syntax,
			Stmt::EmptyStmt(it) => &it.syntax,
			Stmt::ExprStmt(it) => &it.syntax,
			Stmt::IfStmt(it) => &it.syntax,
			Stmt::DoWhileStmt(it) => &it.syntax,
			Stmt::WhileStmt(it) => &it.syntax,
			Stmt::ForStmt(it) => &it.syntax,
			Stmt::ForInStmt(it) => &it.syntax,
			Stmt::ContinueStmt(it) => &it.syntax,
			Stmt::BreakStmt(it) => &it.syntax,
			Stmt::ReturnStmt(it) => &it.syntax,
			Stmt::WithStmt(it) => &it.syntax,
			Stmt::LabelledStmt(it) => &it.syntax,
			Stmt::SwitchStmt(it) => &it.syntax,
			Stmt::ThrowStmt(it) => &it.syntax,
			Stmt::TryStmt(it) => &it.syntax,
			Stmt::DebuggerStmt(it) => &it.syntax,
			Stmt::Decl(it) => it.syntax(),
		}
	}
}

impl std::fmt::Display for Stmt {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		std::fmt::Display::fmt(self.syntax(), f)
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
	pub fn cons(&self) -> Option<Stmt> {
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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ModuleItem {
	ImportDecl(ImportDecl),
	ExportNamed(ExportNamed),
	ExportDefaultDecl(ExportDefaultDecl),
	ExportDefaultExpr(ExportDefaultExpr),
	ExportWildcard(ExportWildcard),
	ExportDecl(ExportDecl),
	TsImportEqualsDecl(TsImportEqualsDecl),
	TsExportAssignment(TsExportAssignment),
	TsNamespaceExportDecl(TsNamespaceExportDecl),
	Stmt(Stmt),
}

impl AstNode for ModuleItem {
	fn can_cast(kind: SyntaxKind) -> bool {
		matches!(
			kind,
			IMPORT_DECL
				| EXPORT_NAMED | EXPORT_DEFAULT_DECL
				| EXPORT_DEFAULT_EXPR
				| EXPORT_WILDCARD
				| EXPORT_DECL | TS_IMPORT_EQUALS_DECL
				| TS_NAMESPACE_EXPORT_DECL
		) || Stmt::can_cast(kind)
	}
	fn cast(syntax: SyntaxNode) -> Option<Self> {
		let res = match syntax.kind() {
			IMPORT_DECL => ModuleItem::ImportDecl(ImportDecl { syntax }),
			EXPORT_NAMED => ModuleItem::ExportNamed(ExportNamed { syntax }),
			EXPORT_DEFAULT_DECL => ModuleItem::ExportDefaultDecl(ExportDefaultDecl { syntax }),
			EXPORT_DEFAULT_EXPR => ModuleItem::ExportDefaultExpr(ExportDefaultExpr { syntax }),
			EXPORT_WILDCARD => ModuleItem::ExportWildcard(ExportWildcard { syntax }),
			EXPORT_DECL => ModuleItem::ExportDecl(ExportDecl { syntax }),
			TS_IMPORT_EQUALS_DECL => ModuleItem::TsImportEqualsDecl(TsImportEqualsDecl { syntax }),
			TS_EXPORT_ASSIGNMENT => ModuleItem::TsExportAssignment(TsExportAssignment { syntax }),
			TS_NAMESPACE_EXPORT_DECL => {
				ModuleItem::TsNamespaceExportDecl(TsNamespaceExportDecl { syntax })
			}
			_ => ModuleItem::Stmt(Stmt::cast(syntax)?),
		};
		Some(res)
	}
	fn syntax(&self) -> &SyntaxNode {
		match self {
			ModuleItem::ImportDecl(it) => &it.syntax,
			ModuleItem::ExportNamed(it) => &it.syntax,
			ModuleItem::ExportDefaultDecl(it) => &it.syntax,
			ModuleItem::ExportDefaultExpr(it) => &it.syntax,
			ModuleItem::ExportWildcard(it) => &it.syntax,
			ModuleItem::ExportDecl(it) => &it.syntax,
			ModuleItem::Stmt(it) => it.syntax(),
			ModuleItem::TsImportEqualsDecl(it) => &it.syntax,
			ModuleItem::TsExportAssignment(it) => &it.syntax,
			ModuleItem::TsNamespaceExportDecl(it) => &it.syntax,
		}
	}
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ConstructorParamOrPat {
	TsConstructorParam(TsConstructorParam),
	Pattern(Pattern),
}

impl AstNode for ConstructorParamOrPat {
	fn can_cast(kind: SyntaxKind) -> bool {
		if kind == TS_CONSTRUCTOR_PARAM {
			true
		} else {
			Pattern::can_cast(kind)
		}
	}

	fn cast(syntax: SyntaxNode) -> Option<Self> {
		if syntax.kind() == TS_CONSTRUCTOR_PARAM {
			Some(ConstructorParamOrPat::TsConstructorParam(
				TsConstructorParam::cast(syntax).unwrap(),
			))
		} else {
			Some(ConstructorParamOrPat::Pattern(Pattern::cast(syntax)?))
		}
	}

	fn syntax(&self) -> &SyntaxNode {
		match self {
			ConstructorParamOrPat::TsConstructorParam(it) => it.syntax(),
			ConstructorParamOrPat::Pattern(it) => it.syntax(),
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
