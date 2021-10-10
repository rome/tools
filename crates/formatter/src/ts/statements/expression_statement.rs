use rslint_parser::ast::ExprStmt;

use crate::{FormatElement, Formatter, ToFormatElement};

impl ToFormatElement for ExprStmt {
	fn to_format_element(&self, formatter: &Formatter) -> FormatElement {
		formatter.format_node(self.expr().expect("Strange error?"))
	}
}
