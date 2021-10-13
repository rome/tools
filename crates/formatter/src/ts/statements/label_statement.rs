use crate::{format_elements, space_token, token, FormatElement, Formatter, ToFormatElement};
use rslint_parser::ast::LabelledStmt;

impl ToFormatElement for LabelledStmt {
	fn to_format_element(&self, formatter: &Formatter) -> FormatElement {
		let label = formatter.format_node(self.label().expect("label token is missing"));
		let statement = formatter.format_node(self.stmt().expect("statement is missing"));

		format_elements![label, token(":"), space_token(), statement]
	}
}
