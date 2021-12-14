use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_parser::ast::{
	CallExpr, ForStmt, ForStmtTest, ForStmtUpdate, JsArrayBindingPattern, JsArrayExpression,
	JsArrowFunctionExpression, JsBlockStatement, JsBooleanLiteralExpression, JsCallArguments,
	JsCaseClause, JsCatchClause, JsClassDeclaration, JsConstructorParameters, JsContinueStatement,
	JsDebuggerStatement, JsDefaultClause, JsDoWhileStatement, JsEmptyStatement,
	JsExpressionStatement, JsFinallyClause, JsForInStatement, JsFunctionDeclaration,
	JsGetterClassMember, JsIdentifierBinding, JsIdentifierExpression, JsIfStatement,
	JsLabeledStatement, JsNullLiteralExpression, JsNumberLiteralExpression, JsObjectExpression,
	JsParameters, JsPropertyClassMember, JsPropertyObjectMember, JsReturnStatement, JsScript,
	JsSequenceExpression, JsSetterClassMember, JsShorthandPropertyObjectMember, JsSpread,
	JsStringLiteralExpression, JsSwitchStatement, JsTryStatement, JsVariableDeclaration,
	JsVariableStatement, JsWhileStatement, JsWithStatement,
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
			SyntaxKind::JS_IDENTIFIER_BINDING => JsIdentifierBinding::cast(self.clone())
				.unwrap()
				.to_format_element(formatter),
			SyntaxKind::JS_NULL_LITERAL_EXPRESSION => JsNullLiteralExpression::cast(self.clone())
				.unwrap()
				.to_format_element(formatter),
			SyntaxKind::JS_NUMBER_LITERAL_EXPRESSION => {
				JsNumberLiteralExpression::cast(self.clone())
					.unwrap()
					.to_format_element(formatter)
			}
			SyntaxKind::JS_IDENTIFIER_EXPRESSION => JsIdentifierExpression::cast(self.clone())
				.unwrap()
				.to_format_element(formatter),
			SyntaxKind::JS_PARAMETERS => JsParameters::cast(self.clone())
				.unwrap()
				.to_format_element(formatter),
			SyntaxKind::JS_SCRIPT => JsScript::cast(self.clone())
				.unwrap()
				.to_format_element(formatter),
			SyntaxKind::JS_SPREAD => JsSpread::cast(self.clone())
				.unwrap()
				.to_format_element(formatter),
			SyntaxKind::JS_VARIABLE_STATEMENT => JsVariableStatement::cast(self.clone())
				.unwrap()
				.to_format_element(formatter),
			SyntaxKind::JS_VARIABLE_DECLARATION => JsVariableDeclaration::cast(self.clone())
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
			SyntaxKind::JS_FOR_IN_STATEMENT => JsForInStatement::cast(self.clone())
				.unwrap()
				.to_format_element(formatter),
			SyntaxKind::JS_ARRAY_BINDING_PATTERN => JsArrayBindingPattern::cast(self.clone())
				.unwrap()
				.to_format_element(formatter),
			SyntaxKind::CALL_EXPR => CallExpr::cast(self.clone())
				.unwrap()
				.to_format_element(formatter),
			SyntaxKind::JS_CALL_ARGUMENTS => JsCallArguments::cast(self.clone())
				.unwrap()
				.to_format_element(formatter),
			SyntaxKind::JS_PROPERTY_OBJECT_MEMBER => JsPropertyObjectMember::cast(self.clone())
				.unwrap()
				.to_format_element(formatter),
			SyntaxKind::JS_CLASS_DECLARATION => JsClassDeclaration::cast(self.clone())
				.unwrap()
				.to_format_element(formatter),
			SyntaxKind::JS_CONSTRUCTOR_PARAMETERS => JsConstructorParameters::cast(self.clone())
				.unwrap()
				.to_format_element(formatter),
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
