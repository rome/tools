use crate::{format_tokens, FormatToken, FormatValue, GroupToken, ListToken};
use rslint_parser::ast::{ArrayExpr, ExprOrSpread};

impl FormatValue for ArrayExpr {
	fn format(&self) -> FormatToken {
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
		let separator = format_tokens!(",", FormatToken::Space);
		format_tokens!(
			"[",
			FormatToken::indent(GroupToken::new(ListToken::join(separator, tokens))),
			",",
			"]",
		)
	}
}
