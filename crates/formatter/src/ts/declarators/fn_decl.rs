use crate::{concat_elements, space_token, FormatElement, Formatter, ToFormatElement};
use rslint_parser::ast::FnDecl;

impl ToFormatElement for FnDecl {
	fn to_format_element(&self, formatter: &Formatter) -> FormatElement {
		let mut tokens = vec![];

		if let Some(token) = self.async_token() {
			tokens.push(formatter.format_token(&token));
			tokens.push(space_token());
		}

		if let Some(token) = self.function_token() {
			tokens.push(formatter.format_token(&token));
		}

		if let Some(token) = self.star_token() {
			tokens.push(formatter.format_token(&token));
		}
		tokens.push(space_token());

		if let Some(name) = self.name() {
			tokens.push(formatter.format_node(name));
		}

		if let Some(params) = self.parameters() {
			tokens.push(formatter.format_node(params));
		}

		tokens.push(space_token());

		if let Some(body) = self.body() {
			tokens.push(formatter.format_node(body));
		}

		concat_elements(tokens)
	}
}
