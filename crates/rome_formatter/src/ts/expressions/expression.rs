use crate::{
	empty_element, format_elements, space_token, token, FormatElement, FormatResult, Formatter,
	ToFormatElement,
};
use rslint_parser::ast::{
	JsAnyExpression, JsAssignmentExpression, JsAwaitExpression, JsBinaryExpression,
	JsComputedMemberExpression, JsConditionalExpression, JsLogicalExpression, JsNewExpression,
	JsParenthesizedExpression, JsThisExpression, JsUnaryExpression, JsYieldArgument,
	JsYieldExpression, NewTarget,
};
use rslint_parser::{token_set, T};

impl ToFormatElement for JsAnyExpression {
	fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
		match self {
			JsAnyExpression::JsArrowFunctionExpression(arrow) => arrow.to_format_element(formatter),
			JsAnyExpression::JsAnyLiteralExpression(literal) => {
				literal.to_format_element(formatter)
			}
			JsAnyExpression::Template(_) => todo!(),
			JsAnyExpression::JsIdentifierExpression(identifier_expr) => {
				identifier_expr.to_format_element(formatter)
			}
			JsAnyExpression::JsThisExpression(this_expression) => {
				this_expression.to_format_element(formatter)
			}
			JsAnyExpression::JsArrayExpression(array_expression) => {
				array_expression.to_format_element(formatter)
			}
			JsAnyExpression::JsObjectExpression(object_expression) => {
				object_expression.to_format_element(formatter)
			}
			JsAnyExpression::JsParenthesizedExpression(parenthesized_expression) => {
				parenthesized_expression.to_format_element(formatter)
			}
			JsAnyExpression::JsComputedMemberExpression(computed_member_expression) => {
				computed_member_expression.to_format_element(formatter)
			}
			JsAnyExpression::JsStaticMemberExpression(static_member_expression) => {
				static_member_expression.to_format_element(formatter)
			}
			JsAnyExpression::JsNewExpression(new_expr) => new_expr.to_format_element(formatter),
			JsAnyExpression::JsCallExpression(call_expression) => {
				call_expression.to_format_element(formatter)
			}
			JsAnyExpression::JsUnaryExpression(unary_expression) => {
				unary_expression.to_format_element(formatter)
			}
			JsAnyExpression::JsBinaryExpression(binary_expression) => {
				binary_expression.to_format_element(formatter)
			}
			JsAnyExpression::JsConditionalExpression(conditional_expression) => {
				conditional_expression.to_format_element(formatter)
			}
			JsAnyExpression::JsAssignmentExpression(assignment_expression) => {
				assignment_expression.to_format_element(formatter)
			}
			JsAnyExpression::JsSequenceExpression(expr) => expr.to_format_element(formatter),
			JsAnyExpression::JsFunctionExpression(function_expression) => {
				function_expression.to_format_element(formatter)
			}
			JsAnyExpression::JsClassExpression(class_expression) => {
				class_expression.to_format_element(formatter)
			}
			JsAnyExpression::NewTarget(expr) => expr.to_format_element(formatter),
			JsAnyExpression::ImportMeta(_) => todo!(),
			JsAnyExpression::JsImportCallExpression(import_call_expr) => {
				import_call_expr.to_format_element(formatter)
			}
			JsAnyExpression::JsYieldExpression(yield_expression) => {
				yield_expression.to_format_element(formatter)
			}
			JsAnyExpression::JsAwaitExpression(await_expression) => {
				await_expression.to_format_element(formatter)
			}
			JsAnyExpression::TsNonNull(_) => todo!(),
			JsAnyExpression::TsAssertion(_) => todo!(),
			JsAnyExpression::TsConstAssertion(_) => todo!(),
			JsAnyExpression::JsPreUpdateExpression(pre_update_expression) => {
				pre_update_expression.to_format_element(formatter)
			}
			JsAnyExpression::JsPostUpdateExpression(post_update_expression) => {
				post_update_expression.to_format_element(formatter)
			}
			JsAnyExpression::JsUnknownExpression(_) => todo!(),
			JsAnyExpression::JsLogicalExpression(logical_expression) => {
				logical_expression.to_format_element(formatter)
			}
			JsAnyExpression::JsSuperExpression(expr) => expr.to_format_element(formatter),
		}
	}
}

