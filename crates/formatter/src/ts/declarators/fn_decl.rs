use rslint_parser::ast::FnDecl;

use crate::{format_tokens, FormatToken, FormatValue};

impl FormatValue for FnDecl {
	fn format(&self) -> FormatToken {
		let mut tokens = vec![];

		if let Some(token) = self.async_token() {
			tokens.push(format_tokens!(token.text().as_str()));
			tokens.push(FormatToken::Space);
		}

		if let Some(token) = self.function_token() {
			tokens.push(format_tokens!(token.text().as_str()));
		}

		if let Some(token) = self.star_token() {
			tokens.push(format_tokens!(token.text().as_str()));
		}
		tokens.push(FormatToken::Space);

		if let Some(name) = self.name() {
			tokens.push(name.format());
		}

		if let Some(params) = self.parameters() {
			tokens.push(params.format());
		}

		tokens.push(FormatToken::Space);

		if let Some(body) = self.body() {
			tokens.push(body.format());
		}

		FormatToken::concat(tokens)
	}
}
