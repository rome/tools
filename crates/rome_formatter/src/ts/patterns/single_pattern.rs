use crate::{FormatElement, FormatResult, Formatter, ToFormatElement};
use rslint_parser::ast::SinglePattern;

impl ToFormatElement for SinglePattern {
	fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
		// TODO: implementation not finished
		formatter.format_node(self.name()?)
	}
}
