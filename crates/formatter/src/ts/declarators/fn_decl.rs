use crate::{
	concat_elements, space_token, ts::format_syntax_token, FormatElement, ToFormatElement,
};
use rslint_parser::ast::FnDecl;

impl ToFormatElement for FnDecl {
	fn to_format_element(&self) -> FormatElement {
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
			tokens.push(name.to_format_element());
		}

		if let Some(params) = self.parameters() {
			tokens.push(params.to_format_element());
		}

		tokens.push(space_token());

		if let Some(body) = self.body() {
			tokens.push(body.to_format_element());
		}

		concat_elements(tokens)
	}
}
