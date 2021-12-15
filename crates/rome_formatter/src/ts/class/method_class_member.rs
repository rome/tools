use crate::{
	empty_element, format_elements, space_token, FormatElement, FormatResult, Formatter,
	ToFormatElement,
};
use rslint_parser::ast::JsMethodClassMember;

impl ToFormatElement for JsMethodClassMember {
	fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
		let static_token = if let Some(token) = self.static_token() {
			format_elements![formatter.format_token(&token)?, space_token()]
		} else {
			empty_element()
		};
		let name = formatter.format_node(self.name()?)?;
		let params = formatter.format_node(self.parameter_list()?)?;
		let body = formatter.format_node(self.body()?)?;
		Ok(format_elements![
			static_token,
			name,
			params,
			space_token(),
			body
		])
	}
}
