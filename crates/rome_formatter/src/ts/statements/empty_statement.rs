use crate::{empty_element, FormatElement, Formatter, ToFormatElement};
use rslint_parser::ast::EmptyStmt;

impl ToFormatElement for EmptyStmt {
	fn to_format_element(&self, _formatter: &Formatter) -> Result<FormatElement, FormatError> {
		Some(empty_element())
	}
}
