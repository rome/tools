use rslint_parser::ast::ReturnStmt;

use crate::{format_tokens, FormatToken, FormatValue};

impl FormatValue for ReturnStmt {
	fn format(&self) -> FormatToken {
		let mut tokens = vec![];
		if let Some(return_token) = self.return_token() {
			tokens.push(format_tokens!(return_token.text().as_str()));
		} else {
			tokens.push(format_tokens!("return"));
		}
		tokens.push(FormatToken::Space);
		if let Some(value) = self.value() {
			tokens.push(value.format());
		}
		tokens.push(format_tokens!(";"));

		FormatToken::concat(tokens)
	}
}
