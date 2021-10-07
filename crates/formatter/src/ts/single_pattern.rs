use crate::{concat_elements, FormatElement, ToFormatElement};
use rslint_parser::ast::SinglePattern;

impl ToFormatElement for SinglePattern {
	fn to_format_element(&self) -> FormatElement {
		let mut tokens = vec![];
		if let Some(name) = self.name() {
			tokens.push(name.to_format_element());
		}

		concat_elements(tokens)
	}
}
