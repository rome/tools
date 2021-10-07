use rslint_parser::ast::SequenceExpr;

use crate::{FormatToken, FormatValue};

impl FormatValue for SequenceExpr {
	fn format(&self) -> FormatToken {
		let tokens: Vec<_> = self.exprs().map(|expression| expression.format()).collect();
		FormatToken::concat(tokens)
	}
}
