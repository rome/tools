use rslint_parser::ast::Literal;

use crate::{format_tokens, FormatToken, FormatValue};

impl FormatValue for Literal {
	fn format(&self) -> FormatToken {
		let new_string: String = self
			.to_string()
			.as_str()
			.chars()
			.map(|ch| match ch {
				'\'' => '"',
				_ => ch,
			})
			.collect();
		format_tokens!(new_string.as_str())
	}
}
