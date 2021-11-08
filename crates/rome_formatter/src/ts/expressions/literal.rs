use rslint_parser::ast::Literal;

use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};

impl ToFormatElement for Literal {
	fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
		formatter.format_token(&self.token())
	}
}
