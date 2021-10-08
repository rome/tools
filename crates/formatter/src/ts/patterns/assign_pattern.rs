use crate::{concat_elements, FormatContext, FormatElement, ToFormatElement};
use rslint_parser::ast::AssignPattern;

impl ToFormatElement for AssignPattern {
	fn to_format_element(&self, context: &FormatContext) -> FormatElement {
		let mut tokens = vec![];
		if let Some(key) = self.key() {
			tokens.push(context.format_node(key));
		}

		tokens.push(context.format_node(self.value().expect("No value")));

		concat_elements(tokens)
	}
}
