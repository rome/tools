use crate::{token, FormatElement, Formatter, ToFormatElement};
use rslint_parser::ast::{
	ArrayExpr, ArrowExpr, AssignPattern, BlockStmt, CaseClause, CatchClause, Condition,
	ContinueStmt, DebuggerStmt, Declarator, DefaultClause, DoWhileStmt, EmptyStmt, ExprStmt,
	Finalizer, FnDecl, ForInStmt, ForStmt, ForStmtInit, ForStmtTest, ForStmtUpdate, IdentProp,
	IfStmt, LabelledStmt, Literal, Name, NameRef, ObjectExpr, ParameterList, ReturnStmt, Script,
	SequenceExpr, SinglePattern, SwitchStmt, TryStmt, VarDecl, WhileStmt, WithStmt,
};
use rslint_parser::{AstNode, AstToken, SyntaxKind, SyntaxNode, SyntaxToken};

impl ToFormatElement for SyntaxNode {
	fn to_format_element(&self, formatter: &Formatter) -> Option<FormatElement> {
		match self.kind() {
			SyntaxKind::ARRAY_EXPR => ArrayExpr::cast(self.clone())
				.unwrap()
				.to_format_element(formatter),
			SyntaxKind::ARROW_EXPR => ArrowExpr::cast(self.clone())
				.unwrap()
				.to_format_element(formatter),
			SyntaxKind::ASSIGN_PATTERN => AssignPattern::cast(self.clone())
				.unwrap()
				.to_format_element(formatter),
			SyntaxKind::LITERAL => Literal::cast(self.clone())
				.unwrap()
				.to_format_element(formatter),
			SyntaxKind::NAME => Name::cast(self.clone())
				.unwrap()
				.to_format_element(formatter),
			SyntaxKind::NAME_REF => NameRef::cast(self.clone())
				.unwrap()
				.to_format_element(formatter),
			SyntaxKind::PARAMETER_LIST => ParameterList::cast(self.clone())
				.unwrap()
				.to_format_element(formatter),
			SyntaxKind::SCRIPT => Script::cast(self.clone())
				.unwrap()
				.to_format_element(formatter),
			SyntaxKind::SINGLE_PATTERN => SinglePattern::cast(self.clone())
				.unwrap()
				.to_format_element(formatter),
			SyntaxKind::SPREAD_ELEMENT => SinglePattern::cast(self.clone())
				.unwrap()
				.to_format_element(formatter),
			SyntaxKind::VAR_DECL => VarDecl::cast(self.clone())
				.unwrap()
				.to_format_element(formatter),
			SyntaxKind::DECLARATOR => Declarator::cast(self.clone())
				.unwrap()
				.to_format_element(formatter),
			SyntaxKind::FN_DECL => FnDecl::cast(self.clone())
				.unwrap()
				.to_format_element(formatter),
			SyntaxKind::SEQUENCE_EXPR => SequenceExpr::cast(self.clone())
				.unwrap()
				.to_format_element(formatter),
			SyntaxKind::BLOCK_STMT => BlockStmt::cast(self.clone())
				.unwrap()
				.to_format_element(formatter),
			SyntaxKind::EXPR_STMT => ExprStmt::cast(self.clone())
				.unwrap()
				.to_format_element(formatter),
			SyntaxKind::RETURN_STMT => ReturnStmt::cast(self.clone())
				.unwrap()
				.to_format_element(formatter),
			SyntaxKind::IF_STMT => IfStmt::cast(self.clone())
				.unwrap()
				.to_format_element(formatter),
			SyntaxKind::CONDITION => Condition::cast(self.clone())
				.unwrap()
				.to_format_element(formatter),
			SyntaxKind::FOR_STMT => ForStmt::cast(self.clone())
				.unwrap()
				.to_format_element(formatter),
			SyntaxKind::FOR_STMT_TEST => ForStmtTest::cast(self.clone())
				.unwrap()
				.to_format_element(formatter),
			SyntaxKind::FOR_STMT_INIT => ForStmtInit::cast(self.clone())
				.unwrap()
				.to_format_element(formatter),
			SyntaxKind::FOR_STMT_UPDATE => ForStmtUpdate::cast(self.clone())
				.unwrap()
				.to_format_element(formatter),
			SyntaxKind::EMPTY_STMT => EmptyStmt::cast(self.clone())
				.unwrap()
				.to_format_element(formatter),
			SyntaxKind::IDENT_PROP => IdentProp::cast(self.clone())
				.unwrap()
				.to_format_element(formatter),
			SyntaxKind::OBJECT_EXPR => ObjectExpr::cast(self.clone())
				.unwrap()
				.to_format_element(formatter),
			SyntaxKind::WHILE_STMT => WhileStmt::cast(self.clone())
				.unwrap()
				.to_format_element(formatter),
			SyntaxKind::DO_WHILE_STMT => DoWhileStmt::cast(self.clone())
				.unwrap()
				.to_format_element(formatter),
			SyntaxKind::SWITCH_STMT => SwitchStmt::cast(self.clone())
				.unwrap()
				.to_format_element(formatter),
			SyntaxKind::DEFAULT_CLAUSE => DefaultClause::cast(self.clone())
				.unwrap()
				.to_format_element(formatter),
			SyntaxKind::CASE_CLAUSE => CaseClause::cast(self.clone())
				.unwrap()
				.to_format_element(formatter),
			SyntaxKind::CONTINUE_STMT => ContinueStmt::cast(self.clone())
				.unwrap()
				.to_format_element(formatter),
			SyntaxKind::LABELLED_STMT => LabelledStmt::cast(self.clone())
				.unwrap()
				.to_format_element(formatter),
			SyntaxKind::TRY_STMT => TryStmt::cast(self.clone())
				.unwrap()
				.to_format_element(formatter),
			SyntaxKind::FINALIZER => Finalizer::cast(self.clone())
				.unwrap()
				.to_format_element(formatter),
			SyntaxKind::CATCH_CLAUSE => CatchClause::cast(self.clone())
				.unwrap()
				.to_format_element(formatter),
			SyntaxKind::WITH_STMT => WithStmt::cast(self.clone())
				.unwrap()
				.to_format_element(formatter),
			SyntaxKind::DEBUGGER_STMT => DebuggerStmt::cast(self.clone())
				.unwrap()
				.to_format_element(formatter),
			SyntaxKind::FOR_IN_STMT => ForInStmt::cast(self.clone())
				.unwrap()
				.to_format_element(formatter),
			_ => todo!(
				"Implement formatting for the {:?} syntax kind.",
				self.kind()
			),
		}
	}
}

impl ToFormatElement for SyntaxToken {
	fn to_format_element(&self, formatter: &Formatter) -> Option<FormatElement> {
		match self.kind() {
			SyntaxKind::STRING => rslint_parser::ast::String::cast(self.clone())
				.unwrap()
				.to_format_element(formatter),
			_ => Some(token(self.text().as_str())),
		}
	}
}
