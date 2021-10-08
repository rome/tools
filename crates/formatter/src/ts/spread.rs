use crate::{format_elements, token, FormatContext, FormatElement, ToFormatElement};
use rslint_parser::ast::SpreadElement;

impl ToFormatElement for SpreadElement {
	fn to_format_element(&self, context: &FormatContext) -> FormatElement {
		format_elements!(token("..."), context.format_node(self.element().unwrap()))
	}
}
