use crate::{FormatElement, Formatter, ToFormatElement};
use rslint_parser::ast::ExprOrBlock;

impl ToFormatElement for ExprOrBlock {
	fn to_format_element(&self, formatter: &Formatter) -> Option<FormatElement> {
		match self {
			ExprOrBlock::Block(block) => block.to_format_element(formatter),
			ExprOrBlock::Expr(expr) => expr.to_format_element(formatter),
		}
	}
}
