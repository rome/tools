use crate::{concat_elements, FormatElement, ToFormatElement};
use rslint_parser::ast::AssignPattern;

impl ToFormatElement for AssignPattern {
	fn to_format_element(&self) -> FormatElement {
		let mut tokens = vec![];
		if let Some(key) = self.key() {
			tokens.push(key.to_format_element());
		}

		tokens.push(self.value().expect("No value").to_format_element());

		concat_elements(tokens)
	}
}
