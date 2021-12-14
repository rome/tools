use rslint_parser::ast::JsMethodObjectMember;

use crate::{
	empty_element, format_elements, space_token, FormatElement, FormatResult, Formatter,
	ToFormatElement,
};

impl ToFormatElement for JsMethodObjectMember {
	fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
		let async_token = if let Some(token) = self.async_token() {
			format_elements![formatter.format_token(&token)?, space_token()]
		} else {
			empty_element()
		};
		let star_token = if let Some(token) = self.star_token() {
			formatter.format_token(&token)?
		} else {
			empty_element()
		};
		Ok(format_elements![
			async_token,
			star_token,
			formatter.format_node(self.name()?)?,
			// TODO self.type_params()
			formatter.format_node(self.parameters()?)?,
			// TODO self.return_type()
			space_token(),
			formatter.format_node(self.body()?)?,
		])
	}
}
