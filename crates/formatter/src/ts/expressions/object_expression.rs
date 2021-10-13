use crate::{
	format_elements, group_elements, if_group_breaks, join_elements, soft_indent,
	soft_line_break_or_space, token, FormatElement, Formatter, ToFormatElement,
};
use rslint_parser::ast::ObjectExpr;

impl ToFormatElement for ObjectExpr {
	fn to_format_element(&self, formatter: &Formatter) -> FormatElement {
		let separator = format_elements!(token(","), soft_line_break_or_space());
		let props = self.props().map(|prop| formatter.format_node(prop));

		group_elements(format_elements!(
			token("{"),
			soft_indent(join_elements(separator, props)),
			if_group_breaks(token(",")),
			token("}"),
		))
	}
}
