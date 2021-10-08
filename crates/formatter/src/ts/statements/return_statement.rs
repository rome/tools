use crate::{concat_elements, space_token, token, FormatContext, FormatElement, ToFormatElement};
use rslint_parser::ast::ReturnStmt;

impl ToFormatElement for ReturnStmt {
	fn to_format_element(&self, context: &FormatContext) -> FormatElement {
		let mut tokens = vec![token("return"), space_token()];

		if let Some(value) = self.value() {
			tokens.push(context.format_node(value));
		}

		tokens.push(token(";"));

		concat_elements(tokens)
	}
}
