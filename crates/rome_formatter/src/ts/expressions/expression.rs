use crate::{FormatElement, FormatError, Formatter, ToFormatElement};
use rslint_parser::ast::Expr;

impl ToFormatElement for Expr {
	fn to_format_element(&self, formatter: &Formatter) -> Result<FormatElement, FormatError> {
		match self {
			Expr::ArrowExpr(arrow) => arrow.to_format_element(formatter),
			Expr::Literal(literal) => literal.to_format_element(formatter),
			Expr::Template(_) => todo!(),
			Expr::NameRef(name_ref) => name_ref.to_format_element(formatter),
			Expr::ThisExpr(_) => todo!(),
			Expr::ArrayExpr(array_expression) => array_expression.to_format_element(formatter),
			Expr::ObjectExpr(object_expression) => object_expression.to_format_element(formatter),
			Expr::GroupingExpr(_) => todo!(),
			Expr::BracketExpr(_) => todo!(),
			Expr::DotExpr(_) => todo!(),
			Expr::NewExpr(_) => todo!(),
			Expr::CallExpr(call_expression) => call_expression.to_format_element(formatter),
			Expr::UnaryExpr(_) => todo!(),
			Expr::BinExpr(_) => todo!(),
			Expr::CondExpr(_) => todo!(),
			Expr::AssignExpr(_) => todo!(),
			Expr::SequenceExpr(expr) => expr.to_format_element(formatter),
			Expr::FnExpr(_) => todo!(),
			Expr::ClassExpr(_) => todo!(),
			Expr::NewTarget(_) => todo!(),
			Expr::ImportMeta(_) => todo!(),
			Expr::SuperCall(super_call) => super_call.to_format_element(formatter),
			Expr::ImportCall(_) => todo!(),
			Expr::YieldExpr(_) => todo!(),
			Expr::AwaitExpr(_) => todo!(),
			Expr::PrivatePropAccess(_) => todo!(),
			Expr::TsNonNull(_) => todo!(),
			Expr::TsAssertion(_) => todo!(),
			Expr::TsConstAssertion(_) => todo!(),
		}
	}
}
