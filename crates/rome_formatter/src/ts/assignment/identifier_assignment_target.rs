use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_parser::ast::JsIdentifierAssignment;

impl ToFormatElement for JsIdentifierAssignment {
	fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
		formatter.format_token(&self.name_token()?)
	}
}
