use crate::{concat_elements, space_token, token, FormatElement, Formatter, ToFormatElement};
use rslint_parser::ast::ReturnStmt;

impl ToFormatElement for ReturnStmt {
	fn to_format_element(&self, formatter: &Formatter) -> Result<FormatElement, FormatError> {
		let mut tokens = vec![token("return")];

		if let Some(value) = self.value() {
			tokens.push(space_token());
			tokens.push(formatter.format_node(value)?);
		}

		tokens.push(token(";"));

		Some(concat_elements(tokens))
	}
}
