use crate::{
	empty_element, format_elements, space_token, token, FormatElement, FormatError, FormatResult,
	Formatter, ToFormatElement,
};
use rslint_parser::ast::ClassProp;

impl ToFormatElement for ClassProp {
	fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
		let static_token = if let Some(static_token) = self.static_token() {
			format_elements![formatter.format_token(&static_token)?, space_token()]
		} else {
			empty_element()
		};
		let value = self.value();
		let equal = self.eq_token();

		let equal_and_value = match (value, equal) {
			(None, None) => Some(empty_element()),
			(Some(value), Some(equal)) => Some(format_elements![
				space_token(),
				formatter.format_token(&equal)?,
				space_token(),
				formatter.format_node(value)?,
			]),
			_ => None,
		};

		match equal_and_value {
			Some(equal_and_value) => {
				let key = formatter.format_node(self.key()?)?;

				Ok(format_elements![
					static_token,
					key,
					equal_and_value,
					token(";")
				])
			}
			// TODO: review, we can have a better error
			None => Err(FormatError::UnknownNode),
		}
	}
}
