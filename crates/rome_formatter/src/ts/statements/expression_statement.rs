use rslint_parser::ast::ExprStmt;

use crate::{format_elements, token, FormatElement, FormatError, Formatter, ToFormatElement};

impl ToFormatElement for ExprStmt {
	fn to_format_element(&self, formatter: &Formatter) -> Result<FormatElement, FormatError> {
		Ok(format_elements![
			formatter.format_node(self.expr()?)?,
			token(";")
		])
	}
}
