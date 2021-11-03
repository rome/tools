use crate::{FormatElement, FormatError, Formatter, ToFormatElement};
use rslint_parser::ast::NameRef;

impl ToFormatElement for NameRef {
	fn to_format_element(&self, formatter: &Formatter) -> Result<FormatElement, FormatError> {
		formatter.format_token(&self.ident_token()?)
	}
}
