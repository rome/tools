use crate::{
	format_elements, group_elements, if_group_breaks, join_elements, soft_indent,
	soft_line_break_or_space, token, FormatElement, Formatter, ToFormatElement,
};
use rslint_parser::ast::ArrayExpr;

impl ToFormatElement for ArrayExpr {
	fn to_format_element(&self, formatter: &Formatter) -> FormatElement {
		let elements = self
			.elements()
			.map(|element| formatter.format_node(element));

		let separator = format_elements!(token(","), soft_line_break_or_space());
		group_elements(format_elements!(
			token("["),
			soft_indent(join_elements(separator, elements)),
			if_group_breaks(token(",")),
			token("]"),
		))
	}
}
