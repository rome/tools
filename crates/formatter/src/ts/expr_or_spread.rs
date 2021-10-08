use crate::{FormatContext, FormatElement, ToFormatElement};
use rslint_parser::ast::ExprOrSpread;

impl ToFormatElement for ExprOrSpread {
	fn to_format_element(&self, context: &FormatContext) -> FormatElement {
		match self {
			ExprOrSpread::Spread(_) => todo!(),
			ExprOrSpread::Expr(expr) => expr.to_format_element(context),
		}
	}
}
