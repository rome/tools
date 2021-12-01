use crate::{format_elements, FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_parser::ast::JsSpread;

impl ToFormatElement for JsSpread {
	fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
		Ok(format_elements![
			formatter.format_token(&self.dotdotdot_token()?)?,
			formatter.format_node(self.argument()?)?
		])
	}
}
