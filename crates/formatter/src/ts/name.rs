use crate::{syntax_token, FormatContext, FormatElement, ToFormatElement};
use rslint_parser::ast::Name;

impl ToFormatElement for Name {
	fn to_format_element(&self, _context: &FormatContext) -> FormatElement {
		syntax_token(&self.ident_token().unwrap())
	}
}
