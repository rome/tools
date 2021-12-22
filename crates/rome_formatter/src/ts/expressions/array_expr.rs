use crate::{
	empty_element, format_elements, group_elements, if_group_breaks, join_elements, soft_indent,
	soft_line_break_or_space, token, FormatElement, FormatResult, Formatter, ToFormatElement,
};
use rslint_parser::ast::{JsArrayExpression, JsArrayHole};
use rslint_parser::AstSeparatedList;

impl ToFormatElement for JsArrayExpression {
	fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
		let elements = self.elements();

		let trailing_comma = if elements.len() == 0 {
			empty_element()
		} else {
			if_group_breaks(token(","))
		};

		Ok(group_elements(format_elements!(
			formatter.format_token(&self.l_brack_token()?)?,
			soft_indent(format_elements![
				join_elements(
					soft_line_break_or_space(),
					formatter.format_separated(elements)?
				),
				trailing_comma,
			]),
			formatter.format_token(&self.r_brack_token()?)?,
		)))
	}
}

impl ToFormatElement for JsArrayHole {
	fn to_format_element(&self, _: &Formatter) -> FormatResult<FormatElement> {
		Ok(empty_element())
	}
}
