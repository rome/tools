use rslint_parser::ast::Pattern;

use crate::{space_token, ToFormatElement};

impl ToFormatElement for Pattern {
	fn to_format_element(&self) -> crate::FormatElement {
		space_token()
	}
}
