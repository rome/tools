use rslint_parser::ast::Ident;

use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};

impl ToFormatElement for Ident {
	fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
		formatter.format_token(&self.ident_token()?)
	}
}
