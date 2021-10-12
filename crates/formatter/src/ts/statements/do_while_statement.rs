use rslint_parser::ast::DoWhileStmt;

use crate::{format_elements, space_token, token, FormatElement, ToFormatElement};

impl ToFormatElement for DoWhileStmt {
	fn to_format_element(&self, formatter: &crate::Formatter) -> FormatElement {
		let do_token = if let Some(do_token) = self.do_token() {
			formatter.format_token(&do_token)
		} else {
			token("do")
		};

		let condition = formatter.format_node(self.condition().expect("Condition doesn't exist"));
		let cons = formatter.format_node(self.cons().expect("Do while misses the body"));
		let while_token = if let Some(while_token) = self.while_token() {
			formatter.format_token(&while_token)
		} else {
			token("while")
		};
		let semi_colon = if let Some(semi_colon) = self.semicolon_token() {
			formatter.format_token(&semi_colon)
		} else {
			token(";")
		};

		format_elements![
			do_token,
			space_token(),
			cons,
			format_elements![
				space_token(),
				while_token,
				space_token(),
				condition,
				semi_colon
			]
		]
	}
}
