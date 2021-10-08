use crate::{
	concat_elements, space_token, syntax_token, FormatContext, FormatElement, ToFormatElement,
};
use rslint_parser::ast::FnDecl;

impl ToFormatElement for FnDecl {
	fn to_format_element(&self, context: &FormatContext) -> FormatElement {
		let mut tokens = vec![];

		if let Some(token) = self.async_token() {
			tokens.push(syntax_token(&token));
			tokens.push(space_token());
		}

		if let Some(token) = self.function_token() {
			tokens.push(syntax_token(&token));
		}

		if let Some(token) = self.star_token() {
			tokens.push(syntax_token(&token));
		}
		tokens.push(space_token());

		if let Some(name) = self.name() {
			tokens.push(context.format_node(name));
		}

		if let Some(params) = self.parameters() {
			tokens.push(context.format_node(params));
		}

		tokens.push(space_token());

		if let Some(body) = self.body() {
			tokens.push(context.format_node(body));
		}

		concat_elements(tokens)
	}
}
