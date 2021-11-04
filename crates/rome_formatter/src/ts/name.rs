use crate::{FormatElement, FormatError, Formatter, ToFormatElement};
use rslint_parser::ast::Name;

impl ToFormatElement for Name {
	fn to_format_element(&self, formatter: &Formatter) -> Result<FormatElement, FormatError> {
		formatter.format_token(&self.ident_token()?)
	}
}
