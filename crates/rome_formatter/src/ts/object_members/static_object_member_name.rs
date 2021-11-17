use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_parser::ast::JsStaticObjectMemberName;

impl ToFormatElement for JsStaticObjectMemberName {
	fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
		formatter.format_token(&self.name_token()?)
	}
}
