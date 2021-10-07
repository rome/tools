use crate::{
	format_elements, group_elements, indent, join_elements, space_token, token, FormatElement,
	FormatValue,
};
use rslint_parser::ast::{ArrayExpr, ExprOrSpread};

impl FormatValue for ArrayExpr {
	fn format(&self) -> FormatElement {
		let elements = self.elements();
		let mut tokens = vec![];

		for element in elements {
			match element {
				ExprOrSpread::Expr(expr) => {
					tokens.push(expr.format());
				}
				ExprOrSpread::Spread(spread) => {
					tokens.push(spread.format());
				}
			}
		}
		let separator = format_elements!(token(","), space_token());
		format_elements!(
			token("["),
			indent(group_elements(join_elements(separator, tokens))),
			token(","),
			token("]"),
		)
	}
}
