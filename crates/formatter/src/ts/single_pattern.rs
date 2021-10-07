use crate::{concat_elements, FormatElement, ToFormatElement};
use rslint_parser::ast::SinglePattern;

impl ToFormatElement for SinglePattern {
	fn to_format_element(&self) -> FormatElement {
		// TODO: a vector will be needed
		self.name()
			.expect("Name should always exist")
			.to_format_element()
	}
}
