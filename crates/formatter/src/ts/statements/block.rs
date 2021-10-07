use rslint_parser::ast::BlockStmt;

use crate::{format_elements, space_token, token, FormatElement, ToFormatElement};

impl ToFormatElement for BlockStmt {
	fn to_format_element(&self) -> FormatElement {
		let body: Vec<_> = self.stmts().map(|stmt| stmt.to_format_element()).collect();

		format_elements![
			token("{"),
			space_token(),
			concat_elements(body),
			space_token(),
			token("}")
		]
	}
}
