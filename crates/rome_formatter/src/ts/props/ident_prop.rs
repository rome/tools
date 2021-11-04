use crate::{FormatElement, FormatError, Formatter, ToFormatElement};
use rslint_parser::ast::IdentProp;

impl ToFormatElement for IdentProp {
	fn to_format_element(&self, formatter: &Formatter) -> Result<FormatElement, FormatError> {
		formatter.format_node(self.name()?)
	}
}
