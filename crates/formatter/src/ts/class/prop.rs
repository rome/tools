use crate::{
	empty_element, format_elements, space_token, token, FormatElement, Formatter, ToFormatElement,
};
use rslint_parser::ast::ClassProp;

impl ToFormatElement for ClassProp {
	fn to_format_element(&self, formatter: &Formatter) -> Option<FormatElement> {
		let static_token = if let Some(static_token) = self.static_token() {
			format_elements![formatter.format_token(&static_token)?, space_token()]
		} else {
			empty_element()
		};
		let value = if let Some(value) = self.value() {
			let value = formatter.format_node(value)?;

			format_elements![space_token(), token("="), space_token(), value]
		} else {
			empty_element()
		};

		let key = formatter.format_node(self.key()?)?;

		Some(format_elements![static_token, key, value, token(";")])
	}
}
