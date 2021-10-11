use crate::{concat_elements, space_token, FormatElement, Formatter, ToFormatElement};
use rslint_parser::ast::Declarator;

impl ToFormatElement for Declarator {
	fn to_format_element(&self, formatter: &Formatter) -> Option<FormatElement> {
		let mut tokens = vec![];

		if let Some(pattern) = self.pattern() {
			tokens.push(formatter.format_node(pattern)?);
		}
		if let Some(equal) = self.eq_token() {
			tokens.push(space_token());
			tokens.push(formatter.format_token(&equal)?);
			tokens.push(space_token());
		}

		if let Some(expression) = self.value() {
			tokens.push(formatter.format_node(expression)?);
		}

		Some(concat_elements(tokens))
	}
}
