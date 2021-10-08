use rslint_parser::ast::ExprStmt;

use crate::{FormatContext, FormatElement, ToFormatElement};

impl ToFormatElement for ExprStmt {
	fn to_format_element(&self, context: &FormatContext) -> FormatElement {
		context.format_node(self.expr().expect("Strange error?"))
	}
}
