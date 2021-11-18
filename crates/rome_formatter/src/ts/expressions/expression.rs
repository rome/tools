use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_parser::ast::JsAnyExpression;

impl ToFormatElement for JsAnyExpression {
	fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
		match self {
			JsAnyExpression::JsArrowFunctionExpression(arrow) => arrow.to_format_element(formatter),
			JsAnyExpression::JsAnyLiteralExpression(literal) => {
				literal.to_format_element(formatter)
			}
			JsAnyExpression::Template(_) => todo!(),
			JsAnyExpression::JsReferenceIdentifierExpression(name_ref) => {
				name_ref.to_format_element(formatter)
			}
			JsAnyExpression::JsThisExpression(_) => todo!(),
			JsAnyExpression::JsArrayExpression(array_expression) => {
				array_expression.to_format_element(formatter)
			}
			JsAnyExpression::JsObjectExpression(object_expression) => {
				object_expression.to_format_element(formatter)
			}
			JsAnyExpression::JsParenthesizedExpression(_) => todo!(),
			JsAnyExpression::JsComputedMemberExpression(_) => todo!(),
			JsAnyExpression::JsStaticMemberExpression(_) => todo!(),
			JsAnyExpression::NewExpr(_) => todo!(),
			JsAnyExpression::CallExpr(call_expression) => {
				call_expression.to_format_element(formatter)
			}
			JsAnyExpression::JsUnaryExpression(_) => todo!(),
			JsAnyExpression::JsBinaryExpression(_) => todo!(),
			JsAnyExpression::JsConditionalExpression(_) => todo!(),
			JsAnyExpression::JsAssignmentExpression(_) => todo!(),
			JsAnyExpression::JsSequenceExpression(expr) => expr.to_format_element(formatter),
			JsAnyExpression::JsFunctionExpression(_) => todo!(),
			JsAnyExpression::JsClassExpression(_) => todo!(),
			JsAnyExpression::NewTarget(_) => todo!(),
			JsAnyExpression::ImportMeta(_) => todo!(),
			JsAnyExpression::JsImportCallExpression(_) => todo!(),
			JsAnyExpression::JsYieldExpression(_) => todo!(),
			JsAnyExpression::JsAwaitExpression(_) => todo!(),
			JsAnyExpression::TsNonNull(_) => todo!(),
			JsAnyExpression::TsAssertion(_) => todo!(),
			JsAnyExpression::TsConstAssertion(_) => todo!(),
			JsAnyExpression::JsPreUpdateExpression(_) => todo!(),
			JsAnyExpression::JsPostUpdateExpression(_) => todo!(),
			JsAnyExpression::JsUnknownExpression(_) => todo!(),
			JsAnyExpression::JsLogicalExpression(_) => todo!(),
			JsAnyExpression::JsSuperExpression(expr) => expr.to_format_element(formatter),
		}
	}
}
