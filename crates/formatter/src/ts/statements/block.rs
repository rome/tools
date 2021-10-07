use rslint_parser::ast::BlockStmt;

use crate::{format_tokens, FormatToken, FormatValue};

impl FormatValue for BlockStmt {
	fn format(&self) -> FormatToken {
		let body: Vec<_> = self.stmts().map(|stmt| stmt.format()).collect();

		format_tokens![
			"{",
			FormatToken::Space,
			FormatToken::concat(body),
			FormatToken::Space,
			"}"
		]
	}
}
