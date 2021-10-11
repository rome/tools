use rslint_parser::ast::NameRef;

use crate::{Formatter, ToFormatElement};

impl ToFormatElement for NameRef {
	fn to_format_element(&self, formatter: &Formatter) -> crate::FormatElement {
		formatter.format_token(
			&self
				.ident_token()
				.expect("This should not fail. If it fails, there's an error."),
		)
	}
}
