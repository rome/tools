use rslint_parser::ast::Literal;

use crate::{FormatElement, Formatter, ToFormatElement};

impl ToFormatElement for Literal {
	fn to_format_element(&self, formatter: &Formatter) -> Option<FormatElement> {
		formatter.format_token(&self.token())
	}
}
