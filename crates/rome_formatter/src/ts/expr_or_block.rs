use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_parser::ast::ExprOrBlock;

impl ToFormatElement for ExprOrBlock {
	fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
		match self {
			ExprOrBlock::BlockStmt(block) => block.to_format_element(formatter),
			ExprOrBlock::JsAnyExpression(expr) => expr.to_format_element(formatter),
		}
	}
}
