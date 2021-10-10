use crate::{concat_elements, space_token, token, FormatElement, Formatter, ToFormatElement};
use rslint_parser::ast::ReturnStmt;

impl ToFormatElement for ReturnStmt {
	fn to_format_element(&self, formatter: &Formatter) -> FormatElement {
		let mut tokens = vec![token("return"), space_token()];

		if let Some(value) = self.value() {
			tokens.push(formatter.format_node(value));
		}

		tokens.push(token(";"));

		concat_elements(tokens)
	}
}
