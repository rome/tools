use rslint_parser::ast::JsForOfStatement;

use crate::{
	format_elements, group_elements, soft_indent, soft_line_break_or_space, space_token,
	FormatElement, FormatResult, Formatter, ToFormatElement,
};

impl ToFormatElement for JsForOfStatement {
	fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
		let for_token = formatter.format_token(&self.for_token()?)?;
		let l_paren = formatter.format_token(&self.l_paren_token()?)?;
		let initializer = formatter.format_node(self.initializer()?)?;
		let of_token = formatter.format_token(&self.of_token()?)?;
		let expression = formatter.format_node(self.expression()?)?;
		let r_paren = formatter.format_token(&self.r_paren_token()?)?;
		let body = formatter.format_node(self.body()?)?;

		Ok(format_elements![
			for_token,
			space_token(),
			l_paren,
			group_elements(soft_indent(format_elements![
				initializer,
				soft_line_break_or_space(),
				of_token,
				soft_line_break_or_space(),
				expression,
			])),
			r_paren,
			space_token(),
			body
		])
	}
}
