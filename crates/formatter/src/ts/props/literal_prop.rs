use crate::{format_elements, space_token, FormatElement, Formatter, ToFormatElement};
use rslint_parser::ast::LiteralProp;

impl ToFormatElement for LiteralProp {
	fn to_format_element(&self, formatter: &Formatter) -> Option<FormatElement> {
		let key = formatter.format_node(self.key()?)?;
		let value = formatter.format_node(self.value()?)?;
		let colon = formatter.format_token(&self.colon_token()?)?;
		Some(format_elements![key, colon, space_token(), value])
	}
}
