use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_parser::ast::{
	ArgList, ArrayPattern, AssignPattern, CallExpr, ForInStmt, ForStmt, ForStmtInit, ForStmtTest,
	ForStmtUpdate, JsArrayExpression, JsArrowFunctionExpression, JsBlockStatement,
	JsBooleanLiteralExpression, JsCaseClause, JsCatchClause, JsClassDeclaration,
	JsConstructorParameterList, JsContinueStatement, JsDebuggerStatement, JsDefaultClause,
	JsDoWhileStatement, JsEmptyStatement, JsExpressionStatement, JsFinallyClause,
	JsFunctionDeclaration, JsGetterClassMember, JsIfStatement, JsLabeledStatement,
	JsNullLiteralExpression, JsNumberLiteralExpression, JsObjectExpression, JsParameterList,
	JsPropertyClassMember, JsPropertyObjectMember, JsReferenceIdentifierExpression,
	JsReturnStatement, JsRoot, JsSequenceExpression, JsSetterClassMember,
	JsShorthandPropertyObjectMember, JsStringLiteralExpression, JsSwitchStatement, JsTryStatement,
	JsVariableDeclarationStatement, JsVariableDeclarator, JsWhileStatement, JsWithStatement, Name,
	SinglePattern,
};
use rslint_parser::{AstNode, SyntaxKind, SyntaxNode};

impl ToFormatElement for SyntaxNode {
	fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
		match self.kind() {
			SyntaxKind::JS_ARRAY_EXPRESSION => JsArrayExpression::cast(self.clone())
				.unwrap()
				.to_format_element(formatter),
			SyntaxKind::JS_ARROW_FUNCTION_EXPRESSION => {
				JsArrowFunctionExpression::cast(self.clone())
					.unwrap()
					.to_format_element(formatter)
			}
			SyntaxKind::ASSIGN_PATTERN => AssignPattern::cast(self.clone())
				.unwrap()
				.to_format_element(formatter),
			SyntaxKind::JS_BOOLEAN_LITERAL_EXPRESSION => {
				JsBooleanLiteralExpression::cast(self.clone())
					.unwrap()
					.to_format_element(formatter)
			}
			SyntaxKind::JS_STRING_LITERAL_EXPRESSION => {
				JsStringLiteralExpression::cast(self.clone())
					.unwrap()
					.to_format_element(formatter)
			}
			SyntaxKind::JS_NULL_LITERAL_EXPRESSION => JsNullLiteralExpression::cast(self.clone())
				.unwrap()
				.to_format_element(formatter),
			SyntaxKind::JS_NUMBER_LITERAL_EXPRESSION => {
				JsNumberLiteralExpression::cast(self.clone())
					.unwrap()
					.to_format_element(formatter)
			}
			SyntaxKind::NAME => Name::cast(self.clone())
				.unwrap()
				.to_format_element(formatter),
			SyntaxKind::JS_REFERENCE_IDENTIFIER_EXPRESSION => {
				JsReferenceIdentifierExpression::cast(self.clone())
					.unwrap()
					.to_format_element(formatter)
			}
			SyntaxKind::JS_PARAMETER_LIST => JsParameterList::cast(self.clone())
				.unwrap()
				.to_format_element(formatter),
			SyntaxKind::JS_ROOT => JsRoot::cast(self.clone())
				.unwrap()
				.to_format_element(formatter),
			SyntaxKind::SINGLE_PATTERN => SinglePattern::cast(self.clone())
				.unwrap()
				.to_format_element(formatter),
			SyntaxKind::SPREAD_ELEMENT => SinglePattern::cast(self.clone())
				.unwrap()
				.to_format_element(formatter),
			SyntaxKind::JS_VARIABLE_DECLARATION_STATEMENT => {
				JsVariableDeclarationStatement::cast(self.clone())
					.unwrap()
					.to_format_element(formatter)
			}
			SyntaxKind::JS_VARIABLE_DECLARATOR => JsVariableDeclarator::cast(self.clone())
				.unwrap()
				.to_format_element(formatter),
			SyntaxKind::JS_FUNCTION_DECLARATION => JsFunctionDeclaration::cast(self.clone())
				.unwrap()
				.to_format_element(formatter),
			SyntaxKind::JS_SEQUENCE_EXPRESSION => JsSequenceExpression::cast(self.clone())
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
			SyntaxKind::JS_SHORTHAND_PROPERTY_OBJECT_MEMBER => {
				JsShorthandPropertyObjectMember::cast(self.clone())
					.unwrap()
					.to_format_element(formatter)
			}
			SyntaxKind::JS_OBJECT_EXPRESSION => JsObjectExpression::cast(self.clone())
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
			SyntaxKind::JS_PROPERTY_OBJECT_MEMBER => JsPropertyObjectMember::cast(self.clone())
				.unwrap()
				.to_format_element(formatter),
			SyntaxKind::JS_CLASS_DECLARATION => JsClassDeclaration::cast(self.clone())
				.unwrap()
				.to_format_element(formatter),
			SyntaxKind::JS_CONSTRUCTOR_PARAMETER_LIST => {
				JsConstructorParameterList::cast(self.clone())
					.unwrap()
					.to_format_element(formatter)
			}
			SyntaxKind::JS_GETTER_CLASS_MEMBER => JsGetterClassMember::cast(self.clone())
				.unwrap()
				.to_format_element(formatter),
			SyntaxKind::JS_SETTER_CLASS_MEMBER => JsSetterClassMember::cast(self.clone())
				.unwrap()
				.to_format_element(formatter),
			SyntaxKind::JS_PROPERTY_CLASS_MEMBER => JsPropertyClassMember::cast(self.clone())
				.unwrap()
				.to_format_element(formatter),

			_ => todo!(
				"Implement formatting for the {:?} syntax kind.",
				self.kind()
			),
		}
	}
}
