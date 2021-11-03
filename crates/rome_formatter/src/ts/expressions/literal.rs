use rslint_parser::ast::Literal;

use crate::{FormatElement, FormatError, Formatter, ToFormatElement};

impl ToFormatElement for Literal {
	fn to_format_element(&self, formatter: &Formatter) -> Result<FormatElement, FormatError> {
		formatter.format_token(&self.token())
	}
}
