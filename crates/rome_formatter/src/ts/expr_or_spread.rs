use crate::{FormatElement, FormatError, Formatter, ToFormatElement};
use rslint_parser::ast::ExprOrSpread;

impl ToFormatElement for ExprOrSpread {
	fn to_format_element(&self, formatter: &Formatter) -> Result<FormatElement, FormatError> {
		match self {
			ExprOrSpread::SpreadElement(spread) => spread.to_format_element(formatter),
			ExprOrSpread::Expr(expr) => expr.to_format_element(formatter),
			ExprOrSpread::GroupingExpr(_) => todo!(),
		}
	}
}
