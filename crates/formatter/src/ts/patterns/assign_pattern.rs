use crate::{concat_elements, FormatElement, Formatter, ToFormatElement};
use rslint_parser::ast::AssignPattern;

impl ToFormatElement for AssignPattern {
	fn to_format_element(&self, formatter: &Formatter) -> Option<FormatElement> {
		let mut tokens = vec![];
		if let Some(key) = self.key() {
			tokens.push(formatter.format_node(key)?);
		}

		tokens.push(formatter.format_node(self.value().expect("No value"))?);

		Some(concat_elements(tokens))
	}
}
