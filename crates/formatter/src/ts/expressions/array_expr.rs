use crate::{
	format_elements, group_elements, if_group_breaks, join_elements, soft_indent,
	soft_line_break_or_space, token, FormatContext, FormatElement, ToFormatElement,
};
use rslint_parser::ast::ArrayExpr;

impl ToFormatElement for ArrayExpr {
	fn to_format_element(&self, context: &FormatContext) -> FormatElement {
		let elements = self.elements().map(|element| context.format_node(element));

		let separator = format_elements!(token(","), soft_line_break_or_space());
		group_elements(format_elements!(
			token("["),
			soft_indent(join_elements(separator, elements)),
			if_group_breaks(token(",")),
			token("]"),
		))
	}
}
