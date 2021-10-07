use rslint_parser::ast::SequenceExpr;

use crate::{concat_elements, FormatElement, FormatValue};

impl FormatValue for SequenceExpr {
	fn format(&self) -> FormatElement {
		let tokens: Vec<_> = self.exprs().map(|expression| expression.format()).collect();
		concat_elements(tokens)
	}
}
