use crate::{format_elements, space_token, token, FormatElement, Formatter, ToFormatElement};
use rslint_parser::ast::WhileStmt;

impl ToFormatElement for WhileStmt {
	fn to_format_element(&self, formatter: &Formatter) -> FormatElement {
		let while_token = formatter.format_token(&self.while_token().expect("While token missing"));
		let condition = formatter.format_node(self.condition().expect("Condition missing"));

		let cons = if let Some(cons) = self.cons() {
			formatter.format_node(cons)
		} else {
			token("{}")
		};
		format_elements![while_token, space_token(), condition, space_token(), cons]
	}
}
