use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_parser::ast::JsSemicolonClassMember;

impl ToFormatElement for JsSemicolonClassMember {
	fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
		formatter.format_token(&self.semicolon_token()?)
	}
}
