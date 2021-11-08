use crate::{format_elements, space_token, FormatElement, Formatter, ToFormatElement};
use rslint_parser::ast::LiteralProp;

impl ToFormatElement for LiteralProp {
	fn to_format_element(&self, formatter: &Formatter) -> Result<FormatElement, FormatError> {
		let key = formatter.format_node(self.key()?)?;
		let colon = formatter.format_token(&self.colon_token()?)?;
		let value = formatter.format_node(self.value()?)?;
		Some(format_elements![key, colon, space_token(), value])
	}
}
