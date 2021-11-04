use rslint_parser::ast::Ident;

use crate::{FormatElement, Formatter, ToFormatElement};

impl ToFormatElement for Ident {
	fn to_format_element(&self, formatter: &Formatter) -> Option<FormatElement> {
		formatter.format_token(&self.ident_token()?)
	}
}
