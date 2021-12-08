use crate::{
	empty_element, format_elements, space_token, FormatElement, FormatResult, Formatter,
	ToFormatElement,
};
use rslint_parser::ast::JsMethodClassMember;

impl ToFormatElement for JsMethodClassMember {
	fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
		let modifiers = if let Some(modifiers) = self.modifiers() {
			formatter.format_node(modifiers)?
		} else {
			empty_element()
		};
		let name = formatter.format_node(self.name()?)?;
		let params = formatter.format_node(self.parameter_list()?)?;
		let body = formatter.format_node(self.body()?)?;
		Ok(format_elements![
			modifiers,
			name,
			params,
			space_token(),
			body
		])
	}
}
