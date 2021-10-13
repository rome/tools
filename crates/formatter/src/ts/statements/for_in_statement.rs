use rslint_parser::ast::ForInStmt;

use crate::{
	format_elements, group_elements, soft_indent, soft_line_break_or_space, space_token,
	FormatElement, Formatter, ToFormatElement,
};

impl ToFormatElement for ForInStmt {
	fn to_format_element(&self, formatter: &Formatter) -> FormatElement {
		let for_token = formatter.format_token(&self.for_token().expect("for token is missing"));
		let l_paren =
			formatter.format_token(&self.l_paren_token().expect("left parenthesis is missing"));
		let left = formatter.format_node(self.left().expect("lef expression is missing"));
		let in_token = formatter.format_token(&self.in_token().expect("the in token is missing"));
		let right = formatter.format_node(self.right().expect("right expression is missing"));
		let r_paren =
			formatter.format_token(&self.r_paren_token().expect("right parenthesis is missing"));
		let cons = formatter.format_node(self.cons().expect("consequence is missing"));

		format_elements![
			for_token,
			space_token(),
			l_paren,
			group_elements(soft_indent(format_elements![
				left,
				soft_line_break_or_space(),
				in_token,
				soft_line_break_or_space(),
				right,
			])),
			r_paren,
			space_token(),
			cons
		]
	}
}
