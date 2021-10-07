use rslint_parser::ast::Literal;

use crate::{token, FormatElement, FormatValue};

impl FormatValue for Literal {
	fn format(&self) -> FormatElement {
		let new_string: String = self
			.to_string()
			.as_str()
			.chars()
			.map(|ch| match ch {
				'\'' => '"',
				_ => ch,
			})
			.collect();
		token(new_string.as_str())
	}
}
