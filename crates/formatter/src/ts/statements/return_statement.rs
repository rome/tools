use crate::{
	concat_elements, space_token, token, ts::format_syntax_token, FormatElement, FormatValue,
};
use rslint_parser::ast::ReturnStmt;

impl FormatValue for ReturnStmt {
	fn format(&self) -> FormatElement {
		let mut tokens = vec![];
		if let Some(return_token) = self.return_token() {
			tokens.push(format_syntax_token(return_token));
		} else {
			tokens.push(token("return"));
		}
		tokens.push(space_token());
		if let Some(value) = self.value() {
			tokens.push(value.format());
		}
		tokens.push(token(";"));

		concat_elements(tokens)
	}
}
