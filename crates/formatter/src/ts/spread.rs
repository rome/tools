use crate::{format_elements, token, FormatElement, Formatter, ToFormatElement};
use rslint_parser::ast::SpreadElement;

impl ToFormatElement for SpreadElement {
	fn to_format_element(&self, formatter: &Formatter) -> FormatElement {
		format_elements!(token("..."), formatter.format_node(self.element().unwrap()))
	}
}
