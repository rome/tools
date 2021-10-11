use crate::{space_token, ToFormatElement};
use rslint_parser::ast::EmptyStmt;

impl ToFormatElement for EmptyStmt {
	fn to_format_element(&self, _formatter: &crate::Formatter) -> crate::FormatElement {
		space_token()
	}
}
