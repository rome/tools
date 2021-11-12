use crate::{token, FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_parser::ast::{
	ArgList, ArrayExpr, ArrayPattern, ArrowExpr, AssignPattern, CallExpr, ClassBody, ClassDecl,
	ClassProp, Condition, ConstructorParameters, Declarator, FnDecl, ForInStmt, ForStmt,
	ForStmtInit, ForStmtTest, ForStmtUpdate, Getter, IdentProp, JsBlockStatement, JsCaseClause,
	JsCatchClause, JsContinueStatement, JsDebuggerStatement, JsDefaultClause, JsDoWhileStatement,
	JsEmptyStatement, JsExpressionStatement, JsFinallyClause, JsIfStatement, JsLabeledStatement,
	JsReturnStatement, JsScript, JsSwitchStatement, JsTryStatement, JsWhileStatement,
	JsWithStatement, Literal, LiteralProp, Name, NameRef, ObjectExpr, ParameterList, SequenceExpr,
	Setter, SinglePattern, VarDecl,
};
use rslint_parser::{AstNode, AstToken, SyntaxKind, SyntaxNode, SyntaxToken};

impl ToFormatElement for SyntaxNode {
	fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
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
			SyntaxKind::JS_SCRIPT => JsScript::cast(self.clone())
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
			SyntaxKind::JS_BLOCK_STATEMENT => JsBlockStatement::cast(self.clone())
				.unwrap()
				.to_format_element(formatter),
			SyntaxKind::JS_EXPRESSION_STATEMENT => JsExpressionStatement::cast(self.clone())
				.unwrap()
				.to_format_element(formatter),
			SyntaxKind::JS_RETURN_STATEMENT => JsReturnStatement::cast(self.clone())
				.unwrap()
				.to_format_element(formatter),
			SyntaxKind::JS_IF_STATEMENT => JsIfStatement::cast(self.clone())
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
			SyntaxKind::JS_EMPTY_STATEMENT => JsEmptyStatement::cast(self.clone())
				.unwrap()
				.to_format_element(formatter),
			SyntaxKind::IDENT_PROP => IdentProp::cast(self.clone())
				.unwrap()
				.to_format_element(formatter),
			SyntaxKind::OBJECT_EXPR => ObjectExpr::cast(self.clone())
				.unwrap()
				.to_format_element(formatter),
			SyntaxKind::JS_WHILE_STATEMENT => JsWhileStatement::cast(self.clone())
				.unwrap()
				.to_format_element(formatter),
			SyntaxKind::JS_DO_WHILE_STATEMENT => JsDoWhileStatement::cast(self.clone())
				.unwrap()
				.to_format_element(formatter),
			SyntaxKind::JS_SWITCH_STATEMENT => JsSwitchStatement::cast(self.clone())
				.unwrap()
				.to_format_element(formatter),
			SyntaxKind::JS_DEFAULT_CLAUSE => JsDefaultClause::cast(self.clone())
				.unwrap()
				.to_format_element(formatter),
			SyntaxKind::JS_CASE_CLAUSE => JsCaseClause::cast(self.clone())
				.unwrap()
				.to_format_element(formatter),
			SyntaxKind::JS_CONTINUE_STATEMENT => JsContinueStatement::cast(self.clone())
				.unwrap()
				.to_format_element(formatter),
			SyntaxKind::JS_LABELED_STATEMENT => JsLabeledStatement::cast(self.clone())
				.unwrap()
				.to_format_element(formatter),
			SyntaxKind::JS_TRY_STATEMENT => JsTryStatement::cast(self.clone())
				.unwrap()
				.to_format_element(formatter),
			SyntaxKind::JS_FINALLY_CLAUSE => JsFinallyClause::cast(self.clone())
				.unwrap()
				.to_format_element(formatter),
			SyntaxKind::JS_CATCH_CLAUSE => JsCatchClause::cast(self.clone())
				.unwrap()
				.to_format_element(formatter),
			SyntaxKind::JS_WITH_STATEMENT => JsWithStatement::cast(self.clone())
				.unwrap()
				.to_format_element(formatter),
			SyntaxKind::JS_DEBUGGER_STATEMENT => JsDebuggerStatement::cast(self.clone())
				.unwrap()
				.to_format_element(formatter),
			SyntaxKind::FOR_IN_STMT => ForInStmt::cast(self.clone())
				.unwrap()
				.to_format_element(formatter),
			SyntaxKind::ARRAY_PATTERN => ArrayPattern::cast(self.clone())
				.unwrap()
				.to_format_element(formatter),
			SyntaxKind::CALL_EXPR => CallExpr::cast(self.clone())
				.unwrap()
				.to_format_element(formatter),
			SyntaxKind::ARG_LIST => ArgList::cast(self.clone())
				.unwrap()
				.to_format_element(formatter),
			SyntaxKind::LITERAL_PROP => LiteralProp::cast(self.clone())
				.unwrap()
				.to_format_element(formatter),
			SyntaxKind::CLASS_DECL => ClassDecl::cast(self.clone())
				.unwrap()
				.to_format_element(formatter),
			SyntaxKind::CLASS_BODY => ClassBody::cast(self.clone())
				.unwrap()
				.to_format_element(formatter),
			SyntaxKind::CONSTRUCTOR_PARAMETERS => ConstructorParameters::cast(self.clone())
				.unwrap()
				.to_format_element(formatter),
			SyntaxKind::GETTER => Getter::cast(self.clone())
				.unwrap()
				.to_format_element(formatter),
			SyntaxKind::SETTER => Setter::cast(self.clone())
				.unwrap()
				.to_format_element(formatter),
			SyntaxKind::CLASS_PROP => ClassProp::cast(self.clone())
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
	fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
		match self.kind() {
			SyntaxKind::STRING => rslint_parser::ast::String::cast(self.clone())
				.unwrap()
				.to_format_element(formatter),
			_ => Ok(token(self.text())),
		}
	}
}
