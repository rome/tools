use crate::ast::{
	JsAnyArrowFunctionParameters, JsAnyBinding, JsAnyClass, JsAnyFunction, JsAnyFunctionBody,
	JsClassMemberList, JsExtendsClause, TsImplementsClause, TsTypeAnnotation, TsTypeParams,
};
use crate::{SyntaxResult, SyntaxToken};

impl JsAnyClass {
	pub fn class_token(&self) -> SyntaxResult<SyntaxToken> {
		match self {
			JsAnyClass::JsClassStatement(statement) => statement.class_token(),
			JsAnyClass::JsClassExpression(expression) => expression.class_token(),
		}
	}

	pub fn id(&self) -> SyntaxResult<Option<JsAnyBinding>> {
		match self {
			JsAnyClass::JsClassStatement(statement) => statement.id().map(Some),
			JsAnyClass::JsClassExpression(expression) => Ok(expression.id()),
		}
	}

	pub fn extends_clause(&self) -> Option<JsExtendsClause> {
		match self {
			JsAnyClass::JsClassStatement(statement) => statement.extends_clause(),
			JsAnyClass::JsClassExpression(expression) => expression.extends_clause(),
		}
	}

	pub fn implements_clause(&self) -> Option<TsImplementsClause> {
		match self {
			JsAnyClass::JsClassStatement(statement) => statement.implements_clause(),
			JsAnyClass::JsClassExpression(expression) => expression.implements_clause(),
		}
	}

	pub fn l_curly_token(&self) -> SyntaxResult<SyntaxToken> {
		match self {
			JsAnyClass::JsClassStatement(statement) => statement.l_curly_token(),
			JsAnyClass::JsClassExpression(expression) => expression.l_curly_token(),
		}
	}

	pub fn members(&self) -> JsClassMemberList {
		match self {
			JsAnyClass::JsClassStatement(statement) => statement.members(),
			JsAnyClass::JsClassExpression(expression) => expression.members(),
		}
	}

	pub fn r_curly_token(&self) -> SyntaxResult<SyntaxToken> {
		match self {
			JsAnyClass::JsClassStatement(statement) => statement.r_curly_token(),
			JsAnyClass::JsClassExpression(expression) => expression.r_curly_token(),
		}
	}
}

impl JsAnyFunction {
	pub fn async_token(&self) -> Option<SyntaxToken> {
		match self {
			JsAnyFunction::JsArrowFunctionExpression(expr) => expr.async_token(),
			JsAnyFunction::JsFunctionExpression(expr) => expr.async_token(),
			JsAnyFunction::JsFunctionStatement(statement) => statement.async_token(),
		}
	}

	pub fn is_async(&self) -> bool {
		self.async_token().is_some()
	}

	pub fn function_token(&self) -> SyntaxResult<Option<SyntaxToken>> {
		match self {
			JsAnyFunction::JsArrowFunctionExpression(_) => Ok(None),
			JsAnyFunction::JsFunctionExpression(expr) => expr.function_token().map(Some),
			JsAnyFunction::JsFunctionStatement(statement) => statement.function_token().map(Some),
		}
	}

	pub fn star_token(&self) -> Option<SyntaxToken> {
		match self {
			JsAnyFunction::JsArrowFunctionExpression(_) => None,
			JsAnyFunction::JsFunctionExpression(expr) => expr.star_token(),
			JsAnyFunction::JsFunctionStatement(statement) => statement.star_token(),
		}
	}

	pub fn is_generator(&self) -> bool {
		self.star_token().is_some()
	}

	pub fn id(&self) -> SyntaxResult<Option<JsAnyBinding>> {
		match self {
			JsAnyFunction::JsArrowFunctionExpression(_) => Ok(None),
			JsAnyFunction::JsFunctionExpression(expr) => Ok(expr.id()),
			JsAnyFunction::JsFunctionStatement(statement) => statement.id().map(Some),
		}
	}

	pub fn type_parameters(&self) -> Option<TsTypeParams> {
		match self {
			JsAnyFunction::JsArrowFunctionExpression(expr) => expr.type_parameters(),
			JsAnyFunction::JsFunctionExpression(expr) => expr.type_parameters(),
			JsAnyFunction::JsFunctionStatement(statement) => statement.type_parameters(),
		}
	}

	pub fn parameters(&self) -> SyntaxResult<JsAnyArrowFunctionParameters> {
		match self {
			JsAnyFunction::JsArrowFunctionExpression(expr) => expr.parameters(),
			JsAnyFunction::JsFunctionExpression(expr) => expr
				.parameters()
				.map(JsAnyArrowFunctionParameters::JsParameters),
			JsAnyFunction::JsFunctionStatement(statement) => statement
				.parameters()
				.map(JsAnyArrowFunctionParameters::JsParameters),
		}
	}

	pub fn return_type(&self) -> Option<TsTypeAnnotation> {
		match self {
			JsAnyFunction::JsArrowFunctionExpression(expr) => expr.return_type(),
			JsAnyFunction::JsFunctionExpression(expr) => expr.return_type(),
			JsAnyFunction::JsFunctionStatement(statement) => statement.return_type(),
		}
	}

	pub fn body(&self) -> SyntaxResult<JsAnyFunctionBody> {
		match self {
			JsAnyFunction::JsArrowFunctionExpression(expr) => expr.body(),
			JsAnyFunction::JsFunctionExpression(expr) => {
				expr.body().map(JsAnyFunctionBody::JsFunctionBody)
			}
			JsAnyFunction::JsFunctionStatement(statement) => {
				statement.body().map(JsAnyFunctionBody::JsFunctionBody)
			}
		}
	}
}
