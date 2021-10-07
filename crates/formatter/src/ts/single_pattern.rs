use crate::{concat_elements, FormatToken, FormatValue};
use rslint_parser::ast::SinglePattern;

impl FormatValue for SinglePattern {
	fn format(&self) -> FormatToken {
		let mut tokens = vec![];
		if let Some(name) = self.name() {
			tokens.push(name.format());
		}

		concat_elements(tokens)
	}
}
