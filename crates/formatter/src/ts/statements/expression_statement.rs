use rslint_parser::ast::ExprStmt;

use crate::{FormatToken, FormatValue};

impl FormatValue for ExprStmt {
	fn format(&self) -> FormatToken {
		if let Some(expr) = self.expr() {
			return expr.format();
		}
		FormatToken::Space
	}
}
