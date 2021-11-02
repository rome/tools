use crate::{
	format_elements, group_elements, if_group_breaks, join_elements, soft_indent,
	soft_line_break_or_space, token, FormatElement, Formatter, ToFormatElement,
};
use rslint_parser::ast::ArrayExpr;

impl ToFormatElement for ArrayExpr {
	fn to_format_element(&self, formatter: &Formatter) -> Option<FormatElement> {
		let elements = formatter.format_nodes(self.elements())?;

		Some(group_elements(format_elements!(
			formatter.format_token(&self.l_brack_token()?)?,
			soft_indent(join_elements(
				format_elements!(token(","), soft_line_break_or_space()),
				elements
			)),
			if_group_breaks(token(",")),
			formatter.format_token(&self.r_brack_token()?)?,
		)))
	}
}
