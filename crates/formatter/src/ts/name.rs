use crate::{ts::format_syntax_token, FormatElement, ToFormatElement};
use rslint_parser::ast::Name;

impl ToFormatElement for Name {
	fn to_format_element(&self) -> FormatElement {
		format_syntax_token(self.ident_token().unwrap())
	}
}
