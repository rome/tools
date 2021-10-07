use rslint_parser::ast::NameRef;

use crate::{ts::format_syntax_token, ToFormatElement};

impl ToFormatElement for NameRef {
	fn to_format_element(&self) -> crate::FormatElement {
		format_syntax_token(
			self.ident_token()
				.expect("This should not fail. If it fails, there's an error."),
		)
	}
}
