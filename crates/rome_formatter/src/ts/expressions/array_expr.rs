use crate::{
	empty_element, format_elements, group_elements, if_group_breaks, join_elements, soft_indent,
	soft_line_break_or_space, token, FormatElement, FormatResult, Formatter, ToFormatElement,
};
use rslint_parser::ast::{JsArrayExpression, JsArrayHole};

impl ToFormatElement for JsArrayExpression {
	fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
		let elements = formatter.format_nodes(self.elements())?;

		Ok(group_elements(format_elements!(
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

impl ToFormatElement for JsArrayHole {
	fn to_format_element(&self, _: &Formatter) -> FormatResult<FormatElement> {
		Ok(empty_element())
	}
}
