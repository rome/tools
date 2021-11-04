use crate::{FormatElement, FormatError, Formatter, ToFormatElement};
use rslint_parser::ast::ExprOrBlock;

impl ToFormatElement for ExprOrBlock {
	fn to_format_element(&self, formatter: &Formatter) -> Result<FormatElement, FormatError> {
		match self {
			ExprOrBlock::BlockStmt(block) => block.to_format_element(formatter),
			ExprOrBlock::Expr(expr) => expr.to_format_element(formatter),
		}
	}
}
