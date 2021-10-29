use crate::{FormatElement, Formatter, ToFormatElement};
use rslint_parser::ast::ExprOrSpread;

impl ToFormatElement for ExprOrSpread {
	fn to_format_element(&self, formatter: &Formatter) -> Option<FormatElement> {
		match self {
			ExprOrSpread::SpreadElement(spread) => spread.to_format_element(formatter),
			ExprOrSpread::Expr(expr) => expr.to_format_element(formatter),
			ExprOrSpread::Literal(literal) => literal.to_format_element(formatter),
			ExprOrSpread::ObjectExpr(object_expression) => {
				object_expression.to_format_element(formatter)
			}
		}
	}
}
