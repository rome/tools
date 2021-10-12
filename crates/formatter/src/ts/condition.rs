use crate::{format_elements, group_elements, soft_indent, FormatElement, ToFormatElement};
use rslint_parser::ast::Condition;

impl ToFormatElement for Condition {
	fn to_format_element(&self, formatter: &crate::Formatter) -> crate::FormatElement {
		let condition = if let Some(condition) = self.condition() {
			soft_indent(formatter.format_node(condition))
		} else {
			FormatElement::Empty
		};

		group_elements(format_elements![
			formatter.format_token(&self.l_paren_token().unwrap()),
			condition,
			formatter.format_token(&self.r_paren_token().unwrap())
		])
	}
}
