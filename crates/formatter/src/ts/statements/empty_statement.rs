use crate::{FormatElement, ToFormatElement};
use rslint_parser::ast::EmptyStmt;

impl ToFormatElement for EmptyStmt {
	fn to_format_element(&self, _formatter: &crate::Formatter) -> FormatElement {
		FormatElement::Empty
	}
}
