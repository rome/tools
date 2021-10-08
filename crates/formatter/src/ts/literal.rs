use rslint_parser::ast::Literal;

use crate::{token, FormatElement, ToFormatElement};

impl ToFormatElement for Literal {
	fn to_format_element(&self) -> FormatElement {
		let new_string: String = self
			.to_string()
			.as_str()
			.chars()
			.map(|ch| match ch {
				// TODO: this is the final solution and will need to find a clever way to do replacing
				'\'' => '"',
				_ => ch,
			})
			.collect();
		token(new_string.as_str())
	}
}
