use rslint_parser::ast::BlockStmt;

use crate::{format_tokens, space_token, token, FormatToken, FormatValue};

impl FormatValue for BlockStmt {
	fn format(&self) -> FormatToken {
		let body: Vec<_> = self.stmts().map(|stmt| stmt.format()).collect();

		format_tokens![
			token("{"),
			space_token(),
			concat_elements(body),
			space_token(),
			token("}")
		]
	}
}
