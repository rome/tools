use crate::{token, FormatElement, FormatError, Formatter, ToFormatElement};
use rslint_parser::ast::String as JsString;

impl ToFormatElement for JsString {
	fn to_format_element(&self, _formatter: &Formatter) -> Result<FormatElement, FormatError> {
		let mut content = self.to_string();

		// uses single quotes
		if content.starts_with('\'') {
			content.replace_range(0..1, "\"");
			content.replace_range(content.len() - 1..content.len(), "\"");
		}

		Ok(token(content.as_str()))
	}
}
