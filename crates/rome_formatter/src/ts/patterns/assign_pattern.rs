use crate::{format_elements, FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_parser::ast::AssignPattern;

impl ToFormatElement for AssignPattern {
	fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
		let key = formatter.format_node(self.key()?)?;
		let assign = formatter.format_token(&self.eq_token()?)?;
		let value = formatter.format_node(self.value()?)?;
		Ok(format_elements![key, assign, value])
	}
}
