use crate::{
	format_elements, group_elements, if_group_breaks, join_elements, soft_indent,
	soft_line_break_or_space, token, FormatElement, ToFormatElement,
};
use rslint_parser::ast::{ArrayExpr, ExprOrSpread};

impl ToFormatElement for ArrayExpr {
	fn to_format_element(&self) -> FormatElement {
		let tokens: Vec<_> = self
			.elements()
			// TODO: use context when introduced or .syntax()
			.map(|element| match element {
				ExprOrSpread::Expr(expr) => expr.to_format_element(),
				ExprOrSpread::Spread(spread) => spread.to_format_element(),
			})
			.collect();

		let separator = format_elements!(token(","), soft_line_break_or_space());
		group_elements(format_elements![
			token("["),
			soft_indent(join_elements(separator, tokens)),
			if_group_breaks(token(",")),
			token("]")
		])
	}
}
