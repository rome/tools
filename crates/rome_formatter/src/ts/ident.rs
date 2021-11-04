use rslint_parser::ast::Ident;

use crate::{FormatElement, FormatError, Formatter, ToFormatElement};

impl ToFormatElement for Ident {
	fn to_format_element(&self, formatter: &Formatter) -> Result<FormatElement, FormatError> {
		formatter.format_token(&self.ident_token()?)
	}
}
