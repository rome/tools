use crate::{
	empty_element, format_elements, FormatElement, FormatError, Formatter, ToFormatElement,
};
use rslint_parser::ast::AssignPattern;

impl ToFormatElement for AssignPattern {
	fn to_format_element(&self, formatter: &Formatter) -> Result<FormatElement, FormatError> {
		let key = if let Ok(key) = self.key() {
			formatter.format_node(key)?
		} else {
			empty_element()
		};
		let assign = if let Ok(eq_token) = self.eq_token() {
			formatter.format_token(&eq_token)?
		} else if let Some(colon_token) = self.colon_token() {
			formatter.format_token(&colon_token)?
		} else {
			empty_element()
		};

		let value = formatter.format_node(self.value()?)?;
		Ok(format_elements![key, assign, value])
	}
}
