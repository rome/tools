use crate::{
	empty_element, format_elements, space_token, FormatElement, FormatError, Formatter,
	ToFormatElement,
};
use rslint_parser::ast::Method;

impl ToFormatElement for Method {
	fn to_format_element(&self, formatter: &Formatter) -> Result<FormatElement, FormatError> {
		let private_token = if let Some(static_token) = self.static_token() {
			format_elements![formatter.format_token(&static_token)?, space_token()]
		} else {
			empty_element()
		};
		let name = formatter.format_node(self.name()?)?;
		let params = formatter.format_node(self.parameters()?)?;
		let body = formatter.format_node(self.body()?)?;
		Ok(format_elements![
			private_token,
			name,
			params,
			space_token(),
			body
		])
	}
}
