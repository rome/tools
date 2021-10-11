use rslint_parser::ast::Literal;

use crate::{FormatElement, Formatter, ToFormatElement};

impl ToFormatElement for Literal {
	fn to_format_element(&self, formatter: &Formatter) -> FormatElement {
		formatter.format_token(&self.token())
	}
}
