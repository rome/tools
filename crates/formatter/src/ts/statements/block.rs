use rslint_parser::ast::BlockStmt;

use crate::{format_elements, space_token, token, FormatElement, FormatValue};

impl FormatValue for BlockStmt {
	fn format(&self) -> FormatElement {
		let body: Vec<_> = self.stmts().map(|stmt| stmt.format()).collect();

		format_elements![
			token("{"),
			space_token(),
			concat_elements(body),
			space_token(),
			token("}")
		]
	}
}
