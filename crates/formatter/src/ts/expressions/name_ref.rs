use rslint_parser::ast::NameRef;

use crate::{syntax_token, FormatContext, ToFormatElement};

impl ToFormatElement for NameRef {
	fn to_format_element(&self, _context: &FormatContext) -> crate::FormatElement {
		syntax_token(
			&self
				.ident_token()
				.expect("This should not fail. If it fails, there's an error."),
		)
	}
}
