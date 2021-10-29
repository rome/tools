use rslint_parser::ast::ExprStmt;

use crate::{format_elements, token, FormatElement, Formatter, ToFormatElement};

impl ToFormatElement for ExprStmt {
	fn to_format_element(&self, formatter: &Formatter) -> Option<FormatElement> {
		dbg!(self);
		Some(format_elements![
			formatter.format_node(self.expr()?)?,
			token(";")
		])
	}
}
