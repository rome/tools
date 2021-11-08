use crate::{empty_element, FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_parser::ast::EmptyStmt;

impl ToFormatElement for EmptyStmt {
	fn to_format_element(&self, _formatter: &Formatter) -> FormatResult<FormatElement> {
		Ok(empty_element())
	}
}
