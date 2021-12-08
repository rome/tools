use crate::{
	empty_element, format_elements, space_token, FormatElement, FormatResult, Formatter,
	ToFormatElement,
};
use rslint_parser::{
	ast::{JsAnyModifier, JsModifier},
	AstNode,
};

impl ToFormatElement for JsAnyModifier {
	fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
		match self {
			JsAnyModifier::JsModifier(modifier) => modifier.to_format_element(formatter),
			JsAnyModifier::JsUnknownModifier(unknown_modifier) => {
				Ok(formatter.format_raw(unknown_modifier.syntax()))
			}
		}
	}
}

impl ToFormatElement for JsModifier {
	fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
		let static_token = if let Some(static_token) = self.static_token() {
			format_elements![formatter.format_token(&static_token)?, space_token()]
		} else {
			empty_element()
		};

		Ok(format_elements![static_token])
	}
}
