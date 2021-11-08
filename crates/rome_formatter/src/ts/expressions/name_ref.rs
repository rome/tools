use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_parser::ast::NameRef;

impl ToFormatElement for NameRef {
	fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
		formatter.format_token(&self.ident_token()?)
	}
}
