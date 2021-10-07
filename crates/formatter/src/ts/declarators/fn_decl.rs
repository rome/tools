use crate::{concat_elements, space_token, ts::format_syntax_token, FormatElement, FormatValue};
use rslint_parser::ast::FnDecl;

impl FormatValue for FnDecl {
	fn format(&self) -> FormatElement {
		let mut tokens = vec![];

		if let Some(token) = self.async_token() {
			tokens.push(format_syntax_token(token));
			tokens.push(space_token());
		}

		if let Some(token) = self.function_token() {
			tokens.push(format_syntax_token(token));
		}

		if let Some(token) = self.star_token() {
			tokens.push(format_syntax_token(token));
		}
		tokens.push(space_token());

		if let Some(name) = self.name() {
			tokens.push(name.format());
		}

		if let Some(params) = self.parameters() {
			tokens.push(params.format());
		}

		tokens.push(space_token());

		if let Some(body) = self.body() {
			tokens.push(body.format());
		}

		concat_elements(tokens)
	}
}
