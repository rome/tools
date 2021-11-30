use crate::{
	format_elements, space_token, FormatElement, FormatResult, Formatter, ToFormatElement,
};
use rslint_parser::ast::JsBindingWithDefault;

impl ToFormatElement for JsBindingWithDefault {
	fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
		Ok(format_elements![
			formatter.format_node(self.binding()?)?,
			space_token(),
			formatter.format_token(&self.eq_token()?)?,
			space_token(),
			formatter.format_node(self.default()?)?
		])
	}
}
