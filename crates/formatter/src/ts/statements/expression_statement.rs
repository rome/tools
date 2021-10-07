use rslint_parser::ast::ExprStmt;

use crate::{FormatElement, ToFormatElement};

impl ToFormatElement for ExprStmt {
	fn to_format_element(&self) -> FormatElement {
		if let Some(expr) = self.expr() {
			return expr.to_format_element();
		}
		// TODO: understand what to do here
		panic!("Strange error?")
	}
}
