use crate::{format_elements, space_token, token, FormatElement, Formatter, ToFormatElement};
use rslint_parser::ast::DoWhileStmt;

impl ToFormatElement for DoWhileStmt {
	fn to_format_element(&self, formatter: &Formatter) -> FormatElement {
		let do_token = formatter.format_token(&self.do_token().expect("do token missing"));

		let condition = formatter.format_node(self.condition().expect("Condition doesn't exist"));
		let cons = formatter.format_node(self.cons().expect("Do while misses the body"));
		let while_token = formatter.format_token(&self.while_token().expect("while token missing"));
		let semi_colon = token(";");

		format_elements![
			do_token,
			space_token(),
			cons,
			space_token(),
			while_token,
			space_token(),
			condition,
			semi_colon
		]
	}
}
