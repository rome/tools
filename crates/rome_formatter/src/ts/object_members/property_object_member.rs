use crate::{
	format_elements, space_token, FormatElement, FormatResult, Formatter, ToFormatElement,
};
use rslint_parser::ast::JsPropertyObjectMember;

impl ToFormatElement for JsPropertyObjectMember {
	fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
		let key = formatter.format_node(self.key()?)?;
		let colon = formatter.format_token(&self.colon_token()?)?;
		let value = formatter.format_node(self.value()?)?;
		Ok(format_elements![key, colon, space_token(), value])
	}
}
