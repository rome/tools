use crate::{FormatElement, Formatter, ToFormatElement};
use rslint_parser::ast::ExprOrSpread;

impl ToFormatElement for ExprOrSpread {
	fn to_format_element(&self, formatter: &Formatter) -> FormatElement {
		match self {
			ExprOrSpread::Spread(spread) => spread.to_format_element(formatter),
			ExprOrSpread::Expr(expr) => expr.to_format_element(formatter),
		}
	}
}
