use crate::{format_elements, FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_parser::ast::JsPrivateClassMemberName;

impl ToFormatElement for JsPrivateClassMemberName {
	fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
		Ok(format_elements![
			formatter.format_token(&self.hash_token()?)?,
			formatter.format_token(&self.id_token()?)?,
		])
	}
}
