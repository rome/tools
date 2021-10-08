use crate::{FormatContext, FormatElement, ToFormatElement};
use rslint_parser::ast::SinglePattern;

impl ToFormatElement for SinglePattern {
	fn to_format_element(&self, context: &FormatContext) -> FormatElement {
		// TODO: implementation not finished
		context.format_node(self.name().expect("Name should always exist"))
	}
}
