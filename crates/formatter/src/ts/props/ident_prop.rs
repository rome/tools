use crate::{FormatElement, Formatter, ToFormatElement};
use rslint_parser::ast::IdentProp;

impl ToFormatElement for IdentProp {
	fn to_format_element(&self, formatter: &Formatter) -> FormatElement {
		formatter.format_node(self.name().unwrap())
	}
}
