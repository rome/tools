use rslint_parser::ast::SinglePattern;

use crate::{FormatToken, FormatValue};

impl FormatValue for SinglePattern {
	fn format(&self) -> FormatToken {
		let mut tokens = vec![];
		if let Some(name) = self.name() {
			tokens.push(name.format());
		}

		FormatToken::concat(tokens)
	}
}
