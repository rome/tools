use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_parser::ast::JsAnyExpression;

impl ToFormatElement for JsAnyExpression {
	fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
		match self {
			JsAnyExpression::ArrowExpr(arrow) => arrow.to_format_element(formatter),
			JsAnyExpression::JsAnyLiteral(literal) => literal.to_format_element(formatter),
			JsAnyExpression::Template(_) => todo!(),
			JsAnyExpression::NameRef(name_ref) => name_ref.to_format_element(formatter),
			JsAnyExpression::ThisExpr(_) => todo!(),
			JsAnyExpression::JsArrayExpression(array_expression) => {
				array_expression.to_format_element(formatter)
			}
			JsAnyExpression::ObjectExpr(object_expression) => {
				object_expression.to_format_element(formatter)
			}
			JsAnyExpression::GroupingExpr(_) => todo!(),
			JsAnyExpression::BracketExpr(_) => todo!(),
			JsAnyExpression::DotExpr(_) => todo!(),
			JsAnyExpression::NewExpr(_) => todo!(),
			JsAnyExpression::CallExpr(call_expression) => {
				call_expression.to_format_element(formatter)
			}
			JsAnyExpression::UnaryExpr(_) => todo!(),
			JsAnyExpression::JsBinaryExpression(_) => todo!(),
			JsAnyExpression::JsConditionalExpression(_) => todo!(),
			JsAnyExpression::AssignExpr(_) => todo!(),
			JsAnyExpression::SequenceExpr(expr) => expr.to_format_element(formatter),
			JsAnyExpression::FnExpr(_) => todo!(),
			JsAnyExpression::ClassExpr(_) => todo!(),
			JsAnyExpression::NewTarget(_) => todo!(),
			JsAnyExpression::ImportMeta(_) => todo!(),
			JsAnyExpression::SuperCall(super_call) => super_call.to_format_element(formatter),
			JsAnyExpression::ImportCall(_) => todo!(),
			JsAnyExpression::YieldExpr(_) => todo!(),
			JsAnyExpression::JsAwaitExpression(_) => todo!(),
			JsAnyExpression::PrivatePropAccess(_) => todo!(),
			JsAnyExpression::TsNonNull(_) => todo!(),
			JsAnyExpression::TsAssertion(_) => todo!(),
			JsAnyExpression::TsConstAssertion(_) => todo!(),
			JsAnyExpression::PreUpdateExpression(_) => todo!(),
			JsAnyExpression::PostUpdateExpression(_) => todo!(),
			JsAnyExpression::JsUnknownExpression(_) => todo!(),
			JsAnyExpression::JsLogicalExpression(_) => todo!(),
		}
	}
}
