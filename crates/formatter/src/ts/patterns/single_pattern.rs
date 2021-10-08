use crate::{FormatElement, ToFormatElement};
use rslint_parser::ast::SinglePattern;

impl ToFormatElement for SinglePattern {
	fn to_format_element(&self) -> FormatElement {
		// TODO: implementation not finished
		self.name()
			.expect("Name should always exist")
			.to_format_element()
	}
}
