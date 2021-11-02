use crate::{format_elements, space_token, FormatElement, Formatter, ToFormatElement};
use rslint_parser::ast::Setter;

impl ToFormatElement for Setter {
	fn to_format_element(&self, formatter: &Formatter) -> Option<FormatElement> {
		let token = formatter.format_token(&self.set_token()?)?;
		let name = formatter.format_node(self.key()?)?;
		let params = formatter.format_node(self.parameters()?)?;
		let body = formatter.format_node(self.body()?)?;
		Some(format_elements![
			token,
			space_token(),
			name,
			params,
			space_token(),
			body
		])
	}
}
