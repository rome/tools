use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_parser::ast::IdentProp;

impl ToFormatElement for IdentProp {
	fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
		formatter.format_node(self.name()?)
	}
}
