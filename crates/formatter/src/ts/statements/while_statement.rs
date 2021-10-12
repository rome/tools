use rslint_parser::ast::WhileStmt;

use crate::{format_elements, space_token, token, FormatElement, ToFormatElement};

impl ToFormatElement for WhileStmt {
	fn to_format_element(&self, formatter: &crate::Formatter) -> FormatElement {
		let while_token = self.while_token().expect("While token missing");
		let condition = formatter.format_node(self.condition().expect("Condition missing"));

		let cons = if let Some(cons) = self.cons() {
			formatter.format_node(cons)
		} else {
			format_elements![token("{"), token("}"),]
		};
		format_elements![
			formatter.format_token(&while_token),
			space_token(),
			condition,
			space_token(),
			cons
		]
	}
}
