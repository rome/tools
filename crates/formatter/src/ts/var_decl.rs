use rslint_parser::ast::VarDecl;

use crate::{format_tokens, FormatToken, FormatValue};

impl FormatValue for VarDecl {
	fn format(&self) -> FormatToken {
		let mut tokens = vec![];

		if let Some(token) = self.const_token() {
			tokens.push(format_tokens!(token.text().as_str()));
		} else if let Some(token) = self.let_token() {
			tokens.push(format_tokens!(token.text().as_str()));
		} else if let Some(token) = self.var_token() {
			tokens.push(format_tokens!(token.text().as_str()));
		} else {
			// TODO: Diagnostic?
			tokens.push(format_tokens!("var"));
		}
		tokens.push(format_tokens!(FormatToken::Space));

		for declarator in self.declared() {
			tokens.push(declarator.format());
		}

		FormatToken::concat(tokens)
	}
}
