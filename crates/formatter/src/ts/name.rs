use crate::{syntax_token, FormatElement, Formatter, ToFormatElement};
use rslint_parser::ast::Name;

impl ToFormatElement for Name {
	fn to_format_element(&self, _formatter: &Formatter) -> FormatElement {
		syntax_token(&self.ident_token().unwrap())
	}
}
