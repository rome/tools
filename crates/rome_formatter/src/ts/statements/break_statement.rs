use crate::{
	empty_element, format_elements, space_token, token, FormatElement, FormatResult, Formatter,
	ToFormatElement,
};
use rslint_parser::ast::JsBreakStatement;

impl ToFormatElement for JsBreakStatement {
	fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
		let label = if let Some(label_token) = self.label_token() {
			format_elements![space_token(), formatter.format_token(&label_token)?]
		} else {
			empty_element()
		};

		Ok(format_elements![
			formatter.format_token(&self.break_token()?)?,
			label,
			token(";")
		])
	}
}
