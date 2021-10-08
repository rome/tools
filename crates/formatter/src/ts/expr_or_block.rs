use crate::{FormatContext, FormatElement, ToFormatElement};
use rslint_parser::ast::ExprOrBlock;

impl ToFormatElement for ExprOrBlock {
	fn to_format_element(&self, context: &FormatContext) -> FormatElement {
		match self {
			ExprOrBlock::Block(block) => block.to_format_element(context),
			ExprOrBlock::Expr(expr) => expr.to_format_element(context),
		}
	}
}
