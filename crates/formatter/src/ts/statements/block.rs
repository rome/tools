use rslint_parser::ast::BlockStmt;

use crate::{concat_elements, format_elements, space_token, token, FormatElement, ToFormatElement};

impl ToFormatElement for BlockStmt {
	fn to_format_element(&self) -> FormatElement {
		let body = concat_elements(self.stmts().map(|stmt| stmt.to_format_element()));
		format_elements![token("{"), space_token(), body, space_token(), token("}")]
	}
}