impl ToFormatElement for JsThisExpression {
	fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
		formatter.format_token(&self.this_token()?)
	}
}

impl ToFormatElement for JsParenthesizedExpression {
	fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
		Ok(format_elements![
			formatter.format_token(&self.l_paren_token()?)?,
			formatter.format_node(self.expression()?)?,
			formatter.format_token(&self.r_paren_token()?)?
		])
	}
}

impl ToFormatElement for JsComputedMemberExpression {
	fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
		let optional_chain_token = if let Some(chain_token) = self.optional_chain_token() {
			formatter.format_token(&chain_token)?
		} else {
			empty_element()
		};

		Ok(format_elements![
			formatter.format_node(self.object()?)?,
			optional_chain_token,
			formatter.format_token(&self.l_brack_token()?)?,
			formatter.format_node(self.member()?)?,
			formatter.format_token(&self.r_brack_token()?)?
		])
	}
}

impl ToFormatElement for JsNewExpression {
	fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
		let arguments = if let Some(arguments) = self.arguments() {
			formatter.format_node(arguments)?
		} else {
			format_elements![token("("), token(")")]
		};

		Ok(format_elements![
			formatter.format_token(&self.new_token()?)?,
			// TODO handle TsTypeArgs
			space_token(),
			formatter.format_node(self.callee()?)?,
			arguments,
		])
	}
}

impl ToFormatElement for JsUnaryExpression {
	fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
		let operator = self.operator()?;
		let space_or_empty =
			if token_set![T![delete], T![void], T![typeof]].contains(operator.kind()) {
				space_token()
			} else {
				empty_element()
			};
		Ok(format_elements![
			formatter.format_token(&self.operator()?)?,
			space_or_empty,
			formatter.format_node(self.argument()?)?,
		])
	}
}

impl ToFormatElement for JsBinaryExpression {
	fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
		Ok(format_elements![
			formatter.format_node(self.left()?)?,
			space_token(),
			formatter.format_token(&self.operator()?)?,
			space_token(),
			formatter.format_node(self.right()?)?
		])
	}
}

impl ToFormatElement for JsConditionalExpression {
	fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
		Ok(format_elements![
			formatter.format_node(self.test()?)?,
			space_token(),
			formatter.format_token(&self.question_mark_token()?)?,
			space_token(),
			formatter.format_node(self.consequent()?)?,
			space_token(),
			formatter.format_token(&self.colon_token()?)?,
			space_token(),
			formatter.format_node(self.alternate()?)?,
		])
	}
}

impl ToFormatElement for JsAssignmentExpression {
	fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
		Ok(format_elements![
			formatter.format_node(self.left()?)?,
			space_token(),
			formatter.format_token(&self.operator_token()?)?,
			space_token(),
			formatter.format_node(self.right()?)?,
		])
	}
}

impl ToFormatElement for NewTarget {
	fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
		Ok(format_elements![
			formatter.format_token(&self.new_token()?)?,
			formatter.format_token(&self.dot_token()?)?,
			formatter.format_token(&self.target_token()?)?,
		])
	}
}

impl ToFormatElement for JsYieldExpression {
	fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
		let argument = if let Some(node) = self.argument() {
			formatter.format_node(node)?
		} else {
			empty_element()
		};

		Ok(format_elements![
			formatter.format_token(&self.yield_token()?)?,
			argument
		])
	}
}

impl ToFormatElement for JsYieldArgument {
	fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
		let star_token = if let Some(token) = self.star_token() {
			formatter.format_token(&token)?
		} else {
			empty_element()
		};

		Ok(format_elements![
			star_token,
			space_token(),
			formatter.format_node(self.expression()?)?
		])
	}
}

impl ToFormatElement for JsAwaitExpression {
	fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
		Ok(format_elements![
			formatter.format_token(&self.await_token()?)?,
			space_token(),
			formatter.format_node(self.argument()?)?,
		])
	}
}

impl ToFormatElement for JsLogicalExpression {
	fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
		Ok(format_elements![
			formatter.format_node(self.left()?)?,
			space_token(),
			formatter.format_token(&self.operator()?)?,
			space_token(),
			formatter.format_node(self.right()?)?,
		])
	}
}
