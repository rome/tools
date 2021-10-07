use crate::{
	format_elements, group_elements, indent, join_elements, space_token, token, FormatElement,
	ToFormatElement,
};
use rslint_parser::ast::{ArrayExpr, ExprOrSpread};

impl ToFormatElement for ArrayExpr {
	fn to_format_element(&self) -> FormatElement {
		let elements = self.elements();
		let mut tokens = vec![];

		for element in elements {
			match element {
				ExprOrSpread::Expr(expr) => {
					tokens.push(expr.to_format_element());
				}
				ExprOrSpread::Spread(spread) => {
					tokens.push(spread.to_format_element());
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
