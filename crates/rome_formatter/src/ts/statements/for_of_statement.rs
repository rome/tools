use rslint_parser::ast::ForOfStmt;

use crate::{
	format_elements, group_elements, soft_indent, soft_line_break_or_space, space_token,
	FormatElement, FormatResult, Formatter, ToFormatElement,
};

impl ToFormatElement for ForOfStmt {
	fn to_format_element(&self, formatter: &Formatter) -> FormatResult<FormatElement> {
		let for_token = formatter.format_token(&self.for_token()?)?;
		let l_paren = formatter.format_token(&self.l_paren_token()?)?;
		let left = formatter.format_node(self.left()?)?;
		let of_token = formatter.format_token(&self.of_token()?)?;
		let right = formatter.format_node(self.right()?)?;
		let r_paren = formatter.format_token(&self.r_paren_token()?)?;
		let cons = formatter.format_node(self.cons()?)?;

		Ok(format_elements![
			for_token,
			space_token(),
			l_paren,
			group_elements(soft_indent(format_elements![
				left,
				soft_line_break_or_space(),
				of_token,
				soft_line_break_or_space(),
				right,
			])),
			r_paren,
			space_token(),
			cons
		])
	}
}
